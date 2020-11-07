#![allow(unused, non_snake_case, dead_code, non_upper_case_globals)]
use {
    proconio::{marker::*, *},
    std::*,
    std::{cmp::*, collections::*, convert::*, iter::*, marker::*, mem::*, ops::*},
};
fn main() {
    input! {//
        xy:[(usize,usize);100],
    }
    let mut input = Input { xy: xy };
    greedy(&input);
}
fn greedy(input: &Input) {
    // 初期値
    let mut state = State::new();
    for to in 0..100 {
        let target = input.xy[to];
        state.move_to(target);
        state.push();
    }
    state.output();
    // dbg!(&state.pos);
    // state.output();
    // state.move_to((2, 5));
    // state.output();
    // dbg!(&state.pos);
}
struct Input {
    xy: Vec<(usize, usize)>,
}
impl Input {}

/// 解と現在地
struct State {
    pos: (usize, usize),
    operations: Vec<char>,
    took: Deque,
}
impl State {
    /// 最初の状態
    fn new() -> State {
        State {
            pos: (0, 0),
            operations: vec![],
            took: Deque::new(123456),
        }
    }
    fn move_to(&mut self, dist: (usize, usize)) {
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
        // 最後に書き換え
        self.pos = dist;
    }
    fn push(&mut self) {
        self.operations.push('I');
    }
    fn output(&self) {
        for &op in self.operations.iter() {
            print!("{}", op);
        }
        println!();
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
