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
    /// 递归搜寻，当root为空或者root为p或者q就返回root
    /// 然后搜寻子树，如果返回的都不为空，说明root是父节点
    pub fn lowest_common_ancestor(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        mut p: Option<Rc<RefCell<TreeNode>>>,
        mut q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || root == p || root == q {
            return root;
        }

        let left = Self::lowest_common_ancestor(
            root.clone().and_then(|x| x.borrow_mut().left.take()),
            p.clone(),
            q.clone(),
        );

        let right = Self::lowest_common_ancestor(
            root.clone().and_then(|x| x.borrow_mut().right.take()),
            p.clone(),
            q.clone(),
        );

        if left.is_none() {
            right
        } else if right.is_none() {
            left
        } else {
            root
        }
    }
}
