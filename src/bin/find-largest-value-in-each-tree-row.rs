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
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut stack = vec![root];
        let mut result = vec![];
        while !stack.is_empty() {
            let mut new = vec![];
            let mut r: Option<i32> = None;
            while let Some(node) = stack.pop() {
                if node.is_none() {
                    continue;
                }
                let v = node.as_ref().unwrap().borrow().val;
                r = r.map_or(Option::from(v), |x| Option::from(x.max(v)));
                if let Some(x) = node.as_ref().unwrap().borrow_mut().left.take() {
                    new.push(Some(x));
                }

                if let Some(x) = node.as_ref().unwrap().borrow_mut().right.take() {
                    new.push(Some(x));
                }
            }
            if r.is_some() {
                result.push(r.unwrap());
            }
            stack = new;
        }

        result
    }
}
