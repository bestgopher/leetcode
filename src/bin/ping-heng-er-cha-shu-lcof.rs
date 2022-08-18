#![allow(dead_code, unused, unused_variables, non_snake_case)]

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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::scan(root) != -1
    }

    pub fn scan(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let root = root.clone().unwrap();
        let left = root.borrow_mut().left.take();
        let right = root.borrow_mut().right.take();

        let h1 = Self::scan(left);
        if h1 == -1 {
            return -1;
        }

        let h2 = Self::scan(right);
        if h2 == -1 {
            return -1;
        }

        if (h1 - h2).abs() > 1 {
            return -1;
        }

        1 + h1.max(h2)
    }
}
