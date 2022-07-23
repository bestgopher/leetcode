#![allow(dead_code, unused, unused_variables)]

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
    pub fn kth_largest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        if k == 0 {
            return root.unwrap().borrow().val;
        }
        let mut v = Vec::<i32>::new();
        Self::scan(root, &mut v, k as usize);

        v[k as usize - 1]
    }

    fn scan(root: Option<Rc<RefCell<TreeNode>>>, val: &mut Vec<i32>, k: usize) {
        let root = root.unwrap();
        if RefCell::borrow(&root).right.is_some() {
            let right = root.borrow_mut().right.take();
            Self::scan(right, val, k);
        }

        val.push(root.borrow().val);
        if val.len() == k {
            return;
        }

        if RefCell::borrow(&root).left.is_some() {
            let right = root.borrow_mut().left.take();
            Self::scan(right, val, k);
        }
    }
}
