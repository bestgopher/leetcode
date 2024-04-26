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
    /// 递归，n为偶数肯定不行
    /// 假设左节点按1,3,5递增
    /// 则右节点数为n-1-1,n-1-3,n-1-5
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut v = vec![];
        if n % 2 == 0 {
            return v;
        }

        if n == 1 {
            v.push(Some(Rc::new(RefCell::new(TreeNode::new(0)))));
            return v;
        }

        for i in (1..=n - 2).step_by(2) {
            let left = Self::all_possible_fbt(i);
            let right = Self::all_possible_fbt(n - i - 1);

            for l in left.iter() {
                for r in right.iter() {
                    let mut root = Rc::new(RefCell::new(TreeNode::new(0)));
                    root.borrow_mut().left = l.clone();
                    root.borrow_mut().right = r.clone();

                    v.push(Some(root))
                }
            }
        }

        v
    }
}
