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

use serde::__private::de;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut depth = 0;
        let mut v = vec![root];

        while !v.is_empty() {
            let mut new_v = vec![];

            while let Some(Some(x)) = v.pop() {
                let left = x.borrow_mut().left.take();
                if left.is_some() {
                    new_v.push(left);
                }

                let right = x.borrow_mut().right.take();
                if right.is_some() {
                    new_v.push(right);
                }
            }

            depth += 1;
            v = new_v;
        }

        depth
    }
}
