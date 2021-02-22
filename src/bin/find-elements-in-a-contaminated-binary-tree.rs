fn main() {}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

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

struct FindElements {
    root: Option<Rc<RefCell<TreeNode>>>,
    v: std::collections::HashMap<i32, ()>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut v = std::collections::HashMap::new();
        let mut root = root;
        Self::scan(&mut root, &mut v, 0);
        Self { root, v }
    }

    fn scan(
        root: &mut Option<Rc<RefCell<TreeNode>>>,
        v: &mut std::collections::HashMap<i32, ()>,
        val: i32,
    ) {
        if root.is_some() {
            v.insert(val, ());
            root.as_ref().unwrap().borrow_mut().val = val;
            Self::scan(
                &mut root.as_ref().unwrap().borrow_mut().left,
                v,
                val * 2 + 1,
            );
            Self::scan(
                &mut root.as_ref().unwrap().borrow_mut().right,
                v,
                val * 2 + 2,
            );
        }
    }

    fn find(&self, target: i32) -> bool {
        self.v.get(&target).is_some()
    }
}
