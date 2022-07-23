#![allow(dead_code, unused, unused_variables)]

fn main() {
    let v = vec![1, 23, 2, 25, 12, 6];
    Solution::bst_from_preorder(v);
}

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

struct Solution;

impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(preorder[0])));

        for &i in preorder[1..].iter() {
            let mut n = Rc::clone(&root);
            loop {
                if n.borrow().val > i {
                    if let Some(s) = &n.clone().borrow().left {
                        n = Rc::clone(s);
                        continue;
                    }
                    n.borrow_mut()
                        .left
                        .replace(Rc::new(RefCell::new(TreeNode::new(i))));
                    break;
                } else {
                    if let Some(s) = n.clone().borrow().right.as_ref() {
                        n = Rc::clone(s);
                        continue;
                    }
                    n.borrow_mut()
                        .right
                        .replace(Rc::new(RefCell::new(TreeNode::new(i))));
                    break;
                }
            }
        }
        Some(root)
    }
}
