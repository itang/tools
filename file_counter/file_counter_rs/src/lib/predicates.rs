use std::path::Path;

use crate::PredicatePathFn;
use glob_match::glob_match;
use path_slash::PathExt;

/// build predicate ext fn
pub fn build_predicate_ext_fn(ext_names: Vec<String>) -> impl Fn(&Path) -> bool {
    move |p| {
        ext_names
            .iter()
            .any(|ext_name| p.extension().is_some_and(|ext| ext.to_str().expect("to_str") == trim(ext_name)))
    }
}

/// build predicate contains fn
pub fn build_predicate_contains_fn(contains: Vec<String>) -> impl Fn(&Path) -> bool {
    move |p| {
        if let Ok(content) = std::fs::read_to_string(p) {
            contains.iter().all(|c| content.contains(c))
        } else {
            false
        }
    }
}

/// build predicate not contains fn
pub fn build_predicate_not_contains_fn(not_contains: Vec<String>) -> impl Fn(&Path) -> bool {
    move |p| {
        if let Ok(content) = std::fs::read_to_string(p) {
            not_contains.iter().all(|c| !content.contains(c))
        } else {
            false
        }
    }
}

/// build glob match fn
//TODO: review
pub fn build_glob_match_fn(glob: String) -> impl Fn(&Path) -> bool {
    move |p| {
        #[cfg(target_os = "windows")]
        let p = p.to_slash().expect("to_slash").to_string();

        #[cfg(not(target_os = "windows"))]
        let p = p.display().to_string();

        glob_match(&glob, &p)
    }
}

/// and predicate fns
pub fn and_predicate_path_fns(fns: Vec<Box<PredicatePathFn>>) -> impl Fn(&Path) -> bool {
    move |path| {
        for f in fns.iter() {
            if !f(path) {
                return false;
            }
        }

        true
    }
}

/// not predicate
pub fn not_predicated_fn(f: Box<PredicatePathFn>) -> impl Fn(&Path) -> bool {
    move |path| !f(path)
}

fn trim(ext_name: &str) -> &str {
    ext_name.strip_prefix(".").unwrap_or(ext_name)
}

#[cfg(test)]
mod tests {
    use super::trim;
    #[test]
    fn test_trim() {
        assert_eq!("txt", trim(".txt"));
        assert_eq!("txt", trim("txt"));

        assert_eq!(".txt", trim("..txt"));
        assert_eq!("", trim("."));
        assert_eq!("", trim(""));
    }
}
