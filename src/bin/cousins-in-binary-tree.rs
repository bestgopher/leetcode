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
    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let mut stack = vec![root];
        while !stack.is_empty() {
            let mut new_stack = vec![];
            let mut exists = false;
            while let Some(Some(p)) = stack.pop() {
                let left = p.borrow_mut().left.take();
                let right = p.borrow_mut().right.take();
                let mut e = false;
                if left.is_some() {
                    let v = left.as_ref().unwrap().borrow().val;
                    if v == x || v == y {
                        if exists {
                            return true;
                        }
                        exists = true;
                        e = true;
                    }

                    new_stack.push(left);
                }

                if right.is_some() {
                    let v = right.as_ref().unwrap().borrow().val;
                    if v == x || v == y {
                        if e {
                            return false;
                        }

                        if exists {
                            return true;
                        }

                        exists = true;
                    }

                    new_stack.push(right);
                }
            }

            stack = new_stack;
        }

        false
    }
}
