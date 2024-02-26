#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

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

impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut result = 0;

        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, result: &mut i32, low: i32, high: i32) {
            if root.is_none() {
                return;
            }

            let root = root.unwrap();
            dfs(root.borrow_mut().left.take(), result, low, high);
            dfs(root.borrow_mut().right.take(), result, low, high);

            if root.borrow().val <= high && root.borrow().val >= low {
                *result += root.borrow().val;
            }
        }

        dfs(root, &mut result, low, high);

        result
    }
}
