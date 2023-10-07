#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

struct StockSpanner {
    n: i32,
    stack: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */
impl StockSpanner {
    fn new() -> Self {
        Self {
            n: -1,
            stack: vec![(-1, i32::MAX)],
        }
    }

    fn next(&mut self, price: i32) -> i32 {
        self.n += 1;

        while !self.stack.is_empty() && price >= self.stack[self.stack.len() - 1].1 {
            self.stack.pop();
        }

        self.stack.push((self.n, price));

        self.n - self.stack[self.stack.len() - 2].0
    }
}
