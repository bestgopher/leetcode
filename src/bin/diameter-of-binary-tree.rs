#![allow(dead_code, unused, unused_variables, non_snake_case)]

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
    /// 左边的最深度 + 右边的最深度
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (_, r) = Self::height(root);
        r
    }

    /// 返回深度和对应节点上的最大直径
    pub fn height(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if root.is_none() {
            return (0, 0);
        }

        let (left, max_left) = Self::height(root.clone().unwrap().borrow().left.clone());
        let (right, max_right) = Self::height(root.clone().unwrap().borrow().right.clone());

        (
            left.max(right) + 1,
            (left + right).max(max_left).max(max_right),
        )
    }
}
