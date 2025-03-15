#![deny(clippy::unwrap_used)]
#![forbid(unsafe_code)]
//#![deny(missing_docs)]

//! # Istock Rs Crate
//!
//! `istock_rs` is something, add doc here

use itertools::Itertools;
use std::fmt::Display;
use std::ops::Deref;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct LabelType {
    name: String,
    description: String,
    parent: Option<Box<LabelType>>,
    sort: i32,
}

impl Display for LabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug)]
pub struct Label {
    pub name: String,
    pub description: String,
    pub ty: LabelType,
}

#[derive(Debug)]
pub struct Labels {
    pub items: Vec<Label>,
}

impl Deref for Labels {
    type Target = [Label];
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl Default for Labels {
    fn default() -> Self {
        let plate = LabelType { name: "Plate".to_string(), description: "板块".to_string(), parent: None, sort: 0 };
        let plates = labels(vec!["大模型", "算力", "通信", "半导体", "光刻机", "先进封装", "消费电子"], plate);

        let concept =
            LabelType { name: "Concept".to_string(), description: "概念".to_string(), parent: None, sort: 1 };
        let concepts = labels(vec!["华为鸿蒙", "华为智驾", "小米", "阿里", "AI眼镜", "AI医疗", "AI制药"], concept);

        Self { items: concepts.into_iter().chain(plates).collect() }
    }
}

impl Labels {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn print_to_console(&self) {
        let indent: String = " ".repeat(4);

        let groups = self.items.iter().chunk_by(|it| &it.ty);
        let groups = groups.into_iter().sorted_by_key(|(ty, _)| ty.sort);
        for (index, (ty, items)) in groups.into_iter().enumerate() {
            let ty_index = format!("{:0>2}", index + 1);
            println!("{ty_index} {}({})", ty.name, ty.description);
            for (index, item) in items.enumerate() {
                let label_index = format!("{:0>3} ", index + 1);
                println!("{indent}{label_index}{}({})", item.name, item.description);
            }
            println!();
        }
    }
}

fn labels<S: Into<String>>(names: Vec<S>, ty: LabelType) -> Vec<Label> {
    names.into_iter().map(|name| Label { name: name.into(), description: "".to_string(), ty: ty.clone() }).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_greet() {
        assert_eq!(1, 1);
    }
}
