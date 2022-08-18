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
        let (result, _) = Self::scan(root);
        result
    }

    pub fn scan(root: Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
        if root.is_none() {
            return (true, 0);
        }
        let root = root.clone().unwrap();
        let left = root.borrow_mut().left.take();
        let right = root.borrow_mut().right.take();

        let (r1, h1) = Self::scan(left);
        if !r1 {
            return (false, 0);
        }

        let (r2, h2) = Self::scan(right);
        if !r2 {
            return (false, 0);
        }

        if (h1 - h2).abs() > 1 {
            return (false, 0);
        }

        (true, 1 + h1.max(h2))
    }
}
