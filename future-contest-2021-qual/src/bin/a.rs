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
    input! {//
        xy:[(usize,usize);100],
    }
    get_time();
    let mut input = Input { xy: xy };
    // greedy(&input);
    // centering(&input);
    simulated_annealing(&input);
}
/// 貪欲(単純)
fn greedy(input: &Input) {
    // 初期値
    let mut state = State::new(&input);
    for to in 0..100 {
        let target = input.xy[to];
        state.move_to(target);
        state.push();
    }
    state.output();
}
fn centering(input: &Input) {
    const INF: i64 = 1 << 60;
    let mut state = State::new(&input);

    let mut s1 = state.clone();
    let mut s2 = state.clone();
    let mut s3 = state.clone();
    let mut s4 = state.clone();
    let mut s5 = state.clone();
    s1.pick_shortest();
    s2.pick_block(2);
    s3.pick_block(4);
    s4.pick_block(5);
    s5.pick_block(10);
    dbg!((s1.score, s2.score, s3.score, s4.score, s5.score));

    state.score = 123456;
    if chmin!(state.score, s1.score) {
        state = s1;
    }
    if chmin!(state.score, s2.score) {
        state = s2;
    }
    if chmin!(state.score, s3.score) {
        state = s3;
    }
    if chmin!(state.score, s4.score) {
        state = s4;
    }
    if chmin!(state.score, s5.score) {
        state = s5;
    }
    // state = s4;

    let mut nice = state.took.clone();
    let mut gr_out = vec![];
    while let Some(x) = nice.pop() {
        gr_out.push(x);
    }
    gr_out.reverse();

    let out = localSearch(&input, &gr_out);
    state = State::new(&input);

    // // 配置
    state.pick_by_out_and_place(&out);
    // 集める
    state.collect_imp();
    // 出力
    state.output();
}
fn simulated_annealing(input: &Input) {
    const INF: i64 = 1 << 60;
    let mut state = State::new(&input);
    let mut s1 = state.clone();
    let mut s2 = state.clone();
    let mut s3 = state.clone();
    let mut s4 = state.clone();
    let mut s5 = state.clone();
    s1.pick_shortest();
    s2.pick_block(2);
    s3.pick_block(4);
    s4.pick_block(5);
    s5.pick_block(10);
    state.score = 123456;
    if chmin!(state.score, s1.score) {
        state = s1;
    }
    if chmin!(state.score, s2.score) {
        state = s2;
    }
    if chmin!(state.score, s3.score) {
        state = s3;
    }
    if chmin!(state.score, s4.score) {
        state = s4;
    }
    if chmin!(state.score, s5.score) {
        state = s5;
    }
    let mut nice = state.took.clone();
    let mut out = vec![];
    while let Some(x) = nice.pop() {
        out.push(x);
    }
    out.reverse();

    const T0: f64 = 2e2; // 開始時点の温度
    const T1: f64 = 5f64; // 低い温度
    const TL: f64 = 2.7f64;
    let mut rng = thread_rng();
    let mut T = T0;
    let mut state = State::new(&input);
    let mut best = state.evaluate(&out);
    let mut best_out = out.clone();
    let mut cnt = 0i64;
    let mut rng = thread_rng();
    loop {
        cnt += 1;
        if cnt % 100 == 0 {
            let t = get_time() / TL;
            if t >= 1.0 {
                break;
            }
            T = T0.powf(1.0 - t) * T1.powf(t);
        }
        let old_score = best;
        let p1 = rng.gen_range(0, 100);
        let p2 = rng.gen_range(0, 100);
        let mut new_out = best_out.clone();
        new_out.swap(p1, p2);
        let mut state = State::new(&input);
        let new_score = state.evaluate(&new_out);
        if new_score > best {
            // dbg!("imp");
            best_out = new_out;
        } else {
            if rng.gen_bool(f64::exp((new_score - best) as f64 / T)) {
                best_out = new_out;
            }
        }
    }
    dbg!(cnt);
    let mut state = State::new(&input);
    // // 配置
    state.pick_by_out_and_place(&best_out);
    // 集める
    state.collect_imp();
    // 出力
    state.output();
    // dbg!(state.score);
}
/// 二点間の距離
fn dif(a: (usize, usize), b: (usize, usize)) -> i64 {
    let dx = a.0 as i64 - b.0 as i64;
    let dy = a.1 as i64 - b.1 as i64;
    (dx.abs() + dy.abs()) as i64
}
/// 入力
struct Input {
    xy: Vec<(usize, usize)>,
}
impl Input {}

