use crate::types::{IndicesResult, IndicesValues, Pair, StatResult};
use std::error::Error;
use std::path::PathBuf;

//#[derive(Default)]
pub struct StatApiOptions {
    pub debug: bool,
    pub pv_list: Vec<f64>,
}

impl Default for StatApiOptions {
    fn default() -> Self {
        Self { debug: false, pv_list: vec![0.995, 0.99, 0.98, 0.97, 0.96, 0.95, 0.90, 0.75, 0.50] }
    }
}
// impl StatApiOptions {
//     pub fn new_with_debug() -> Self {
//         Self { debug: true, ..Self::default() }
//     }
// }

pub fn stat_api(
    access_log_file: PathBuf, api_name: String, options: StatApiOptions,
) -> Result<StatResult, Box<dyn Error>> {
    let content = std::fs::read_to_string(access_log_file)?;
    let lines: Vec<&str> = content.lines().filter(|line| line.contains(&api_name)).collect();

    if options.debug {
        for line in &lines {
            println!("{}", line)
        }
    }

    let items: Vec<IndicesValues> = lines
        .iter()
        .map(|line| {
            let array: Vec<&str> = line.split(' ').collect();
            let req_t = {
                let it = array[0];
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
            IndicesValues { request_time: req_t, response_time: resp_t }
        })
        .collect();

    Ok(StatResult {
        count: items.len() as u64,
        indices_items: vec![
            indices_result("request time".to_string(), |item| item.request_time, &items, &options.pv_list)?,
            indices_result("response time".to_string(), |item| item.response_time, &items, &options.pv_list)?,
        ],
    })
}

pub fn display_for_cli(result: StatResult) {
    println!("count: {}", result.count);
    for item in result.indices_items {
        println!("{}", item.name);
        println!("\t{:-5}: {:.3}", "avg", item.avg);
        println!("\t{:-5}: {:.3}", "max", item.max);
        for kv in item.percent_list {
            println!("\t{:-5}: {:.3}", kv.name, kv.value);
        }
        println!("\t{:-5}: {:.3}", "min", item.min);
        println!();
    }
}

fn indices_result<T: Fn(&IndicesValues) -> f64>(
    name: String, value_fn: T, items: &[IndicesValues], pv_list: &[f64],
) -> Result<IndicesResult, Box<dyn Error>> {
    //let pv_list: Vec<f64> = vec![0.995, 0.99, 0.98, 0.97, 0.96, 0.95, 0.90, 0.75, 0.50];
    let values: Vec<f64> = items.iter().map(value_fn).collect();
    Ok(IndicesResult {
        name,
        avg: values.iter().sum::<f64>() / items.len() as f64,
        max: values.iter().max_by(|a, b| a.partial_cmp(b).expect("Tried to compare a NaN")).expect("max").to_owned(),
        min: values.iter().min_by(|a, b| a.partial_cmp(b).expect("Tried to compare a NaN")).expect("min").to_owned(),
        percent_list: pv_list
            .iter()
            .map(|pv| Pair { name: format!("p{}", (pv * 1000.0) / 10.0), value: percent(&values, *pv) })
            .collect(),
    })
}

fn percent(list: &[f64], value: f64) -> f64 {
    let mut list_sorted = list.to_owned();
    list_sorted.sort_by(|a, b| a.partial_cmp(b).expect("sort by"));

    let index = (list.len() as f64 * value) as usize;
    let index = if index == 0 {
        0
    } else if index >= list.len() {
        list.len() - 1
    } else {
        index
    };

    list_sorted[index]
}
