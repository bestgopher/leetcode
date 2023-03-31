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
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut hash = std::collections::HashMap::new();
        Self::f(root, &mut hash, k)
    }

    pub fn f(
        root: Option<Rc<RefCell<TreeNode>>>,
        hash: &mut std::collections::HashMap<i32, ()>,
        k: i32,
    ) -> bool {
        if root.is_none() {
            return false;
        }
        let val = root.clone().unwrap().borrow().val;
        if hash.contains_key(&(k - val)) {
            return true;
        }

        hash.insert(val, ());
        let left = root.clone().unwrap().borrow().left.clone();
        let right = root.clone().unwrap().borrow().right.clone();

        Self::f(left, hash, k) || Self::f(right, hash, k)
    }
}
