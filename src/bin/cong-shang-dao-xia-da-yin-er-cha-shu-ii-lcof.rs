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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }

        let mut data = vec![];
        let mut stack = vec![root];

        while !stack.is_empty() {
            let mut r = vec![];
            let mut new_stack = vec![];
            let mut index = 0;

            while index < stack.len() {
                if let Some(x) = stack[index].clone() {
                    r.push(x.borrow().val);

                    let left = x.borrow_mut().left.take();
                    if left.is_some() {
                        new_stack.push(left);
                    }

                    let right = x.borrow_mut().right.take();
                    if right.is_some() {
                        new_stack.push(right);
                    }
                }
                index += 1;
            }

            stack = new_stack;
            data.push(r);
        }

        data
    }
}
