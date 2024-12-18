#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! crate filelinec.
//!
//! add doc here

use std::error::Error;
use std::path::Path;
use std::{fs, io, io::BufRead};

/// list and count file lines
//TODO: use glob
pub fn list_and_count_file_lines<P: AsRef<Path>>(
    path: P, ext: Option<String>, sort: bool,
) -> Result<(), Box<dyn Error>> {
    let mut ret: Vec<(String, usize)> = fs::read_dir(path)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.file_type().expect("file type").is_file()
                && (ext.is_none() || entry.path().extension().unwrap_or_default().to_str() == ext.as_deref())
        })
        .map(|entry| {
            let path = entry.path();
            let file_name = path.file_name().expect("file name").to_str().expect("to_str");
            let num = count_lines(&path).expect("count_lines");
            (file_name.to_string(), num)
        })
        .collect();

    if sort {
        ret.sort_by(|a, b| a.1.cmp(&b.1));
    }
    let mut sum = 0;
    for (file_name, num) in ret {
        sum += num;
        println!("{:<30}: {}", file_name, num);
    }
    println!("{}", "-".repeat(36));
    println!("{:<30}: {}", "sum", sum);

    Ok(())
}

fn count_lines<P: AsRef<Path>>(file_path: P) -> io::Result<usize> {
    let file = fs::File::open(file_path)?;
    let buf_reader = io::BufReader::new(file);
    Ok(buf_reader.lines().count())
}
