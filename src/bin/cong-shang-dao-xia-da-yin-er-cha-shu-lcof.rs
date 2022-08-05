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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v = vec![];
        let mut stack = vec![root];
        let mut index = 0;

        while index < stack.len() {
            if let Some(ref x) = stack[index].clone() {
                v.push(x.borrow().val);

                stack.push(x.borrow_mut().left.take());

                stack.push(x.borrow_mut().right.take());
            }

            index += 1;
        }

        v
    }
}
