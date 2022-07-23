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
    pub fn merge_trees(
        t1: Option<Rc<RefCell<TreeNode>>>,
        t2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if t1.is_some() {
            Self::scan(Some(Rc::clone(t1.as_ref().unwrap())), t2);
            t1
        } else {
            t2
        }
    }

    fn scan(t1: Option<Rc<RefCell<TreeNode>>>, t2: Option<Rc<RefCell<TreeNode>>>) {
        if t1.is_none() {
            return;
        } else if t1.is_some() && t2.is_some() {
            t1.as_ref().unwrap().borrow_mut().val += t2.as_ref().unwrap().borrow().val;
            if t1.as_ref().unwrap().borrow().left.is_some() {
                Self::scan(
                    Some(Rc::clone(
                        t1.as_ref().unwrap().borrow_mut().left.as_ref().unwrap(),
                    )),
                    Rc::clone(t2.as_ref().unwrap()).borrow_mut().left.take(),
                );
            } else {
                t1.as_ref().unwrap().borrow_mut().left =
                    t2.as_ref().unwrap().borrow_mut().left.take();
            }

            if t1.as_ref().unwrap().borrow().right.is_some() {
                Self::scan(
                    Some(Rc::clone(
                        Rc::clone(t1.as_ref().unwrap())
                            .borrow_mut()
                            .right
                            .as_ref()
                            .unwrap(),
                    )),
                    Rc::clone(t2.as_ref().unwrap()).borrow_mut().right.take(),
                );
            } else {
                t1.as_ref().unwrap().borrow_mut().right =
                    t2.as_ref().unwrap().borrow_mut().right.take();
            }
        }
    }
}
