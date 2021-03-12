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
    let mut count = 0i64;
    loop {
        count += 1;
        if count % 100 == 0 {
            if get_time() > LIMIT {
                break;
            }
        }
        state.random_change(&input);
    }
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
    fn is_valid(&self) -> bool {
        return (0..10000).contains(&self.sx)
            && (0..10000).contains(&self.sy)
            && (0..10000).contains(&self.ex)
            && (0..10000).contains(&self.ey);
    }
    fn next(&self, direction: usize) -> Option<Rect> {
        let dsx = [-2, -1, 0, 0, 0, -1, 1, 0, 0, 0];
        let dsy = [-2, -1, -1, 0, 0, 0, 0, 1, 0, 0];
        let dex = [2, 1, 0, 1, 0, 0, 0, 0, -1, 0];
        let dey = [2, 1, 0, 0, 1, 0, 0, 0, 0, -1];

        let rect = Rect {
            sx: self.sx + dsx[direction],
            sy: self.sy + dsy[direction],
            ex: self.ex + dex[direction],
            ey: self.ey + dey[direction],
        };
        if !rect.is_valid() {
            return None;
        }
        return Some(rect);
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
    score: f64,
    threshold: f64,
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
        let score = calc_score(&input, &out);
        State {
            out: out,
            score: score,
            threshold: 0.5f64,
        }
    }

    fn score(&self, input: &Input) -> f64 {
        return calc_score(input, &self.out);
    }

    fn random_change(&mut self, input: &Input) {
        let mut rng = thread_rng();
        let n = self.out.len();

        // decide target index
        let mut index: usize = rng.gen_range(0, n);
        let mut count = 0i64;
        while one_score(input.xyr[index].2, self.out[index].area()) < self.threshold {
            index = rng.gen_range(0, n);
            count += 1;
            if count % 100 == 0 {
                if get_time() > LIMIT {
                    return;
                }
            }
        }

        let mut nx = self.out.clone();

        // 4 directions
        let mut best = self.out.clone();
        let mut best_score = self.score(&input);

        let mut rect: Rect = self.out[index];
        let mut best_one_score = one_score(input.xyr[index].2, rect.area());

        for direction in 0..12 {
            let nextRect = rect.next(direction);
            if let Some(x) = nextRect {
                nx[index] = x;
                if !is_valid(&input, &nx) {
                    continue;
                }
                if best_one_score < one_score(input.xyr[index].2, x.area()) {
                    best = nx.clone();
                    break;
                }
                // let nx_score = calc_score(&input, &nx);
                // if best_score < nx_score {
                //     best = nx.clone();
                //     best_score = nx_score;
                // }
            }
        }
        self.out = best;
        // self.score = best_score;
    }
}

fn one_score(request: i64, area: i64) -> f64 {
    let mn = min(request, area) as f64;
    let mx = max(request, area) as f64;
    return mn / mx;
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
        sum += mn / mx;
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
