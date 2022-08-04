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
use std::ops::Index;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        let left = root.as_ref().unwrap().borrow_mut().left.take();
        let right = root.as_ref().unwrap().borrow_mut().right.take();

        let mut v = vec![left, right];
        let (mut i, mut j) = (0, 1);

        while !v.is_empty() {
            match (v.pop(), v.pop()) {
                (Some(Some(x)), Some(Some(y))) => {
                    if x.borrow().val != y.borrow().val {
                        return false;
                    }

                    v.push(x.borrow_mut().left.take());
                    v.push(y.borrow_mut().right.take());
                    v.push(x.borrow_mut().right.take());
                    v.push(y.borrow_mut().left.take());
                }
                (Some(None), Some(None)) => {}
                _ => return false,
            }
        }

        true
    }
}
