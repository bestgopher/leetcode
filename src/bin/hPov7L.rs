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
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = vec![];
        let mut ans = vec![];

        if root.is_some() {
            stack.push(root);
        }

        while !stack.is_empty() {
            let mut new_stack = vec![];
            let mut max = 0;

            for (i, node) in stack.into_iter().enumerate() {
                let node = node.unwrap();
                if node.borrow().left.is_some() {
                    new_stack.push(node.borrow_mut().left.take());
                }

                if node.borrow().right.is_some() {
                    new_stack.push(node.borrow_mut().right.take());
                }

                if i == 0 {
                    max = node.borrow().val;
                } else {
                    max = max.max(node.borrow().val);
                }
            }

            stack = new_stack;
            ans.push(max);
        }

        ans
    }
}
