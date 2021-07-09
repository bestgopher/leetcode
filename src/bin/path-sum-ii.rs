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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        match Self::f(root, target_sum) {
            Some(x) => x,
            None => vec![]
        }
    }

    fn f(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Option<Vec<Vec<i32>>> {
        if root.is_none() {
            return None;
        }

        let value = root.as_ref().unwrap().borrow().val;
        let left = root.as_ref().unwrap().borrow_mut().left.take();
        let right = root.as_ref().unwrap().borrow_mut().right.take();

        if target_sum - value == 0 {
            if left.is_none() && right.is_none() {
                return Some(vec![vec![value]]);
            }
        }

        let mut left = match Self::f(left, target_sum - value) {
            Some(mut x) => {
                x.iter_mut().for_each(|x| x.insert(0, value));
                x
            }
            None => vec![]
        };

        let mut right = match Self::f(right, target_sum - value) {
            Some(mut x) => {
                x.iter_mut().for_each(|x| x.insert(0, value));
                x
            }
            None => vec![]
        };

        right.append(&mut left);
        if right.len() == 0 {
            None
        } else {
            Some(right)
        }
    }
}
