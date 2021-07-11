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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        let root = root.unwrap();

        let r1 = Rc::clone(&root);
        let r2 = Rc::clone(&root);
        let (x1, t1) = Self::height(RefCell::borrow_mut(&r1).left.take());
        let (x2, t2) = Self::height(RefCell::borrow_mut(&r2).right.take());

        t1 && t2 && (x1 - x2).abs() <= 1
    }

    fn height(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) {
        if root.is_none() {
            return (0, true);
        }

        let root = root.unwrap();

        let r1 = Rc::clone(&root);
        let r2 = Rc::clone(&root);
        let (x1, t1) = Self::height(RefCell::borrow_mut(&r1).left.take());
        if !t1 {
            return (0, false);
        }

        let (x2, t2) = Self::height(RefCell::borrow_mut(&r2).right.take());
        if !t2 {
            return (0, false);
        }

        (x1.max(x2) + 1, (x1 - x2).abs() <= 1)
    }
}
