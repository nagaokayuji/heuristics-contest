#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    rand::*,
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
macro_rules ! chmin {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_min = min ! ($ ($ cmps ) ,+ ) ; if $ base > cmp_min {$ base = cmp_min ; true } else {false } } } ; }
macro_rules ! chmax {($ base : expr , $ ($ cmps : expr ) ,+ $ (, ) * ) => {{let cmp_max = max ! ($ ($ cmps ) ,+ ) ; if $ base < cmp_max {$ base = cmp_max ; true } else {false } } } ; }
macro_rules ! min {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: min ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: min ($ a , min ! ($ ($ rest ) ,+ ) ) } } ; }
macro_rules ! max {($ a : expr $ (, ) * ) => {{$ a } } ; ($ a : expr , $ b : expr $ (, ) * ) => {{std :: cmp :: max ($ a , $ b ) } } ; ($ a : expr , $ ($ rest : expr ) ,+ $ (, ) * ) => {{std :: cmp :: max ($ a , max ! ($ ($ rest ) ,+ ) ) } } ; }
const LIMIT: f64 = 4.88;
fn main() {
    input! {
        n: usize,
        xyr: [(i64,i64,i64); n],
    }
    get_time();
    dbg!(get_time());
    let input = Input { n: n, xyr: xyr };
    solve(&input);
}

fn solve(input: &Input) {}
/// 入力
struct Input {
    n: usize,
    xyr: Vec<(i64, i64, i64)>,
}
// 出力１個分
#[derive(Clone, Copy)]
struct Rect {
    sx: i64,
    sy: i64,
    ex: i64,
    ey: i64,
}
impl Rect {
    fn area(&self) -> i64 {
        // validation
        assert!(self.sx < self.ex);
        assert!(self.sy < self.ey);

        //
        return (self.ex - self.sx) * (self.ey - self.sy);
    }
}
fn output(out: &[Rect]) {
    for &rect in out.iter() {
        println!("{} {} {} {}", rect.sx, rect.sy, rect.ex, rect.ey);
    }
}
struct State {
    out: Vec<Rect>,
}
impl State {}
fn calc_score(input: &Input, out: &Vec<Rect>) -> f64 {}

/// 実行時間
fn get_time() -> f64 {
    static mut STIME: f64 = -1.0;
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        if STIME < 0.0 {
            STIME = ms;
        }
        ms - STIME
    }
}
const INF: i64 = 1 << 60;
