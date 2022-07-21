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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::build(&preorder[..], &inorder[..])
    }

    fn build(preorder: &[i32], inorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }

        let index = inorder
            .iter()
            .enumerate()
            .map(|(x, &y)| (y, x))
            .collect::<std::collections::HashMap<i32, usize>>();

        let node_value = preorder[0];
        let node_index = index.get(&node_value).unwrap();
        let left_len = *node_index;
        let node = Rc::new(RefCell::new(TreeNode::new(node_value)));

        if left_len > 0 {
            node.borrow_mut().left =
                Self::build(&preorder[1..left_len + 1], &inorder[..left_len + 1]);
        }

        if left_len < inorder.len() {
            node.borrow_mut().right =
                Self::build(&preorder[left_len + 1..], &inorder[node_index + 1..]);
        }

        Some(node)
    }
}
