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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut v = vec![];

        if root.is_none() {
            return v;
        }

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let mut l = queue.len();
            let mut v1 = Vec::with_capacity(l);

            while l > 0 {
                let node = queue.pop_front().unwrap();
                let left = node.as_ref().unwrap().borrow_mut().left.take();
                let right = node.as_ref().unwrap().borrow_mut().right.take();
                if left.is_some() {
                    queue.push_back(left);
                }

                if right.is_some() {
                    queue.push_back(right);
                }
                v1.push(node.as_ref().unwrap().borrow().val);
                l -= 1;
            }

            v.push(v1);
        }

        v
    }
}
