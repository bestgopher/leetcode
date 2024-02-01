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
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root = root.unwrap();
        let root_val = root.borrow().val;
        let left = root.borrow_mut().left.take();
        let right = root.borrow_mut().right.take();
        Self::dfs(left, root_val, root_val).max(Self::dfs(right, root_val, root_val))
    }

    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, max_parent: i32, min_parent: i32) -> i32 {
        if root.is_none() {
            return 0;
        }

        let root = root.unwrap();
        let v = (max_parent - root.borrow().val)
            .abs()
            .max((min_parent - root.borrow().val).abs());

        let max_parent = max_parent.max(root.borrow().val);
        let min_parent = min_parent.min(root.borrow().val);

        let left = root.borrow_mut().left.take();
        let right = root.borrow_mut().right.take();

        v.max(Self::dfs(left, max_parent, min_parent).max(Self::dfs(right, max_parent, min_parent)))
    }
}
