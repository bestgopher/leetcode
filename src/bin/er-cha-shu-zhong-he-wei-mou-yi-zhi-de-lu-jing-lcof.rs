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
use std::vec;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }

        let root = root.unwrap();
        let val = root.borrow().val;
        let left = root.borrow_mut().left.take();
        let right = root.borrow_mut().right.take();

        if val == target && left.is_none() && right.is_none() {
            return vec![vec![val]];
        }

        let v1 = Self::path_sum(left, target - val);
        let v2 = Self::path_sum(right, target - val);

        let mut data = vec![];

        for i in v1 {
            let mut v = vec![val];
            v.extend(i);
            data.push(v);
        }

        for i in v2 {
            let mut v = vec![val];
            v.extend(i);
            data.push(v);
        }

        data
    }
}
