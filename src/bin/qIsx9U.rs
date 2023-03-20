#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

struct MovingAverage {
    v: Vec<f64>,
    len: f64,
    index: usize,
    sum: f64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {
    /** Initialize your data structure here. */
    fn new(size: i32) -> Self {
        Self {
            v: vec![0f64; size as usize],
            len: 0f64,
            index: 0,
            sum: 0f64,
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        self.sum += val as f64;
        self.sum -= self.v[self.index];
        self.v[self.index] = val as f64;
        self.index += 1;

        if self.len < self.v.len() as f64 {
            self.len += 1f64;
        }

        if self.index >= self.v.len() {
            self.index = 0;
        }

        self.sum / self.len as f64
    }
}
