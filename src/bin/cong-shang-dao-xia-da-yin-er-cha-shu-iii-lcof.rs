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

        let mut stack = vec![root.unwrap()];
        let mut data = vec![];
        let mut level = 1;

        while !stack.is_empty() {
            let mut new_stack = vec![];
            let mut d = vec![];

            while let Some(x) = stack.pop() {
                d.push(x.borrow().val);
                if level % 2 == 1 {
                    x.borrow_mut().left.take().map(|x| new_stack.push(x));
                    x.borrow_mut().right.take().map(|x| new_stack.push(x));
                } else {
                    x.borrow_mut().right.take().map(|x| new_stack.push(x));
                    x.borrow_mut().left.take().map(|x| new_stack.push(x));
                }
            }

            level += 1;

            data.push(d);
            stack = new_stack;
        }

        data
    }
}
