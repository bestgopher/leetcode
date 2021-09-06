use rand::Rng;

fn main() {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */
struct Solution {
    head: Option<Box<ListNode>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    /** @param head The linked list's head.
        Note that the head is guaranteed to be not null, so it contains at least one node. */
    fn new(head: Option<Box<ListNode>>) -> Self {
        Self { head }
    }

    /** Returns a random node's value. */
    fn get_random(&self) -> i32 {
        use rand::Rng;
        let mut s = &self.head;
        let mut n = 1;
        let mut r = 0;
        while let Some(x) = s {
            if rand::thread_rng().gen_range(0..n) == 0 {
                r = x.val;
            }

            s = &x.next;
            n += 1;
        }
        r
    }
}
