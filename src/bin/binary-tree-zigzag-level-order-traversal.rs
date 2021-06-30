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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut v = vec![];
        if root.is_none() {
            return v;
        }

        let mut l = vec![root];
        let mut level = 0;

        while !l.is_empty() {
            let mut r = Vec::new();
            let mut l1 = Vec::new();
            while let Some(x) = l.pop() {
                r.push(x.as_ref().unwrap().borrow().val);
                let left = x.as_ref().unwrap().borrow_mut().left.take();
                let right = x.as_ref().unwrap().borrow_mut().right.take();

                if level % 2 == 0 {
                    if left.is_some() {
                        l1.push(left);
                    }

                    if right.is_some() {
                        l1.push(right);
                    }
                } else {
                    if right.is_some() {
                        l1.push(right);
                    }

                    if left.is_some() {
                        l1.push(left);
                    }
                }
            }

            v.push(r);
            l = l1;
            level += 1;
        }

        v
    }
}
