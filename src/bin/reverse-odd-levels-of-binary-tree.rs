#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

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

fn main() {}

struct Solution;

impl Solution {
    pub fn reverse_odd_levels(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut edage = 0;
        let mut stack = vec![root.clone()];

        while !stack.is_empty() {
            if edage % 2 == 1 {
                let (mut start, mut end) = (0, stack.len() - 1);
                while start < end {
                    let l = stack.get(start).unwrap().as_ref().unwrap().borrow().val;
                    let r = stack.get(end).unwrap().as_ref().unwrap().borrow().val;

                    stack.get(start).unwrap().as_ref().unwrap().borrow_mut().val = r;
                    stack.get(end).unwrap().as_ref().unwrap().borrow_mut().val = l;

                    start += 1;
                    end -= 1;
                }
            }

            let mut new_stack = Vec::with_capacity(stack.len() * 2);
            while let Some(x) = stack.pop() {
                let left = x.as_ref().unwrap().borrow().left.clone();
                if left.is_some() {
                    new_stack.push(left);
                }
                let right = x.as_ref().unwrap().borrow().right.clone();
                if right.is_some() {
                    new_stack.push(right);
                }
            }

            stack = new_stack;

            edage += 1;
        }

        root
    }
}
