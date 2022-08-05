#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::vec;

fn main() {}

struct Solution;

struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>, // 辅助栈存放当前栈中的最小值
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            stack: vec![],
            min_stack: vec![],
        }
    }

    fn push(&mut self, x: i32) {
        if self.min_stack.is_empty() {
            self.min_stack.push(x);
        } else {
            if self.min_stack[self.min_stack.len() - 1] >= x {
                self.min_stack.push(x);
            }
        }

        self.stack.push(x);
    }

    fn pop(&mut self) {
        if let Some(x) = self.stack.pop() {
            if x == *self.min_stack.last().unwrap() {
                self.min_stack.pop();
            }
        }
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}

// Your MinStack object will be instantiated and called as such:
// let obj = MinStack::new();
// obj.push(x);
// obj.pop();
// let ret_3: i32 = obj.top();
// let ret_4: i32 = obj.min();
