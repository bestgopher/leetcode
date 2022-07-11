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
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }
        let v = root.as_ref().unwrap().borrow().val;

        if v > val {
            let left = root.as_ref().unwrap().borrow_mut().left.take();
            root.as_ref().unwrap().borrow_mut().left = Self::insert_into_bst(left, val);
        } else {
            let right = root.as_ref().unwrap().borrow_mut().right.take();
            root.as_ref().unwrap().borrow_mut().right = Self::insert_into_bst(right, val);
        }

        root
    }
}
