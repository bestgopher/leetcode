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
    pub fn add_one_row(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Self::add(root, val, depth, true)
    }

    fn add(root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32, is_left: bool) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            let mut node = TreeNode::new(val);
            if is_left {
                node.left = root;
            } else {
                node.right = root;
            }
            Some(Rc::new(RefCell::new(node)))
        } else {

            if root.is_none() {
                return None;
            }

            let left = root.as_ref().unwrap().borrow_mut().left.take();
            let left = Self::add(left, val, depth - 1, true);

            let right = root.as_ref().unwrap().borrow_mut().right.take();
            let right = Self::add(right, val, depth - 1, false);

            root.as_ref().unwrap().borrow_mut().left = left;
            root.as_ref().unwrap().borrow_mut().right = right;

            root
        }
    }
}
