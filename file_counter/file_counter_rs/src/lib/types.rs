use std::path::Path;

/// the predicate path fn types
pub type PredicatePathFn = dyn Fn(&Path) -> bool;
