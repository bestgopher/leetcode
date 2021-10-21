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

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() { return std::i32::MAX; }
        let v = root.as_ref().unwrap().borrow().val;
        let left = root.as_ref().unwrap().borrow_mut().left.take();
        let right = root.as_ref().unwrap().borrow_mut().right.take();

        let x = (left.as_ref().map_or(std::i32::MAX, |x| x.borrow().val) - v).abs();
        let x = x.min((right.as_ref().map_or(std::i32::MAX, |x| x.borrow().val) - v).abs());
        x.min(Self::get_minimum_difference(left)).min(Self::get_minimum_difference(right))
    }
}
