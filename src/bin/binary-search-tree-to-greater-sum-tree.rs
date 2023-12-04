#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

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

fn main() {}

struct Solution;

impl Solution {
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut total = 0;
        Self::dfs(root.clone(), &mut total);
        root
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, total: &mut i32) {
        if root.is_none() {
            return;
        }

        Self::dfs(root.as_ref().and_then(|x| x.borrow().right.clone()), total);

        root.as_ref().map(|x| x.borrow_mut().val += *total);
        *total = root.as_ref().map(|x| x.borrow().val).unwrap_or(0);
        Self::dfs(root.as_ref().and_then(|x| x.borrow().left.clone()), total);
    }
}
