#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::{cell::RefCell, rc::Rc};

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
struct CBTInserter {
    count: i32,
    root: Option<Rc<RefCell<TreeNode>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CBTInserter {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut count = 0;
        let mut stack = vec![];
        if root.is_some() {
            stack.push(root.clone().unwrap());
        }
        while !stack.is_empty() {
            let mut new_stack = vec![];
            while let Some(s) = stack.pop() {
                if s.borrow().left.is_some() {
                    new_stack.push(s.borrow().left.clone().unwrap());
                }

                if s.borrow().right.is_some() {
                    new_stack.push(s.borrow().right.clone().unwrap());
                }
                count += 1;
            }

            stack = new_stack;
        }

        Self { root, count }
    }

    fn insert(&mut self, v: i32) -> i32 {
        self.count += 1;

        let mut node = self.root.clone();

        for i in (1..=self.get_bits_len() - 2).rev() {
            if self.count >> i & 1 == 0 {
                node = node.map(|x| x.borrow().left.clone().unwrap());
            } else {
                node = node.map(|x| x.borrow().right.clone().unwrap());
            }
        }
        let ans = node.as_deref().unwrap().borrow().val;

        if self.count & 1 == 0 {
            node.unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
        } else {
            node.unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
        }

        ans
    }

    fn get_root(&self) -> Option<Rc<RefCell<TreeNode>>> {
        self.root.clone()
    }

    fn get_bits_len(&self) -> i32 {
        let mut ans = 0;
        let mut c = self.count;
        while c > 0 {
            ans += 1;
            c >>= 1;
        }

        ans
    }
}
