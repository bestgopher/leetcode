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
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        if root.is_none() {
            return vec![];
        }

        let mut result = vec![];
        let mut stack = vec![root];

        while !stack.is_empty() {
            let mut s = vec![];
            let mut l = stack.len();
            let mut sum = 0;

            while let Some(v) = stack.pop() {
                if v.is_none() {
                    continue;
                }

                let v = v.unwrap();
                sum += v.borrow().val as i64;
                let left = v.borrow_mut().left.take();
                if left.is_some() {
                    s.push(left);
                }

                let right = v.borrow_mut().right.take();
                if right.is_some() {
                    s.push(right);
                }
            }

            result.push(sum as f64 / l as f64);
            stack = s;
        }

        result
    }
}
