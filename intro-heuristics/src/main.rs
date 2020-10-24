#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
macro_rules ! chmin {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_min = min ! ($ ($ cmps ) ,+ ) ; if $ base > cmp_min {$ base = cmp_min ; true } else {false } } } ; }
macro_rules ! chmax {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_max = max ! ($ ($ cmps ) ,+ ) ; if $ base < cmp_max {$ base = cmp_max ; true } else {false } } } ; }
macro_rules ! min {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: min ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: min ($ a , min ! ($ ($ rest ) ,+ ) ) } } ; }
macro_rules ! max {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: max ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: max ($ a , max ! ($ ($ rest ) ,+ ) ) } } ; }
fn main() {
    input! {d:usize,c:[i64;26],s:[[i64;26];d]};

    let mut input = Input { D: d, s: s, c: c };
    let ans = solve_greedy(&input);
    for &x in ans.iter() {
        println!("{}", x + 1);
    }
}
struct Input {
    D: usize,
    s: Vec<Vec<i64>>,
    c: Vec<i64>,
}
// from editorial
fn calc_score(input: &Input, out: &Vec<usize>) -> i64 {
    let mut score = 0i64;
    let mut last = vec![0; 26];
    for d in 0..out.len() {
        last[out[d]] = d + 1;
        for i in 0..26 {
            score -= (d + 1 - last[i]) as i64 * input.c[i];
        }
        // d 日目 に out[d] を開催したときに増える満足度
        score += input.s[d][out[d]];
    }
    score
}
fn solve_greedy(input: &Input) -> Vec<usize> {
    let mut out = vec![];
    for day in 0..input.D {
        let mut max_score = -INF;
        let mut best_i = 0usize;
        for i in 0..26 {
            out.push(i);
            let score = calc_score(&input, &out);
            if chmax!(max_score, score) {
                best_i = i;
            }
            out.pop();
        }
        out.push(best_i);
    }
    out
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
