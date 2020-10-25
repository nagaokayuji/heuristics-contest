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
const LIMIT: f64 = 1.88;
#[fastout]
fn main() {
    dbg!(get_time());
    input! {d:usize,c:[i64;26],s:[[i64;26];d],};

    let mut input = Input { D: d, s: s, c: c };
    // let mut state = State::new(&input, t);
    // for &(d, q) in dq.iter() {
    //     state.change(&input, d, q);
    //     println!("{}", state.score);
    // }
    // // let ans = solve_greedy_evaluate_wrapper(&input);
    // // let ans = solve_greedy(&input);
    // let ans = localSearch(&input);
    let ans = simulated_annealing(&input);
    for &x in ans.iter() {
        println!("{}", x + 1);
    }
    dbg!(get_time());
}
/// 入力
struct Input {
    D: usize,
    s: Vec<Vec<i64>>,
    c: Vec<i64>,
}
/// 差分計算高速化
/// editorial pp.7
/// ds:= 各コンテストタイプごとの開催日
struct State {
    out: Vec<usize>,
    score: i64,
    ds: Vec<Vec<usize>>,
}

fn cost(a: usize, b: usize) -> i64 {
    let d = b - a;
    (d * (d - 1) / 2) as i64
}
impl State {
    fn new(input: &Input, out: Vec<usize>) -> State {
        let mut ds = vec![vec![]; 26];
        for d in 0..input.D {
            ds[out[d]].push(d + 1);
        }
        let score = calc_score(&input, &out);
        State { out, score, ds }
    }
    fn change(&mut self, input: &Input, d: usize, new_i: usize) {
        // d 日目に開催していたコンテスト
        let old_i = self.out[d];
        // index (position() := true を返す最初の要素のインデックス)
        let p = self.ds[old_i].iter().position(|a| *a == d + 1).unwrap();
        // 一つ前の開催日
        let prev = self.ds[old_i].get(p.wrapping_sub(1)).cloned().unwrap_or(0);
        // 一つ後の開催日
        let next = self.ds[old_i].get(p + 1).cloned().unwrap_or(input.D + 1);
        // 該当コンテストを削除
        self.ds[old_i].remove(p);
        // スコアの差分
        self.score += (cost(prev, d + 1) + cost(d + 1, next) - cost(prev, next)) * input.c[old_i];

        // d 日目以降で new_i を開催する日
        let p = self.ds[new_i]
            .iter()
            .position(|a| *a > d + 1)
            .unwrap_or(self.ds[new_i].len());
        // 一つ前の開催日
        let prev = self.ds[new_i].get(p.wrapping_sub(1)).cloned().unwrap_or(0);
        // 一つ後の開催日
        let next = self.ds[new_i].get(p).cloned().unwrap_or(input.D + 1);
        // ds に追加
        self.ds[new_i].insert(p, d + 1);
        // 差分計算
        self.score -= (cost(prev, d + 1) + cost(d + 1, next) - cost(prev, next)) * input.c[new_i];
        self.score += input.s[d][new_i] - input.s[d][old_i];
        // out 書き換え
        self.out[d] = new_i;
    }
}

/// from editorial
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
    const TL: f64 = 0.3f64;
    let mut rng = thread_rng();
    // let mut out = (0..input.D)
    //     .map(|_| rng.gen_range(0, 26))
    //     .collect::<Vec<_>>();
    // 初期値を貪欲
    let mut out = solve_greedy_evaluate_wrapper(&input);
    let mut state = State::new(&input, out);
    let mut score = state.score;
    while get_time() < TL {
        if rng.gen_bool(0.5) {
            let d1 = rng.gen_range(0, input.D);
            let d2 = rng.gen_range(0, input.D);
            let q1 = rng.gen_range(0, 26);
            let q2 = rng.gen_range(0, 26);
            let old1 = state.out[d1];
            let old2 = state.out[d2];
            state.change(&input, d1, q1);
            state.change(&input, d2, q2);
            let new_score = state.score;
            if !chmax!(score, new_score) {
                state.change(&input, d1, old1);
                state.change(&input, d2, old2);
            }
        } else {
            let mut out = state.out.clone();
            let d1 = rng.gen_range(0, input.D - 1);
            let d2 = rng.gen_range(d1.saturating_sub(7), (d1 + 7).min(input.D));
            let d3 = rng.gen_range(d2.saturating_sub(7), (d2 + 7).min(input.D));
            out.swap(d1, d2);
            out.swap(d1, d3);
            let new_score = calc_score(&input, &out);
            if chmax!(score, new_score) {
                state = State::new(&input, out);
            }
        }
    }
    state.out
}

