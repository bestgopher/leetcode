#![allow(dead_code, unused, unused_variables)]

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

struct Solution;

impl Solution {
    // 递归
    pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(r) => {
                let value = r.borrow().val;

                let left_result = match &r.borrow().left {
                    Some(left) => {
                        if left.borrow().val != value {
                            false
                        } else {
                            Solution::is_unival_tree(Some(Rc::clone(left)))
                        }
                    }
                    None => true,
                };

                let right_result = match &r.borrow().right {
                    Some(right) => {
                        if right.borrow().val != value {
                            false
                        } else {
                            Solution::is_unival_tree(Some(Rc::clone(right)))
                        }
                    }
                    None => true,
                };

                left_result && right_result
            }
        }
    }
}

fn main() {}
