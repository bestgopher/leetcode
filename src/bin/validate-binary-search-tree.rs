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
    /// 中序遍历，遍历值都是递增的，当当前值小于或者等于前一个值，说明不是搜索二叉树
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // 获取最小的值的节点，也就是最左节点
        let mut v = None;
        Self::scan(root, &mut v)
    }

    fn scan(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Option<i32>) -> bool {
        if root.is_none() {
            return true;
        }
        let value = root.as_ref().unwrap().borrow().val;
        let left = root.as_ref().unwrap().borrow_mut().left.take();
        let right = root.as_ref().unwrap().borrow_mut().right.take();

        if left.is_some() {
            if !Self::scan(left, v) {
                return false;
            }
        }

        if v.is_some() && value <= v.unwrap() {
            return false;
        }

        *v = Some(value);

        if right.is_some() {
            if !Self::scan(right, v) {
                return false;
            }
        }

        true
    }
}
