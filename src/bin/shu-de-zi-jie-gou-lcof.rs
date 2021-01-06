fn main() {
    assert_eq!(true, Solution::is_sub_structure(
        Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            }))),
        }))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            // left: None,
            right: None,
        }))),
    ));

    assert_eq!(false, Solution::is_sub_structure(
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),

            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),

        }))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),

            right: None,
        }))))
    );

    assert_eq!(true, Solution::is_sub_structure(
        Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 8,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 9,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),

            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),

        }))),
        Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: None,
                right: None,
            }))),

            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
        }))))
    );
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

use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_sub_structure(a: Option<Rc<RefCell<TreeNode>>>, b: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if a.is_none() || b.is_none() {
            return false;
        }

        let a = a.unwrap();
        let b = b.unwrap();
        let a_value = RefCell::borrow(&Rc::clone(&a)).val;
        let b_value = RefCell::borrow(&Rc::clone(&b)).val;

        // 当值相等时，需要比较它们的子节点
        if a_value == b_value {
            if b.borrow().left.is_none() && b.borrow().right.is_none() {  // 但b的左右节点都没有时，直接返回true
                true
            } else if b.borrow().left.is_some() && b.borrow().right.is_none() {  // 当b只有左节点时，只需要与a的左节点比较
                if a.borrow().left.is_some() {
                    Self::is_sub_structure(a.borrow_mut().left.take(), b.borrow_mut().left.take())
                } else {
                    false
                }
            } else if b.borrow().left.is_none() && b.borrow().right.is_some() {  // 当b只有右节点时，只需要与a的右节点比较
                if a.borrow().right.is_some() {
                    Self::is_sub_structure(a.borrow_mut().left.take(), b.borrow_mut().left.take())
                } else {
                    false
                }
            } else {  // b的左右节点都不为空时，需要与a的左右节点比较
                if a.borrow().left.is_none() || a.borrow().right.is_none() {
                    false
                } else {
                    let mut a = a.borrow_mut();
                    let mut b = b.borrow_mut();
                    let x = Self::is_sub_structure(a.left.take(), b.left.take()) &&
                        Self::is_sub_structure(a.right.take(), b.right.take());
                    x
                }
            }
        } else {
            let mut a = a.borrow_mut();
            let x = Solution::is_sub_structure(a.left.take(), Some(Rc::clone(&b))) ||
                Solution::is_sub_structure(a.right.take(), Some(Rc::clone(&b)));
            x
        }
    }
}
