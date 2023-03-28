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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut ans = 0;
        ans += Self::travel(root.clone(), target_sum as i64);
        if root.is_some() {
            ans += Self::path_sum(root.clone().unwrap().borrow().left.clone(), target_sum);
            ans += Self::path_sum(root.clone().unwrap().borrow().right.clone(), target_sum);
        }

        ans
    }

    pub fn travel(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i64) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut ans = 0;
        let root = root.unwrap();
        let val = root.borrow().val as i64;
        if target_sum == val {
            ans += 1;
        }

        let left = root.borrow().left.clone();
        let right = root.borrow().right.clone();

        ans + Self::travel(left, target_sum - val) + Self::travel(right, target_sum - val)
    }
}
