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

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if let Some(r) = root {
            Self::f(vec![Rc::clone(&r)])
        } else {
            vec![]
        }
    }

    fn f(nodes: Vec<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = vec![];


        if nodes.len() == 0 {
            return result;
        }

        let mut v = vec![];
        let mut n = vec![];

        for i in nodes.into_iter() {
            v.push(i.borrow().val);
            if let Some(left) = i.borrow_mut().left.take() {
                n.push(left);
            }

            if let Some(right) = i.borrow_mut().right.take() {
                n.push(right);
            }
        }

        for i in Self::f(n).into_iter() {
            result.push(i);
        }
        result.push(v);
        result
    }
}
