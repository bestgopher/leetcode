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
    /// 1.搜索二叉树
    /// 2.节点值唯一
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }

        let val = root.as_ref().unwrap().borrow().val;
        let left = p.as_ref().unwrap().borrow().val;
        let right = q.as_ref().unwrap().borrow().val;

        if (left <= val && right >= val) || (right <= val && left >= val) {
            root
        } else if left < val && right < val {
            Self::lowest_common_ancestor(root.as_ref().unwrap().borrow_mut().left.take(), p, q)
        } else {
            Self::lowest_common_ancestor(root.as_ref().unwrap().borrow_mut().right.take(), p, q)
        }
    }
}
