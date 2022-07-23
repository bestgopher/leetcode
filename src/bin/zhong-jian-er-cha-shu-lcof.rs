#![allow(dead_code, unused, unused_variables)]

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!(
        "{:?}",
        Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7])
    );
    println!("{:?}", Solution::build_tree(vec![1, 2], vec![1, 2]));
}

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
        if preorder.is_empty() {
            return None;
        }

        let index = inorder
            .iter()
            .enumerate()
            .map(|(x, &y)| (y, x))
            .collect::<std::collections::HashMap<i32, usize>>();

        Self::build(
            &preorder,
            &inorder,
            (0, preorder.len() - 1),
            (0, inorder.len() - 1),
            &index,
        )
    }

    fn build(
        preorder: &Vec<i32>,
        inorder: &Vec<i32>,
        preorder_index: (usize, usize),
        inorder_index: (usize, usize),
        index: &std::collections::HashMap<i32, usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder_index.0 > preorder_index.1 {
            return None;
        }

        let root_value = preorder[preorder_index.0];
        let &root_index = index.get(&root_value).unwrap();
        let left_len = root_index - inorder_index.0;
        let node = Rc::new(RefCell::new(TreeNode::new(root_value)));

        if root_index > 0 {
            node.borrow_mut().left = Self::build(
                preorder,
                inorder,
                (preorder_index.0 + 1, (preorder_index.0 + left_len)),
                (inorder_index.0, (root_index - 1)),
                index,
            );
        }

        node.borrow_mut().right = Self::build(
            preorder,
            inorder,
            (preorder_index.0 + left_len + 1, preorder_index.1),
            (root_index + 1, inorder_index.1),
            index,
        );

        Some(node)
    }
}
