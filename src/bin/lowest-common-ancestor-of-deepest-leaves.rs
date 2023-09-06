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
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let (s, _) = Self::f(root);
        s
    }

    pub fn f(r: Option<Rc<RefCell<TreeNode>>>) -> (Option<Rc<RefCell<TreeNode>>>, i32) {
        if r.is_none() {
            return (r, 0);
        }
        let (s1, h1) = Self::f(r.as_ref().unwrap().borrow().left.clone());
        let (s2, h2) = Self::f(r.as_ref().unwrap().borrow().right.clone());

        if h1 == h2 {
            (r, h1 + 1)
        } else if h1 < h2 {
            (s2, h2 + 1)
        } else {
            (s1, h1 + 1)
        }
    }
}
