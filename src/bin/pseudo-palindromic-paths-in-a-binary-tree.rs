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
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut map = [0; 10];
        Self::dfs(root, &mut map)
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, map: &mut [u8]) -> i32 {
        if root.is_none() {
            return 0;
        }

        let root = root.unwrap();
        let root_val = root.borrow().val;
        let left = root.borrow_mut().left.take();
        let right = root.borrow_mut().right.take();

        map[(root_val - 0) as usize] += 1;

        let value = if left.is_none() && right.is_none() {
            if map.iter().filter(|x| **x % 2 == 1).count() > 1 {
                0
            } else {
                1
            }
        } else {
            Self::dfs(left, map) + Self::dfs(right, map)
        };

        map[(root_val - 0) as usize] -= 1;

        value
    }
}