/// 焼きなまし法
/// editorial pp.10
fn simulated_annealing(input: &Input) -> Vec<usize> {
    const T0: f64 = 2e3; // 開始時点の温度
    const T1: f64 = 6e2; // 低い温度
    const TL: f64 = 1.92;
    let mut rng = thread_rng();
    // 局所探索からのパターン
    // let mut state = State::new(input, localSearch(&input));
    // ランダムな解からのパターン
    // let mut state = State::new(input, (0..input.D).map(|_| rng.gen_range(0, 26)).collect());
    // 貪欲解からのパターン
    let mut state = State::new(input, solve_greedy_evaluate_wrapper(&input));
    let mut T = T0;
    let mut best = state.score;
    let mut best_out = state.out.clone();
    let mut cnt = 0i64;
    loop {
        cnt += 1;
        if cnt % 85 == 0 {
            // 時刻を[0,1] に正規化
            let t = get_time() / TL;
            if t >= 1.0 {
                break;
            }
            T = T0.powf(1.0 - t) * T1.powf(t);
        }
        let old_score = state.score;
        // d日目のコンテストを適当に変更 or d1 日目 と d2 日目をスワップ
        if rng.gen_bool(0.3) {
            let d = rng.gen_range(0, input.D);
            let old = state.out[d];
            state.change(input, d, rng.gen_range(0, 26));
            // 劣化しても一定確率で変更する
            if old_score > state.score
                && !rng.gen_bool(f64::exp((state.score - old_score) as f64 / T))
            {
                state.change(input, d, old);
            }
        } else if rng.gen_bool(0.2) {
            let d1 = rng.gen_range(0, input.D);
            let old1 = state.out[d1];
            let d2 = rng.gen_range(0, input.D);
            let old2 = state.out[d2];
            state.change(input, d1, rng.gen_range(0, 26));
            state.change(input, d2, rng.gen_range(0, 26));
            if old_score > state.score
                && !rng.gen_bool(f64::exp((state.score - old_score) as f64 / T))
            {
                state.change(input, d1, old1);
                state.change(input, d2, old2);
            }
        } else if rng.gen_bool(0.3) {
            let mut d1 = rng.gen_range(0, input.D);
            let mut d2 = rng.gen_range(d1.saturating_sub(8), (d1 + 8).min(input.D));
            let (a, b) = (state.out[d1], state.out[d2]);
            state.change(input, d1, b);
            state.change(input, d2, a);
            if old_score > state.score
                && !rng.gen_bool(f64::exp((state.score - old_score) as f64 / T))
            {
                state.change(input, d1, a);
                state.change(input, d2, b);
            }
        } else {
            let mut d1 = rng.gen_range(0, input.D);
            let mut d2 = rng.gen_range(d1.saturating_sub(8), (d1 + 8).min(input.D));
            let mut d3 = rng.gen_range(d1.saturating_sub(8), (d2 + 8).min(input.D));
            let (a, b, c) = (state.out[d1], state.out[d2], state.out[d3]);
            state.change(input, d1, b);
            state.change(input, d2, c);
            state.change(input, d3, a);
            if old_score > state.score
                && !rng.gen_bool(f64::exp((state.score - old_score) as f64 / T))
            {
                state.change(input, d1, a);
                state.change(input, d2, b);
                state.change(input, d3, c);
            }
        }
        if chmax!(best, state.score) {
            best_out = state.out.clone();
        }
    }
    best_out
}

fn sm(start: usize, end: usize) -> i64 {
    ((end * (end + 1) / 2) - (start * (start + 1) / 2)) as i64
}
const INF: i64 = 1 << 60;
