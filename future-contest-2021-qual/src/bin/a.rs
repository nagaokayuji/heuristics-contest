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
    input! {//
        xy:[(usize,usize);100],
    }
    let mut input = Input { xy: xy };
    // greedy(&input);
    centering(&input);
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
    let mut visited = vec![false; 100];
    let mut nearest = 0;
    let mut neareset_distance = INF;
    for i in 0..100 {
        if chmin!(neareset_distance, dif((0, 0), input.xy[i])) {
            nearest = i;
        }
    }
    // dbg!(nearest);
    // dbg!(&input.xy[nearest]);
    visited[nearest] = true;
    state.move_to(input.xy[nearest]);
    state.push();

    let mut stack = vec![];
    stack.push(nearest);
    while let Some(pp) = stack.pop() {
        visited[pp] = true;
        let mut nearest_dis = INF;
        let mut nearest = None;
        for i in 0..100 {
            if i == pp || visited[i] {
                continue;
            }
            if chmin!(nearest_dis, dif(input.xy[pp], input.xy[i])) {
                nearest = Some(i);
            }
        }
        if let Some(a) = nearest {
            state.move_to(input.xy[a]);
            state.push();
            stack.push(a);
        }
    }
    // dbg!(&state.operations);
    // dbg!(&state.took);
    // 全部回収した
    let mut used = vec![vec![false; 20]; 20];
    while state.took.len() > 0 {
        let mut nearest_dis = INF;
        let mut nearest = None;
        for i in 0..20 {
            for j in 0..20 {
                if used[i][j] {
                    continue;
                }
                if chmin!(nearest_dis, dif(state.pos, (i, j))) {
                    nearest = Some((i, j));
                }
            }
        }
        if let Some(ij) = nearest {
            used[ij.0][ij.1] = true;
            state.move_to(ij);
            state.pop();
        }
    }
    // // 最後
    for to in 0..100 {
        let target = state.field[to].unwrap();
        state.move_to(target);
        state.push();
    }
    state.output();
}
fn dif(a: (usize, usize), b: (usize, usize)) -> i64 {
    let dx = a.0 as i64 - b.0 as i64;
    let dy = a.1 as i64 - b.1 as i64;
    (dx.abs() + dy.abs()) as i64
}
struct Input {
    xy: Vec<(usize, usize)>,
}
impl Input {}

/// 解と現在地
struct State {
    pos: (usize, usize),
    field: Vec<Option<(usize, usize)>>,
    rev_field: Vec<Vec<Option<usize>>>,
    operations: Vec<char>,
    score: usize,
    took: Vec<usize>,
}
impl State {
    /// 最初の状態
    fn new(input: &Input) -> State {
        let mut rev_field = vec![vec![None; 20]; 20];
        for (i, &(x, y)) in input.xy.iter().enumerate() {
            rev_field[x][y] = Some(i);
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
            dbg!("やばい@push");
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
            dbg!("やばそう@pop");
            return;
        }
        if self.rev_field[self.pos.0][self.pos.1] != None {
            dbg!("やばい@pop");
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
}
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
