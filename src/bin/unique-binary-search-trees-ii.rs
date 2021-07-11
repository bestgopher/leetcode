fn main() {
    println!("{:?}", Solution::generate_trees(3))
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

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Self::f(1, n)
    }

    fn f(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if start > end {
            return vec![None];
        }

        let mut result = vec![];

        for i in start..=end {
            let left = Self::f(start, i - 1);
            let right = Self::f(i + 1, end);

            for j in &left {
                for k in &right {
                    let mut node = TreeNode::new(i);
                    node.left = j.clone();
                    node.right = k.clone();
                    result.push(Some(Rc::new(RefCell::new(node))));
                }
            }
        }

        result
    }
}
