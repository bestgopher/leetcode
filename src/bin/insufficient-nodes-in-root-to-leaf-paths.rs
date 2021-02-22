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
    pub fn sufficient_subset(
        root: Option<Rc<RefCell<TreeNode>>>,
        limit: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        {
            if root.as_ref().unwrap().borrow().left.is_none()
                && root.as_ref().unwrap().borrow().right.is_none()
            {
                return if root.as_ref().unwrap().borrow().val < limit {
                    None
                } else {
                    root
                };
            }
        }

        let val = root.as_ref().unwrap().borrow_mut().val;

        let left =
            Self::sufficient_subset(root.as_ref().unwrap().borrow_mut().left.take(), limit - val);
        root.as_ref().unwrap().borrow_mut().left = left;

        let right = Self::sufficient_subset(
            root.as_ref().unwrap().borrow_mut().right.take(),
            limit - val,
        );
        root.as_ref().unwrap().borrow_mut().right = right;

        {
            let r = root.as_ref().unwrap().borrow();
            if r.left.is_none() && r.right.is_none() {
                return None;
            }
        }

        root
    }
}
