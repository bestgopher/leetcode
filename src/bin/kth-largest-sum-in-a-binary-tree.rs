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
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        if root.is_none() {
            return -1;
        }

        let mut results = vec![];
        let mut stack = vec![root];

        while !stack.is_empty() {
            let mut sum = 0;
            let mut new_stack = vec![];
            while let Some(x) = stack.pop() {
                let r = x.unwrap();
                sum += r.borrow().val as i64;
                let left = r.borrow_mut().left.take();
                if left.is_some() {
                    new_stack.push(left);
                }

                let right = r.borrow_mut().right.take();
                if right.is_some() {
                    new_stack.push(right);
                }
            }

            results.push(std::cmp::Reverse(sum));
            stack = new_stack;
        }

        results.sort_unstable();

        results.get(k as usize - 1).map(|x| x.0).unwrap_or(-1)
    }
}
