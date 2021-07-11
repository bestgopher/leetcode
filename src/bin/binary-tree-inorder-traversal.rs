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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut r = vec![];

        Self::f(root, &mut r);

        r
    }

    fn f(root: Option<Rc<RefCell<TreeNode>>>, r: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }

        let root = root.unwrap();
        let v = root.borrow().val;
        let left = root.borrow_mut().left.take();
        if left.is_some() {
            Self::f(left, r);
        }

        r.push(v);

        let right = root.borrow_mut().right.take();
        if right.is_some() {
            Self::f(right, r);
        }
    }
}
