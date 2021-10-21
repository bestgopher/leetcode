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

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut v = vec![];
        let mut stack = vec![root];

        while let Some(x) = stack.pop() {
            if let Some(y) = x {
                v.push(y.borrow().val);
                if let Some(z) = y.borrow_mut().left.take() {
                    stack.push(Some(z));
                }
                if let Some(o) = y.borrow_mut().right.take() {
                    stack.push(Some(o));
                }
            }
        }

        v.sort();

        let mut x = std::i32::MAX;

        for i in 1..v.len() {
            x = x.min(v[i] - v[i - 1]);
        }

        x
    }
}
