fn main() {}

struct Solution;

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */
struct MyStack {
    data: std::cell::RefCell<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            data: std::cell::RefCell::new(Vec::new()),
        }
    }

    /** Push element x onto stack. */
    fn push(&self, x: i32) {
        self.data.borrow_mut().push(x);
    }

    /** Removes the element on top of the stack and returns that element. */
    fn pop(&self) -> i32 {
        self.data.borrow_mut().pop().unwrap()
    }

    /** Get the top element. */
    fn top(&self) -> i32 {
        *self.data.borrow_mut().last().unwrap()
    }

    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        self.data.borrow().len() == 0
    }
}
