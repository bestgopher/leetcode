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
    /// 先求出总和
    /// 因为中序遍历是按大小遍历的，所以记录下前缀和，则当前节点的
    pub fn convert_bst1(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let mut total = 0;
        let mut stack = vec![root.clone()];
        while !stack.is_empty() {
            let s = stack.pop().unwrap();
            total += s.clone().unwrap().borrow().val;
            if s.clone().unwrap().borrow().left.is_some() {
                stack.push(s.clone().unwrap().borrow().left.clone());
            }

            if s.clone().unwrap().borrow().right.is_some() {
                stack.push(s.clone().unwrap().borrow().right.clone());
            }
        }

        Self::retrive1(root.clone(), total);

        root
    }

    /// 输入前缀和
    /// 当前节点的和 = total - left节点的和
    fn retrive1(root: Option<Rc<RefCell<TreeNode>>>, total: i32) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut prefix_sum = 0;
        // 左节点
        prefix_sum += Self::retrive1(root.clone().unwrap().borrow().left.clone(), total);

        let val = root.clone().unwrap().borrow().val;

        // 当前root的和为total - 左节点的前缀和
        root.clone().unwrap().borrow_mut().val = total - prefix_sum;
        prefix_sum += val;

        // 右节点
        prefix_sum += Self::retrive1(
            root.clone().unwrap().borrow().right.clone(),
            total - prefix_sum,
        );

        prefix_sum
    }

    /// 倒叙中序遍历, 右，中，左的顺序遍历
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sum = 0;

        Self::retive(root.clone(), &mut sum);
        root
    }

    pub fn retive(root: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
        if root.is_none() {
            return;
        }

        Self::retive(root.clone().unwrap().borrow().right.clone(), sum);

        *sum += root.clone().unwrap().borrow().val;
        root.clone().unwrap().borrow_mut().val = *sum;

        Self::retive(root.clone().unwrap().borrow().left.clone(), sum);
    }
}
