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
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        let (ans, _) = Self::f(root);
        ans
    }

    // 返回头部和尾部
    fn f(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) {
        let root = root.unwrap();
        let left = root.borrow_mut().left.take();
        let right = root.borrow_mut().right.take();

        match (left, right) {
            (Some(l), Some(r)) => {
                let (n1, m1) = Self::f(Some(l));
                let (n2, m2) = Self::f(Some(r));
                m1.clone().unwrap().borrow_mut().right = Some(root.clone());
                root.borrow_mut().right = n2;
                (n1, m2)
            }
            (Some(l), None) => {
                let (n, m) = Self::f(Some(l));
                m.clone().unwrap().borrow_mut().right = Some(root.clone());
                (n, Some(root.clone()))
            }
            (None, Some(r)) => {
                let (n, m) = Self::f(Some(r));
                root.borrow_mut().right = n;
                (Some(root), m)
            }
            (None, None) => (Some(root.clone()), Some(root.clone())),
        }
    }
}
