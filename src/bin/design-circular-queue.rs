fn main() {}

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * let obj = MyCircularQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */
struct MyCircularQueue {
    cap: usize,
    start: usize,
    end: usize,
    len: usize,
    data: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    fn new(k: i32) -> Self {
        Self {
            cap: k as usize,
            len: 0,
            start: 0,
            end: 0,
            data: vec![0; k as usize],
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.len += 1;
        self.data[self.start] = value;

        if self.start == self.cap - 1 {
            self.start = 0;
        } else {
            self.start += 1;
        }

        true
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        self.len -= 1;

        self.data[self.end] = 0;

        if self.end == self.cap - 1 {
            self.end = 0;
        } else {
            self.end += 1;
        }

        true
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }

        self.data[self.end]
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }

        self.data[if self.start == 0 { self.cap - 1 } else { self.start - 1 }]
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len == self.cap
    }
}
