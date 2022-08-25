#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

struct MaxQueue {
    queue: Vec<i32>,
    deque: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MaxQueue {
    fn new() -> Self {
        MaxQueue {
            queue: vec![],
            deque: vec![],
        }
    }

    fn max_value(&self) -> i32 {
        self.deque.first().map(|x| *x).unwrap_or(-1)
    }

    fn push_back(&mut self, value: i32) {
        while !self.deque.is_empty() {
            if self.deque[self.deque.len() - 1] >= value {
                break;
            }

            self.deque.pop();
        }

        self.deque.push(value);
        self.queue.push(value);
    }

    fn pop_front(&mut self) -> i32 {
        if self.deque.is_empty() {
            return -1;
        }

        let v = self.queue[0];
        self.queue = self.queue[1..].to_vec();

        if v == self.deque[0] {
            self.deque = self.deque[1..].to_vec();
        }

        v
    }
}

// /**
//  * Your MaxQueue object will be instantiated and called as such:
//  * let obj = MaxQueue::new();
//  * let ret_1: i32 = obj.max_value();
//  * obj.push_back(value);
//  * let ret_3: i32 = obj.pop_front();
//  */
