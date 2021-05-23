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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build(&inorder[..], &postorder[..])
    }

    fn build(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if postorder.len() == 0 {
            return None;
        }

        let mut root = TreeNode::new(postorder[postorder.len() - 1]);
        let mut s = postorder.len() - 1;
        for (i, &v) in inorder.iter().enumerate() {
            if v == postorder[postorder.len() - 1] {
                s = i;
                break;
            }
        }

        root.left = Self::build(&inorder[..s], &postorder[..s]);
        root.right = Self::build(&inorder[s + 1..], &postorder[s..postorder.len() - 1]);

        Some(Rc::new(RefCell::new(root)))
    }
}
