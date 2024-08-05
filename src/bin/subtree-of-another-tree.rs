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
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root.is_none() || sub_root.is_none() {
            return root == sub_root;
        }

        Self::check(root.clone(), sub_root.clone())
            || Self::is_subtree(
                root.clone().and_then(|x| x.borrow().left.clone()),
                sub_root.clone(),
            )
            || Self::is_subtree(
                root.clone().and_then(|x| x.borrow().right.clone()),
                sub_root.clone(),
            )
    }

    pub fn check(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root.is_none() && sub_root.is_none() {
            return true;
        } else if root.is_none() && sub_root.is_some() {
            return false;
        } else if root.is_some() && sub_root.is_none() {
            return false;
        }

        let v1 = root.clone().unwrap().borrow().val;
        let v2 = sub_root.clone().unwrap().borrow().val;

        if v1 != v2 {
            return false;
        }

        Self::check(
            root.clone().and_then(|x| x.borrow().left.clone()),
            sub_root.clone().and_then(|x| x.borrow().left.clone()),
        ) && Self::check(
            root.clone().and_then(|x| x.borrow().right.clone()),
            sub_root.clone().and_then(|x| x.borrow().right.clone()),
        )
    }
}
