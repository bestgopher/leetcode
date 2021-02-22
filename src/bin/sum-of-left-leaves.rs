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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::sum(root, false)
    }

    fn sum(node: Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        if node.is_none() {
            return 0;
        }

        let node = node.unwrap();
        if is_left && node.borrow().left.is_none() && node.borrow().right.is_none() {
            return node.borrow().val;
        }

        let left = Rc::clone(&node).borrow_mut().left.take();
        let right = Rc::clone(&node).borrow_mut().right.take();

        Self::sum(left, true) + Self::sum(right, false)
    }
}
