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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        Self::f(&mut sum, 0, root);
        sum
    }

    fn f(sum: &mut i32, val: i32, root: Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }

        let v = root.clone().unwrap().borrow().val;
        let left = root.clone().unwrap().borrow_mut().left.take();
        let right = root.clone().unwrap().borrow_mut().right.take();

        if left.is_none() && right.is_none() {
            *sum += v + val * 10;
            return;
        }

        if left.is_some() {
            Self::f(sum, v + val * 10, left);
        }

        if right.is_some() {
            Self::f(sum, v + val * 10, right);
        }
    }
}
