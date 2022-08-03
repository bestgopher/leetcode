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
    pub fn is_sub_structure(
        a: Option<Rc<RefCell<TreeNode>>>,
        b: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if !(b.is_some() && a.is_some()) {
            return false;
        }

        if Self::recurse(a.clone(), b.clone()) {
            return true;
        }

        Self::is_sub_structure(a.as_ref().unwrap().borrow().left.clone(), b.clone())
            || Self::is_sub_structure(a.as_ref().unwrap().borrow().right.clone(), b.clone())
    }

    fn recurse(a: Option<Rc<RefCell<TreeNode>>>, b: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if b.is_none() {
            return true;
        }

        if a.is_none() {
            return false;
        }

        let a_value = a.as_ref().unwrap().borrow().val;
        let b_value = b.as_ref().unwrap().borrow().val;

        if a_value != b_value {
            return false;
        }

        let mut b_left = b.as_ref().unwrap().borrow().left.clone();
        let mut b_right = b.as_ref().unwrap().borrow().right.clone();

        Self::recurse(a.as_ref().unwrap().borrow().left.clone(), b_left)
            && Self::recurse(a.as_ref().unwrap().borrow().right.clone(), b_right)
    }
}
