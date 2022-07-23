#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
struct MinStack {
    data: Vec<i32>,
    min_stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            data: vec![],
            min_stack: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.data.push(val);

        if !self.min_stack.is_empty() {
            if self.min_stack.last().unwrap().lt(&val) {
                return;
            }
        }

        self.min_stack.push(val);
    }

    fn pop(&mut self) {
        if let Some(x) = self.data.pop() {
            if let Some(&y) = self.min_stack.last() {
                if y == x {
                    self.min_stack.pop();
                }
            }
        }
    }

    fn top(&self) -> i32 {
        self.data.last().unwrap().clone()
    }

    fn get_min(&self) -> i32 {
        self.min_stack.last().unwrap().clone()
    }
}
