#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

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

fn main() {}

struct Solution;

impl Solution {
    pub fn closest_nodes(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<Vec<i32>> {
        let mut items = vec![];
        Self::dfs(root, &mut items);

        let mut result = vec![];
        for q in queries {
            let j = items.partition_point(|x| *x < q);
            let mx = if j < items.len() { items[j] } else { -1 };
            let mn = if j < items.len() && items[j] == q {
                q
            } else if j > 0 {
                items[j - 1]
            } else {
                -1
            };
            result.push(vec![mn, mx]);
        }

        result
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, items: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        let root = root.unwrap();
        Self::dfs(root.borrow_mut().left.take(), items);
        items.push(root.borrow().val);
        Self::dfs(root.borrow_mut().right.take(), items);
    }
}
