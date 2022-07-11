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
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = root;
        while let Some(r) = root {
            let v = r.borrow().val;
            match v.cmp(&val) {
                std::cmp::Ordering::Equal => return Some(r),
                std::cmp::Ordering::Greater => root = r.borrow_mut().left.take(),
                std::cmp::Ordering::Less => root = r.borrow_mut().right.take(),
            }
        }

        None
    }
}
