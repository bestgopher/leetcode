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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut k = k;
        Self::bst(root, &mut k).unwrap()
    }

    fn bst(root: Option<Rc<RefCell<TreeNode>>>, k: &mut i32) -> Option<i32> {
        if root.is_none() {
            return None;
        }

        let r1 = Self::bst(root.as_ref().unwrap().borrow_mut().left.take(), k);
        if r1.is_some() {
            return r1;
        }

        *k -= 1;
        if *k == 0 {
            return Some(root.as_ref().unwrap().borrow().val);
        }

        let r2 = Self::bst(root.as_ref().unwrap().borrow_mut().right.take(), k);
        if r2.is_some() {
            return r2;
        }

        None
    }
}
