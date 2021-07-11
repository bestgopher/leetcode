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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if (p.is_none() && q.is_some()) || (p.is_some() && q.is_none()) {
            return false;
        }

        if p.is_none() && q.is_none() {
            return true;
        }

        let v1 = p.as_ref().unwrap().borrow().val;
        let v2 = q.as_ref().unwrap().borrow().val;
        if v1 != v2 {
            return false;
        }

        let p = p.unwrap();
        let q = q.unwrap();

        let p_left = p.borrow_mut().left.take();
        let p_right = p.borrow_mut().right.take();

        let q_left = q.borrow_mut().left.take();
        let q_right = q.borrow_mut().right.take();

        Self::is_same_tree(p_left, q_left) && Self::is_same_tree(p_right, q_right)
    }
}
