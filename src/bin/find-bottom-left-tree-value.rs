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
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = vec![root];
        let mut result = 0;
        while !stack.is_empty() {
            let mut new_stack = vec![];
            let mut flag = false;
            for node in stack.into_iter() {
                if let Some(x) = node {
                    if !flag {
                        result = x.borrow().val;
                        flag = true;
                    }
                    new_stack.push(x.borrow_mut().left.take());
                    new_stack.push(x.borrow_mut().right.take());
                }
            }

            stack = new_stack;
            flag = false;
        }

        result
    }
}
