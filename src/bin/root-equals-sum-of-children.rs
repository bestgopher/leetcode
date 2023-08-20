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
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root = root.unwrap();
        let root_val = root.borrow().val;
        let left_val = root.borrow_mut().left.take().unwrap().borrow().val;
        let right_val = root.borrow_mut().right.take().unwrap().borrow().val;

        left_val + right_val == root_val
    }
}
