#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

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

impl Solution {
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn expand(root: Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<i32>) {
            if root.is_none() {
                return;
            }
            // 中序遍历，因为这是二叉搜索树
            let root = root.unwrap();
            expand(root.borrow_mut().left.take(), list);
            list.push(root.borrow().val);
            expand(root.borrow_mut().right.take(), list);
        }

        let mut list = vec![];
        expand(root, &mut list);

        let mut result = i32::MAX;

        for i in 1..list.len() {
            result = result.min(list[i] - list[i - 1]);
        }

        result
    }
}
