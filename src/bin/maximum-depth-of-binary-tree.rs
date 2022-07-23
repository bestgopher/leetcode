#![allow(dead_code, unused, unused_variables)]

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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let root = root.unwrap();

        let left = 1 + Self::max_depth(Rc::clone(&root).borrow_mut().left.take());
        let right = 1 + Self::max_depth(Rc::clone(&root).borrow_mut().right.take());

        if left > right {
            left
        } else {
            right
        }
    }
}
