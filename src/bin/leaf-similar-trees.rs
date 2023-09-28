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
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut v1 = vec![];
        let mut v2 = vec![];
        Self::get(root1, &mut v1);
        Self::get(root2, &mut v2);

        v1 == v2
    }

    fn get(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }

        let root = root.unwrap();
        let left = root.borrow_mut().left.take();
        let right = root.borrow_mut().right.take();
        if left.is_none() && right.is_none() {
            v.push(root.borrow().val);
        }

        Self::get(left, v);
        Self::get(right, v);
    }
}