/// 局所探索
fn localSearch(input: &Input, initial: &Vec<usize>) -> Vec<usize> {
    let mut TL = 1.5f64;
    let mut out = vec![0; 100];
    out = initial.clone();
    let mut rng = thread_rng();
    let mut score = calc_score_pick_up(&input, &out);
    let mut cnt = 0i64;
    loop {
        cnt += 1;
        if cnt % 300 == 0 && get_time() > TL {
            break;
        }
        let p1 = rng.gen_range(0, 100);
        let p2 = rng.gen_range(0, 100);
        let p3 = rng.gen_range(0, 100);

        let mut new_out = out.clone();
        new_out.swap(p1, p2);
        new_out.swap(p1, p3);
        new_out.swap(p2, p3);
        let new_score = calc_score_pick_up(&input, &new_out);
        if score > new_score {
            out = new_out;
        }
    }
    out
}
fn calc_score_pick_up(input: &Input, out: &Vec<usize>) -> i64 {
    let mut now = (0, 0);
    let mut score = 0i64;
    for &x in out.iter() {
        let target = input.xy[x];
        score += dif(target, now);
        now = target;
    }
    score
}
/// 解と現在地
#[derive(Clone)]
struct State {
    pos: (usize, usize),
    field: Vec<Option<(usize, usize)>>,
    rev_field: Vec<Vec<Option<usize>>>,
    operations: Vec<char>,
    score: usize,
    took: Vec<usize>,
    available: Vec<Vec<bool>>,
    over: Vec<usize>,
}
impl State {
    /// 高いほうがよい評価
    fn evaluate(&mut self, out: &Vec<usize>) -> i64 {
        // // 配置
        self.pick_by_out_and_place(&out);
        // 集める
        self.collect_imp();
        10000 - self.score as i64 * 4
    }
    /// 最初の状態
    fn new(input: &Input) -> State {
        let mut rev_field = vec![vec![None; 20]; 20];
        for (i, &(x, y)) in input.xy.iter().enumerate() {
            rev_field[x][y] = Some(i);
        }
        let mut available = vec![vec![false; 20]; 20];
        let center = (9, 9);
        for i in 0..20 {
            for j in 0..20 {
                if dif((i, j), center) <= 7 {
                    available[i][j] = true;
                }
            }
        }
        State {
            pos: (0, 0),
            field: input
                .xy
                .iter()
                .cloned()
                .map(|x| Some(x))
                .collect::<Vec<_>>(),
            rev_field: rev_field,
            operations: vec![],
            score: 0,
            took: vec![],
            available: available,
            over: vec![],
        }
    }
    fn move_to(&mut self, dist: (usize, usize)) {
        let bef = self.operations.len();
        for _ in self.pos.0..dist.0 {
            self.operations.push('D');
        }
        for _ in dist.0..self.pos.0 {
            self.operations.push('U');
        }
        for _ in self.pos.1..dist.1 {
            self.operations.push('R');
        }
        for _ in dist.1..self.pos.1 {
            self.operations.push('L');
        }
        let aft = self.operations.len();
        self.score += aft - bef;
        // 最後に書き換え
        self.pos = dist;
    }
    /// rev_field から どの数字か取得
    /// 現在地の数字を取得
    fn getNum(&self) -> Option<usize> {
        self.rev_field[self.pos.0][self.pos.1]
    }
    fn push(&mut self) {
        if self.getNum() == None {
            // dbg!("やばい@push");
            return;
        }
        self.operations.push('I');
        let val = self.getNum().unwrap();
        self.field[val] = None;
        self.took.push(val);
        self.rev_field[self.pos.0][self.pos.1] = None;
    }
    fn pop(&mut self) {
        if self.took.len() == 0 {
            // dbg!("やばそう@pop");
            return;
        }
        if self.rev_field[self.pos.0][self.pos.1] != None {
            // dbg!("やばい@pop");
            return;
        }
        self.operations.push('O');
        let val = self.took.pop();
        self.field[val.unwrap()] = Some(self.pos);
        self.rev_field[self.pos.0][self.pos.1] = val;
    }
    fn output(&self) {
        for &op in self.operations.iter() {
            print!("{}", op);
        }
    }
    fn gather(&mut self) {
        for to in 0..100 {
            let target = self.field[to].unwrap();
            self.move_to(target);
            self.push();
        }
    }
    /// 貪欲に最短経路にいく
    fn pick_shortest(&mut self) {
        let mut stack = vec![];
        let mut visited = vec![false; 100];
        self.move_to(self.field[0].unwrap());
        self.push();
        stack.push(0);
        visited[0] = true;
        while let Some(pp) = stack.pop() {
            visited[pp] = true;
            let mut nearest_dis = INF;
            let mut nearest = None;
            for i in 0..100 {
                if i == pp || visited[i] {
                    continue;
                }
                if let Some(t) = self.field[i] {
                    if chmin!(nearest_dis, dif(self.pos, t)) {
                        nearest = Some(i);
                    }
                }
            }
            if let Some(a) = nearest {
                self.move_to(self.field[a].unwrap());
                self.push();
                stack.push(a);
            }
        }
    }
    fn pick_block(&mut self, block_size: usize) {
        // 20の約数がいい？
        // let mut block_size = 4;
        for base_x in 0..20 / block_size {
            for y in 0..20 {
                let y = if base_x % 2 == 0 { y } else { 20 - y - 1 };
                for dx in 0..block_size {
                    let dx = if y % 2 == 0 { dx } else { block_size - dx - 1 };
                    let target = (base_x * block_size + dx, y);
                    if self.rev_field[target.0][target.1] != None {
                        self.move_to(target);
                        self.push();
                    }
                }
            }
        }
    }
    fn pick_by_out(&mut self, out: &Vec<usize>) {
        // dbg!(&out);
        for &x in out.iter() {
            let target = self.field[x].unwrap();
            self.move_to(target);
            self.push();
        }
    }
    fn pick_by_out_and_place(&mut self, out: &Vec<usize>) {
        // ひろう
        for &x in out.iter() {
            let target = self.field[x].unwrap();
            if self.place_judge(target, self.rev_field[target.0][target.1].unwrap())
                && self.available[target.0][target.1]
            {
                self.available[target.0][target.1] = false;
                continue;
            }
            self.move_to_and_place(target);
            self.push();
        }
        // 余ったやつをおく
        while let Some(pp) = self.took.last() {
            let mut best = (0, 0);
            let mut best_dis = INF;
            for i in 0..20 {
                for j in 0..20 {
                    if self.available[i][j] && chmin!(best_dis, dif((i, j), self.pos)) {
                        best = (i, j);
                    }
                }
            }
            self.available[best.0][best.1] = false;
            self.move_to_and_place(best);
            if self.place_judge(self.pos, *self.took.last().unwrap()) {
                self.pop();
                continue;
            }

            if self.took.len() >= 2 {
                let first = self.took.pop().unwrap();
                let second = self.took.pop().unwrap();
                self.took.push(second);
                self.took.push(first);
                if self.place_judge(self.pos, second) {
                    let mut best2 = (0, 0);
                    let mut bestdis = INF;
                    assert!(self.pos == best);
                    for i in 0..20 {
                        for j in 0..20 {
                            if self.available[i][j]
                                && self.rev_field[i][j] == None
                                && chmin!(bestdis, dif((i, j), best))
                            {
                                best2 = (i, j);
                            }
                        }
                    }
                    self.move_to(best2);
                    self.pop();
                    self.move_to(best);
                    self.pop();
                    self.available[best2.0][best2.1] = false;
                    // self.move_to(best2);
                    // self.push();
                    continue;
                }
            }
            self.pop();
        }
    }
    fn place_judge(&mut self, now: (usize, usize), num: usize) -> bool {
        (now.1 <= 9 && num < 40)
            || (now.1 >= 9 && num >= 40)
            || (num > 30 && num < 70 && now.0 > 5 && now.1 > 5 && now.0 < 14 && now.1 < 14)
    }
    fn move_to_and_place(&mut self, dist: (usize, usize)) {
        while self.pos != dist {
            if self.took.len() > 0 && self.available[self.pos.0][self.pos.1] {
                // dbg!("おく");
                if self.place_judge(self.pos, *self.took.last().unwrap()) {
                    self.pop();
                    self.available[self.pos.0][self.pos.1] = false;
                }
            }
            if self.pos.0 < dist.0 {
                self.move_to((self.pos.0 + 1, self.pos.1));
            }
            if self.pos.0 > dist.0 {
                self.move_to((self.pos.0 - 1, self.pos.1));
            }
            if self.pos.1 < dist.1 {
                self.move_to((self.pos.0, self.pos.1 + 1));
            }
            if self.pos.1 > dist.1 {
                self.move_to((self.pos.0, self.pos.1 - 1));
            }
        }
    }
    fn placing1(&mut self) {
        let mut last = self.pos;
        let (xs, xg, xr) = if last.0 + 10 >= 20 {
            (last.0 - 10, last.0, true)
        } else {
            (last.0, last.0 + 10, false)
        };
        let (ys, yg, yr) = if last.1 + 10 >= 20 {
            (last.1 - 10, last.1, true)
        } else {
            (last.1, last.1 + 10, false)
        };

        while let Some(pp) = self.took.last() {
            let t = pp / 10;
            let p = pp % 10;
            let dx = t;
            let dy = if t % 2 == 0 { p } else { 10 - p - 1 };
            let target = (xs + dx, ys + dy);
            self.move_to(target);
            self.pop();
        }
    }
    /// ひし形
    fn placing2(&mut self) {
        // 10,10 を中心
        let mut available = vec![vec![false; 20]; 20];
        let center = (9, 9);
        for i in 0..20 {
            for j in 0..20 {
                if dif((i, j), center) <= 7 {
                    available[i][j] = true;
                }
            }
        }

        while let Some(pp) = self.took.last() {
            let mut best = (0, 0);
            let mut best_dis = INF;
            for i in 0..20 {
                for j in 0..20 {
                    if available[i][j] && chmin!(best_dis, dif((i, j), self.pos)) {
                        best = (i, j);
                    }
                }
            }
            available[best.0][best.1] = false;
            self.move_to(best);
            self.pop();
        }
    }
    /// 集める
    fn collect(&mut self) {
        for to in 0..100 {
            let target = self.field[to].unwrap();
            self.move_to(target);
            self.push();
        }
    }
    /// 改善版
    ///
    fn collect_imp(&mut self) {
        let mut left = vec![];
        for i in (0..100) {
            left.push(i);
        }
        // ２こまでもつ
        let mut k = 2;
        for &x in left.iter() {
            if self.over.contains(&x) {
                self.picking(x);
            } else {
                let target = self.field[x].unwrap();
                self.move_to_and_collect(target, x, 3);
            }
            // self.move_to(target);
            // self.push();
        }
    }
    fn picking(&mut self, target: usize) {
        let mut lefts = vec![];
        let mut one = ((0, (0, 0)));
        for &x in self.over.clone().iter() {
            let target_x = self.find_near_pos(self.pos);
            self.move_to(target_x);
            self.pop();
            if x == target {
                one = (x, target_x);
            } else {
                lefts.push((x, target_x));
            }
        }
        self.move_to(one.1);
        self.push();
        self.over.clear();
        for &(x, target) in lefts.iter().rev() {
            self.move_to(target);
            self.push();
            self.over.push(x);
        }
    }
    fn move_to_and_collect(&mut self, dist: (usize, usize), target: usize, k: usize) {
        // let mut took = vec![];
        while self.pos != dist {
            if let Some(fnd) = self.rev_field[self.pos.0][self.pos.1] {
                // 拾っておく
                if target < fnd && target + k > fnd {
                    self.push();
                    self.over.push(fnd);
                }
            }
            if self.pos.0 < dist.0 {
                self.move_to((self.pos.0 + 1, self.pos.1));
            }
            if self.pos.0 > dist.0 {
                self.move_to((self.pos.0 - 1, self.pos.1));
            }
            if self.pos.1 < dist.1 {
                self.move_to((self.pos.0, self.pos.1 + 1));
            }
            if self.pos.1 > dist.1 {
                self.move_to((self.pos.0, self.pos.1 - 1));
            }
        }
        let mut lefts = vec![];
        for &x in self.over.clone().iter() {
            let target_x = self.find_near_pos(self.pos);
            lefts.push((x, target_x));
            self.move_to(target_x);
            self.pop();
        }
        self.move_to(dist);
        self.push();

        self.over.clear();
        for &(x, target) in lefts.iter().rev() {
            self.move_to(target);
            self.push();
            self.over.push(x);
        }
    }
    fn find_near_pos(&mut self, now: (usize, usize)) -> (usize, usize) {
        let mut best = (0, 0);
        let mut best_dis = INF;
        for i in 0..20 {
            for j in 0..20 {
                if self.rev_field[i][j] == None && chmin!(best_dis, dif((i, j), self.pos)) {
                    best = (i, j);
                }
            }
        }
        best
    }
}

