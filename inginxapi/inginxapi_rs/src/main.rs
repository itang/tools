#![forbid(unsafe_code)]

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "access.log-20240304")]
    file: PathBuf,
    #[arg(long, default_value = "/getClient")]
    api_name: String,
}

#[derive(Debug)]
struct Item {
    request_time: f64,
    response_time: f64,
}

#[derive(Debug)]
struct Kv {
    name: String,
    value: f64,
}

#[derive(Debug)]
struct SubResult {
    name: String,
    avg: f64,
    max: f64,
    min: f64,
    ps: Vec<Kv>,
}

struct StatResult {
    count: u64,
    items: Vec<SubResult>,
}

fn sub_result<T: Fn(&Item) -> f64>(name: String, value_fn: T, items: &[Item]) -> SubResult {
    let pv_list: Vec<f64> = vec![0.995, 0.99, 0.98, 0.97, 0.96, 0.95, 0.90, 0.75, 0.50];
    let values: Vec<f64> = items.iter().map(value_fn).collect();
    SubResult {
        name,
        avg: values.iter().sum::<f64>() / items.len() as f64,
        max: values.iter().max_by(|a, b| a.partial_cmp(b).expect("Tried to compare a NaN")).unwrap().to_owned(),
        min: values.iter().min_by(|a, b| a.partial_cmp(b).expect("Tried to compare a NaN")).unwrap().to_owned(),
        ps: pv_list
            .iter()
            .map(|pv| Kv { name: format!("p{}", (pv * 1000.0) / 10.0), value: p(&values, *pv) })
            .collect(),
    }
}

fn get_result(content: String, api_name: String) -> StatResult {
    let lines: Vec<&str> = content.lines().filter(|line| line.contains(&api_name)).collect();
    for line in lines.iter() {
        println!("{}", line)
    }

    let items: Vec<Item> = lines
        .iter()
        .map(|line| {
            let array: Vec<&str> = line.split(' ').collect();
            let req_t = {
                let it = array[0];
                //println!("\nxxxx:{:?}", &it[1..(it.len() - 2)]);
                it[1..=(it.len() - 2)].parse().expect("")
            };
            let resp_t = {
                let it = array[1];
                if it == "-" {
                    "0"
                } else {
                    &it[1..(it.len() - 2)]
                }
            }
            .parse()
            .expect("");
            Item { request_time: req_t, response_time: resp_t }
        })
        .collect();

    StatResult {
        count: items.len() as u64,
        items: vec![
            sub_result("request time".to_string(), |item| item.request_time, &items),
            sub_result("response time".to_string(), |item| item.response_time, &items),
        ],
    }
}

fn p(list: &[f64], value: f64) -> f64 {
    let mut list_sorted = list.to_owned();
    list_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let index = {
        let it = (list.len() as f64 * value) as isize - 1;
        if it < 0 {
            0
        } else if it as usize >= list.len() {
            list.len()
        } else {
            it as usize
        }
    };

    list_sorted[index]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("file: {:?}\napiname:{}", args.file, args.api_name);
    println!("*****************");

    let content = std::fs::read_to_string(args.file)?;

    let result = get_result(content, args.api_name);

    println!("count: {}", result.count);

    for item in result.items {
        println!("{}", item.name);
        println!("\tavg  : {:.3}", item.avg);
        println!("\tmax  : {}", item.max);
        for kv in item.ps {
            println!("\t{:-5}: {}", kv.name, kv.value);
        }
        println!("\tmin  : {}", item.min);
        println!();
    }

    Ok(())
}
