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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut result = 0;

        let mut stack = vec![root];
        while let Some(x) = stack.pop() {
            let left = x.clone().unwrap().borrow().left.clone();
            if left.is_some() {
                stack.push(left);
            }

            let right = x.clone().unwrap().borrow().right.clone();
            if right.is_some() {
                stack.push(right);
            }

            Self::dfs(x, target_sum as i64, 0, &mut result);
        }

        result
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i64, current: i64, result: &mut i32) {
        if root.is_none() {
            return;
        }

        let v = root.clone().unwrap().borrow().val as i64;
        if v + current == target_sum {
            *result += 1;
        }

        let left = root.clone().unwrap().borrow().left.clone();
        let right = root.clone().unwrap().borrow().right.clone();

        Self::dfs(left, target_sum, current + v, result);
        Self::dfs(right, target_sum, current + v, result);
    }
}
