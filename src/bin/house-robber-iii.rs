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
    // 每个节点都有偷或者不偷两种情况
    pub fn force_rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::r(root.clone(), true).max(Self::r(root.clone(), false))
    }

    pub fn r(root: Option<Rc<RefCell<TreeNode>>>, steal: bool) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut r = 0;
        // 如果当前节点可以偷，那么下一个节点就不能偷
        let r1 = if steal {
            let mut r1 = root.clone().unwrap().borrow().val;
            r1 += Self::r(root.clone().unwrap().borrow().left.clone(), false);
            r1 += Self::r(root.clone().unwrap().borrow().right.clone(), false);
            r1
        } else {
            // 如果当前节点不能偷，那么下一个节点可偷可不偷
            let mut r1 = 0;
            r1 += Self::r(root.clone().unwrap().borrow().left.clone(), true)
                .max(Self::r(root.clone().unwrap().borrow().left.clone(), false));
            r1 += Self::r(root.clone().unwrap().borrow().right.clone(), true)
                .max(Self::r(root.clone().unwrap().borrow().right.clone(), false));
            r1
        };

        r1
    }

    /// 每个节点都有偷或者不偷两种情况
    /// 先计算高度，然后每个节点都有偷或者不偷两种情况，因此
    /// i节点的最大值 = max(i节点可偷值 + i的子节点不偷的情况下的值, i节点不可偷+i的左子节点可偷+i右子节点可偷,  i节点不可偷+i的左子节点不可偷+i右子节点不可偷,  i节点不可偷+i的左子节点可偷+i右子节点不可偷,  i节点不可偷+i的左子节点不可偷+i右子节点可偷）
    /// 因此定义
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let r = Self::r1(root);

        r.0.max(r.1)
    }

    /// 设ans为返回值
    /// ans.0表示选择此节点的值
    /// ans.1表示不选择此节点的值
    /// 则：
    /// ans.0 = l.1 + r.1 + node.val
    /// ans.1 = l.0.max(l.1) + r.0.max(r.1)
    fn r1(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if root.is_none() {
            return (0, 0);
        }

        let l = Self::r1(root.clone().unwrap().borrow().left.clone());
        let r = Self::r1(root.clone().unwrap().borrow().right.clone());

        (
            l.1 + r.1 + root.unwrap().borrow().val,
            l.0.max(l.1) + r.0.max(r.1),
        )
    }
}
