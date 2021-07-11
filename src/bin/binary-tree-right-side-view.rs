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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut r = vec![];
        if root.is_none() {
            return r;
        }

        struct S {
            level: i32,
            node: Option<Rc<RefCell<TreeNode>>>,
        }
        let mut v = vec![S {
            level: 0,
            node: root,
        }];
        let mut level = 0;
        let mut index = 0;

        while index < v.len() {
            if let Some(x) = v.get(index) {
                let l = x.level;
                if l == level {
                    r.push(x.node.as_ref().unwrap().borrow().val);
                    level += 1;
                }

                let right = x.node.as_ref().unwrap().borrow_mut().right.take();
                let left = x.node.as_ref().unwrap().borrow_mut().left.take();
                if right.is_some() {
                    v.push(S {
                        level: l + 1,
                        node: right,
                    });
                }

                if left.is_some() {
                    v.push(S {
                        level: l + 1,
                        node: left,
                    });
                }
            }
            index += 1;
        }

        r
    }
}
