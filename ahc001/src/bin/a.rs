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

fn solve(input: &Input) {
    let mut state = State::new(&input);
    output(&state.out);
}
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

/// 答え出力
fn output(out: &[Rect]) {
    for &rect in out.iter() {
        println!("{} {} {} {}", rect.sx, rect.sy, rect.ex, rect.ey);
    }
}

#[derive(Clone)]
struct State {
    out: Vec<Rect>,
}
impl State {
    fn new(input: &Input) -> State {
        let mut out = vec![];
        for (i, &xyr) in input.xyr.iter().enumerate() {
            out.push(Rect {
                sx: xyr.0,
                sy: xyr.1,
                ex: xyr.0 + 1,
                ey: xyr.1 + 1,
            });
        }
        State { out: out }
    }
    fn score(&self, input: &Input) -> f64 {
        return calc_score(input, &self.out);
    }
    fn change(&self, index: usize, to: Rect) -> f64 {
        let mut nx = self.clone();
        nx.out[index] = to;
    }
}

fn is_valid(input: &Input, out: &Vec<Rect>) -> bool {
    // 他の矩形と被らないか
    fn is_valid_duplicate(out: &Vec<Rect>) -> bool {
        let n = out.len();
        for i in 0..n {
            for j in i + 1..n {
                if has_duplicate_area(&out[i], &out[j]) {
                    return false;
                }
            }
        }
        true
    }
    is_valid_duplicate(&out)
}
fn has_duplicate_area(a: &Rect, b: &Rect) -> bool {
    ((a.sx..=a.ex).contains(&b.sx) || (b.sx..=b.ex).contains(&a.sx))
        && ((a.sy..=a.ey).contains(&b.sy) || (b.sy..=b.ey).contains(&a.sy))
}

/// スコア計算
fn calc_score(input: &Input, out: &Vec<Rect>) -> f64 {
    let mut sum = 0f64;
    for i in 0..input.n {
        let mn = min(out[i].area(), input.xyr[i].2) as f64;
        let mx = max(out[i].area(), input.xyr[i].2) as f64;
        sum += 1f64 - (1f64 - mn / mx).powi(2);
    }
    sum
}

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