const INF: i64 = 1 << 60;
struct Deque {
    data: Vec<(usize, usize)>, // カードの数字, index
    left: usize,
    right: usize,
    size: usize,
    buf_size: usize,
}
impl Deque {
    /// 初期化
    ///
    /// n バッファサイズ
    fn new(n: usize) -> Deque {
        Deque {
            data: vec![(0, 0); n],
            left: 0,
            right: 0,
            size: 0,
            buf_size: n,
        }
    }
    fn push_front(&mut self, x: (usize, usize)) {
        self.left = (self.buf_size + self.left - 1) % self.buf_size;
        self.data[self.left] = x;
        self.size += 1;
    }
    fn pop_front(&mut self) -> Option<(usize, usize)> {
        if self.size == 0 {
            return None;
        }
        let ret = self.data[self.left];
        self.left += 1;
        self.size -= 1;
        Some(ret)
    }
    fn push_back(&mut self, x: (usize, usize)) {
        self.right = (self.right + 1) % self.buf_size;
        self.data[self.right] = x;
        self.size += 1;
    }
    fn pop_back(&mut self) -> Option<(usize, usize)> {
        if self.size == 0 {
            return None;
        }
        let ret = self.data[self.right];
        self.right -= 1;
        self.size -= 1;
        Some(ret)
    }
}

pub trait BinarySearch<T> {
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
