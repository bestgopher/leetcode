#![allow(dead_code, unused, unused_variables)]

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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build(&preorder[..], &inorder[..])
    }

    fn build(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        }

        let mut root = TreeNode::new(preorder[0]);

        let mut s = 0;
        for (i, &v) in inorder.iter().enumerate() {
            if v == preorder[0] {
                s = i;
                break;
            }
        }

        root.left = Self::build(&preorder[1..1 + s], &inorder[..s]);
        root.right = Self::build(&preorder[1 + s..], &inorder[1 + s..]);

        Some(Rc::new(RefCell::new(root)))
    }
}
