fn main() {}

struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let v = root.as_ref().unwrap().borrow().val;
        let mut s = vec![(v, root)];
        let mut sum = 0;

        while !s.is_empty() {
            let (x, r) = s.pop().unwrap();
            let left = r.as_ref().unwrap().borrow_mut().left.take();
            let right = r.as_ref().unwrap().borrow_mut().right.take();

            if left.is_none() && right.is_none() {
                sum += x;
            }

            if left.is_some() {
                let v = left.as_ref().unwrap().borrow().val;
                s.push((x * 10 + v, left));
            }

            if right.is_some() {
                let v = right.as_ref().unwrap().borrow().val;
                s.push((x * 10 + v, right));
            }
        }

        sum
    }
}
