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
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let s = Self::f(root);
        s.into_iter().map(|x| x.into_iter().map(|y| y.to_string()).collect::<Vec<String>>().join("->")).collect()
    }

    fn f(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }

        let left = root.as_ref().unwrap().borrow_mut().left.take();
        let right = root.as_ref().unwrap().borrow_mut().right.take();
        let v = root.as_ref().unwrap().borrow().val;
        match (left, right) {
            (Some(x), Some(y)) => {
                let f = Self::f(Some(x));
                let r = Self::f(Some(y));
                f.into_iter().chain(r.into_iter()).map(|mut x| {
                    x.insert(0, v);
                    x
                }).collect::<Vec<Vec<i32>>>()
            }
            (Some(x), None) => {
                let f = Self::f(Some(x));
                f.into_iter().map(|mut x| {
                    x.insert(0, v);
                    x
                }).collect::<Vec<Vec<i32>>>()
            }
            (None, Some(y)) => {
                let f = Self::f(Some(y));
                f.into_iter().map(|mut x| {
                    x.insert(0, v);
                    x
                }).collect::<Vec<Vec<i32>>>()
            }
            (None, None) => vec![vec![v]]
        }
    }
}
