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
}
struct Input {
    xy: Vec<(usize, usize)>,
}

struct State {
    pos: (usize, usize),
    operations: Vec<char>,
}
impl State {
    fn move_to(&mut self, input: &Input, dist: (usize, usize)) {
        self.pos = dist;
        // TODO: 移動
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
