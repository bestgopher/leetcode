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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut v: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        let mut r = vec![];
        let mut root = root;

        while root.is_some() {
            r.push(root.as_ref().unwrap().borrow().val);
            let left = root.as_ref().unwrap().borrow_mut().left.take();
            let right = root.as_ref().unwrap().borrow_mut().right.take();

            if left.is_some() {
                root = left;
                if right.is_some() {
                    v.push(right);
                }
            } else if right.is_some() {
                root = right;
            } else {
                root = v.pop().unwrap_or(None);
            }
        }

        r
    }
}
