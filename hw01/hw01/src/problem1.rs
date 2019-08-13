use std::collections::HashSet;

/// Computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
    let mut cnt = 0;
    for x in slice {
        cnt += x;
    }
    cnt
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut vec = Vec::new();
    let mut set = HashSet::new();
    for x in vs.clone() {
        if !set.contains(&x) {
            set.insert(x);
            vec.push(x);
        } 
    }
    vec
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut vec = Vec::new();
    for x in vs.clone() {
        if pred(x) {
            vec.push(x);
        }
    }
    vec
}