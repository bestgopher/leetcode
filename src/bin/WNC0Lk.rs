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
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = vec![];
        if root.is_some() {
            stack.push(root);
        }

        while !stack.is_empty() {
            let mut new_stack = vec![];
            ans.push(stack[stack.len() - 1].as_deref().unwrap().borrow().val);
            for i in 0..stack.len() {
                if stack[i].as_deref().unwrap().borrow().left.is_some() {
                    new_stack.push(stack[i].as_deref().unwrap().borrow_mut().left.take());
                }

                if stack[i].as_deref().unwrap().borrow().right.is_some() {
                    new_stack.push(stack[i].as_deref().unwrap().borrow_mut().right.take());
                }
            }

            stack = new_stack;
        }

        ans
    }
}
