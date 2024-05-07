#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

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

impl Solution {
    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let r1 = Self::get_list(root1);
        let r2 = Self::get_list(root2);

        let mut result = Vec::with_capacity(r1.len() + r2.len());
        let (mut i1, mut i2) = (0, 0);
        while i1 < r1.len() || i2 < r2.len() {
            match (r1.get(i1), r2.get(i2)) {
                (Some(&x), Some(&y)) => {
                    if x <= y {
                        result.push(x);
                        i1 += 1;
                    } else {
                        result.push(y);
                        i2 += 1;
                    }
                }

                (Some(&x), _) => {
                    result.push(x);
                    i1 += 1;
                }

                (_, Some(&y)) => {
                    result.push(y);
                    i2 += 1;
                }
                _ => unreachable!(),
            }
        }

        result
    }

    fn get_list(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn f(root: Option<Rc<RefCell<TreeNode>>>, r: &mut Vec<i32>) {
            if root.is_none() {
                return;
            }

            let root = root.unwrap();
            f(root.borrow_mut().left.take(), r);
            r.push(root.borrow().val);
            f(root.borrow_mut().right.take(), r);
        }

        let mut result = vec![];
        f(root, &mut result);

        result
    }
}
