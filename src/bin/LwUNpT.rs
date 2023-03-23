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
use std::vec;

impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = vec![root];

        let mut ans = 0;

        while !stack.is_empty() {
            let mut new_stack = vec![];

            for i in 0..stack.len() {
                if i == 0 {
                    ans = stack[i].clone().as_deref().unwrap().borrow().val;
                }

                let node = stack[i].clone().unwrap();

                if node.borrow().left.is_some() {
                    new_stack.push(node.borrow_mut().left.take());
                }

                if node.borrow().right.is_some() {
                    new_stack.push(node.borrow_mut().right.take());
                }
            }

            stack = new_stack;
        }

        ans
    }
}
