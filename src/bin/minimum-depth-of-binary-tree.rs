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
    pub fn min_depth1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let root = root.unwrap();

        if root.borrow().left.is_none() && root.borrow().right.is_none() {
            1
        } else if root.borrow().left.is_none() && root.borrow().right.is_some() {
            1 + Solution::min_depth(RefCell::borrow_mut(&root).right.take())
        } else if root.borrow().left.is_some() && root.borrow().right.is_none() {
            1 + Solution::min_depth(RefCell::borrow_mut(&root).left.take())
        } else {
            let left = 1 + Solution::min_depth(RefCell::borrow_mut(&root).left.take());
            let right = 1 + Solution::min_depth(RefCell::borrow_mut(&root).right.take());

            if left > right {
                return right;
            }

            left
        }
    }

    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut height = 1;
        let mut v = vec![(root, height)];

        let mut index = 0;

        while index < v.len() {
            if v[index].0.as_ref().unwrap().borrow().left.is_none()
                && v[index].0.as_ref().unwrap().borrow().right.is_none()
            {
                height = v[index].1;
                break;
            }

            if v[index].0.as_ref().unwrap().borrow().left.is_some() {
                let x = v[index].1 + 1;
                let left = v[index].0.as_ref().unwrap().borrow_mut().left.take();
                v.push((left, x));
            }

            if v[index].0.as_ref().unwrap().borrow().right.is_some() {
                let x = v[index].1 + 1;
                let right = v[index].0.as_ref().unwrap().borrow_mut().right.take();
                v.push((right, x));
            }

            index += 1;
        }

        height
    }
}
