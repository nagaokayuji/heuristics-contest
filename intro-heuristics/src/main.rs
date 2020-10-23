#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {d:usize,c:[i64;26],s:[[i64;26];d]};

    let mut usable = BTreeSet::new();
    for i in 0..26 {
        usable.insert(i);
    }
    for day in 0..d {
        if usable.len() == 0 {
            for i in 0..26 {
                usable.insert(i as usize);
            }
        }
        let contest = *usable.iter().max_by_key(|&&x| c[x as usize]).unwrap();
        usable.remove(&contest);
        println!("{}", contest + 1);
    }
}
fn sm(start: usize, end: usize) -> i64 {
    ((end * (end + 1) / 2) - (start * (start + 1) / 2)) as i64
}
static MX: usize = 1010101;
static MOD: i64 = 1000000007;
static INF: i64 = std::i64::MAX >> 1;
trait BinarySearch<T> {
    fn lower_bound_by<F: Fn(&T) -> bool>(&self, f: F) -> usize;
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound_by<F: Fn(&T) -> bool>(&self, f: F) -> usize {
        let mut ng = -1;
        let mut ok = self.len() as i64;
        while (ok as i32 - ng as i32).abs() > 1 {
            let mid = (ok + ng) / 2;
            if f(&self[mid as usize]) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }
    fn lower_bound(&self, x: &T) -> usize {
        self.lower_bound_by(|y| y >= x)
    }

    fn upper_bound(&self, x: &T) -> usize {
        self.lower_bound_by(|y| y > x)
    }
}
