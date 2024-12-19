//! domain

use std::path::Path;
use std::{fs, io, io::BufRead};

/// CountItem
#[derive(Debug)]
pub struct CountItem {
    /// file name
    pub file_name: String,
    /// file lines
    pub num: usize,
}

/// FileLineCount
#[derive(Debug)]
pub struct FileLineCount {
    /// count items
    pub items: Vec<CountItem>,
}

/// FromDirOptions
#[derive(Debug)]
pub struct FromDirOptions {
    ///ext
    pub ext: Option<String>,
    ///sort
    pub sort: bool,
}

impl FromDirOptions {
    /// new
    pub fn new(ext: Option<String>, sort: bool) -> Self {
        Self { ext, sort }
    }
}

impl FileLineCount {
    /// from dir
    pub fn from_dir<P: AsRef<Path>>(path: P, options: FromDirOptions) -> Result<FileLineCount, io::Error> {
        let mut items: Vec<CountItem> = fs::read_dir(path)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.file_type().expect("file type").is_file()
                    && (options.ext.is_none()
                        || entry.path().extension().unwrap_or_default().to_str() == options.ext.as_deref())
            })
            .map(|entry| {
                let path = entry.path();
                let file_name = path.file_name().expect("file name").to_str().expect("to_str");
                let num = Self::count_lines(&path).expect("count_lines");
                CountItem { file_name: file_name.to_string(), num }
            })
            .collect();

        if options.sort {
            items.sort_by(|a, b| a.num.cmp(&b.num));
        }

        Ok(Self { items })
    }

    /// list and count file lines`
    pub fn pretty_print(&self) {
        let mut sum = 0;
        for item in &self.items {
            sum += item.num;
            println!("{:<30}: {}", item.file_name, item.num);
        }
        println!("{}", "-".repeat(36));
        println!("{:<30}: {}", "sum", sum);
    }

    fn count_lines<P: AsRef<Path>>(file_path: P) -> io::Result<usize> {
        let file = fs::File::open(file_path)?;
        let buf_reader = io::BufReader::new(file);
        Ok(buf_reader.lines().count())
    }
}
