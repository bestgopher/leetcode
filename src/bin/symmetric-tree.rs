fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        }))),
    })));

    assert!(Solution::is_symmetric(root));
}

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
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        let mut has_more = true;
        let mut v = vec![];
        v.push(root.as_ref().unwrap().borrow_mut().left.take());
        v.push(root.as_ref().unwrap().borrow_mut().right.take());

        while has_more {
            has_more = false;
            let (mut i, mut j) = (0, v.len() - 1);
            let mut v1 = vec![None; v.len() * 2];
            while i < j {
                match (v[i].as_ref(), v[j].as_ref()) {
                    (Some(x), Some(y)) => {
                        if x.borrow().val != y.borrow().val {
                            return false;
                        }
                        has_more = true;

                        v1[i * 2] = x.borrow_mut().left.take();
                        v1[i * 2 + 1] = x.borrow_mut().right.take();
                        v1[j * 2] = y.borrow_mut().left.take();
                        v1[j * 2 + 1] = y.borrow_mut().right.take();
                    }
                    (None, None) => {
                        v1[i * 2] = None;
                        v1[i * 2 + 1] = None;
                        v1[j * 2] = None;
                        v1[j * 2 + 1] = None;
                    }
                    _ => return false,
                }
                i += 1;
                j -= 1;
            }
            v = v1;
        }

        true
    }
}
