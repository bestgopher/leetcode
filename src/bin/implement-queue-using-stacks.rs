fn main() {}

struct Solution;

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
struct MyQueue {
    data: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self { data: vec![] }
    }

    /** Push element x to the back of queue. */
    fn push(&mut self, x: i32) {
        self.data.push(x);
    }

    /** Removes the element from in front of queue and returns that element. */
    fn pop(&mut self) -> i32 {
        let data = self.peek();
        self.data = self.data[1..].to_vec();
        data
    }

    /** Get the front element. */
    fn peek(&mut self) -> i32 {
        let data = self.data[0];
        data
    }

    /** Returns whether the queue is empty. */
    fn empty(&self) -> bool {
        self.data.len() == 0
    }
}

