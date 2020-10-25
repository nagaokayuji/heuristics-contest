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
fn main() {
    dbg!(get_time());
    input! {d:usize,c:[i64;26],s:[[i64;26];d]};

    let mut input = Input { D: d, s: s, c: c };
    // let ans = solve_greedy_evaluate_wrapper(&input);
    // let ans = solve_greedy(&input);
    let ans = localSearch(&input);
    for &x in ans.iter() {
        println!("{}", x + 1);
    }
    dbg!(get_time());
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
/// 日毎に最大になる選択を採用する
/// score: 62634806
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

/// editorial pp.3
/// 評価関数
/// k 日後までコンテストを開催しない場合における満足度
fn evaluate(input: &Input, out: &Vec<usize>, k: usize) -> i64 {
    let mut score = 0i64;
    let mut last = vec![0; 26];
    for d in 0..out.len() {
        last[out[d]] = d + 1;
        for i in 0..26 {
            score -= (d + 1 - last[i]) as i64 * input.c[i];
        }
        score += input.s[d][out[d]];
    }
    for d in out.len()..(out.len() + k).min(input.D) {
        for i in 0..26 {
            score -= (d + 1 - last[i]) as i64 * input.c[i];
        }
    }
    score
}
/// k を色々試す
fn solve_greedy_evaluate_wrapper(input: &Input) -> Vec<usize> {
    let mut max_score = i64::min_value();
    let mut max_k = 1;
    for k in 1..14 {
        let out = solve_greedy_evaluate(&input, k);
        let score = calc_score(&input, &out);
        if chmax!(max_score, score) {
            max_k = k;
        }
    }
    let out = solve_greedy_evaluate(&input, max_k);
    out
}
/// 評価関数
fn solve_greedy_evaluate(input: &Input, k: usize) -> Vec<usize> {
    let mut out = vec![];
    for _ in 0..input.D {
        let mut max_score = i64::min_value();
        let mut best_i = 0;
        for i in 0..26 {
            out.push(i);
            let score = evaluate(&input, &out, k);
            if chmax!(max_score, score) {
                best_i = i;
            }
            out.pop();
        }
        out.push(best_i);
    }
    out
}
/// 実行時間
/// editorial より
fn get_time() -> f64 {
    // ↓なるほど
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
/// 局所探索（山登り法）
/// ランダムな初期階からスタート
/// 一点変更と二点スワップ（editorial）
fn localSearch(input: &Input) -> Vec<usize> {
    const TL: f64 = 1.98f64;
    let mut rng = rand::thread_rng();
    // let mut out = (0..input.D)
    //     .map(|_| rng.gen_range(0, 26))
    //     .collect::<Vec<_>>();
    // 初期値を貪欲
    let mut out = solve_greedy_evaluate_wrapper(&input);
    let mut score = calc_score(&input, &out);
    while get_time() < TL {
        if rng.gen_bool(0.5) {
            // 日
            let d = rng.gen_range(0, input.D);
            // 変更後のコンテスト
            let q = rng.gen_range(0, 26);
            let old = out[d];
            out[d] = q;
            let new_score = calc_score(&input, &out);
            // 劣化したら戻す
            if !chmax!(score, new_score) {
                out[d] = old;
            }
        } else {
            let d1 = rng.gen_range(0, input.D - 1);
            let d2 = rng.gen_range(d1.saturating_sub(7), (d1 + 7).min(input.D));
            let d3 = rng.gen_range(d2.saturating_sub(7), (d2 + 7).min(input.D));
            out.swap(d1, d2);
            out.swap(d1, d3);
            let new_score = calc_score(&input, &out);
            if !chmax!(score, new_score) {
                out.swap(d1, d3);
                out.swap(d1, d2);
            }
        }
    }
    out
}

fn sm(start: usize, end: usize) -> i64 {
    ((end * (end + 1) / 2) - (start * (start + 1) / 2)) as i64
}
const INF: i64 = 1 << 60;
