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
    /// 遍历二叉树
    /// 遍历的时候判断hash表中是否有k-node的值
    /// 存在则返回true
    /// 不存在就把当前node的值放入hash表中
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut hash = std::collections::HashMap::<i32, ()>::new();
        return Solution::scan_tree(root, &mut hash, k);
    }

    fn scan_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
        hash: &mut std::collections::HashMap<i32, ()>,
        k: i32,
    ) -> bool {
        if root.is_none() {
            return false;
        }

        let v = root.as_ref().unwrap().borrow().val;
        if hash.get(&(k - v)).is_some() {
            return true;
        }

        hash.insert(v, ());

        let root = root.unwrap();

        let left = Solution::scan_tree(root.borrow_mut().left.take(), hash, k);
        if left {
            return left;
        }

        let right = Solution::scan_tree(root.borrow_mut().right.take(), hash, k);
        if right {
            return right;
        }

        false
    }
}
