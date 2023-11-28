#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

struct FrontMiddleBackQueue {
    pre: std::collections::LinkedList<i32>,
    back: std::collections::LinkedList<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
/**
 * Your FrontMiddleBackQueue object will be instantiated and called as such:
 * let obj = FrontMiddleBackQueue::new();
 * obj.push_front(val);
 * obj.push_middle(val);
 * obj.push_back(val);
 * let ret_4: i32 = obj.pop_front();
 * let ret_5: i32 = obj.pop_middle();
 * let ret_6: i32 = obj.pop_back();
 */
impl FrontMiddleBackQueue {
    fn new() -> Self {
        Self {
            pre: std::collections::LinkedList::new(),
            back: std::collections::LinkedList::new(),
        }
    }

    fn push_front(&mut self, val: i32) {
        if self.pre.len() > self.back.len() {
            if let Some(x) = self.pre.pop_back() {
                self.back.push_front(x);
            }
        }
        self.pre.push_front(val);
    }

    fn push_middle(&mut self, val: i32) {
        match self.pre.len() - self.back.len() {
            1 => {
                if let Some(x) = self.pre.pop_back() {
                    self.back.push_front(x);
                }
                self.pre.push_back(val);
            }
            _ => self.pre.push_back(val),
        }
    }

    fn push_back(&mut self, val: i32) {
        self.back.push_back(val);
        if self.back.len() > self.pre.len() {
            if let Some(x) = self.back.pop_front() {
                self.pre.push_back(x);
            }
        }
    }

    fn pop_front(&mut self) -> i32 {
        if self.pre.len() == self.back.len() {
            if let Some(x) = self.back.pop_front() {
                self.pre.push_back(x);
            }
        }

        self.pre.pop_front().unwrap_or(-1)
    }

    fn pop_middle(&mut self) -> i32 {
        match self.pre.len() - self.back.len() {
            1 => self.pre.pop_back().unwrap_or(-1),
            _ => {
                let val = self.pre.pop_back().unwrap_or(-1);
                if let Some(x) = self.back.pop_front() {
                    self.pre.push_back(x);
                }
                val
            }
        }
    }

    fn pop_back(&mut self) -> i32 {
        if self.pre.len() - self.back.len() > 0 {
            if let Some(x) = self.pre.pop_back() {
                self.back.push_front(x);
            }
        }

        self.back.pop_back().unwrap_or(-1)
    }
}
