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

/**
 * Your BSTIterator object will be instantiated and called as such:
 * let obj = BSTIterator::new(root);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
struct BSTIterator {
    stack: Vec::<Option<Rc<RefCell<TreeNode>>>>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut s = Self { stack: vec![] };
        if root.is_some() {
            s.stack.push(root);
        }
        s
    }

    fn next(&mut self) -> i32 {
        let mut node = self.stack.pop().unwrap();

        while node.is_some() {
            let s = node;
            let left = s.as_ref().unwrap().borrow_mut().left.take();
            let right = s.as_ref().unwrap().borrow_mut().right.take();

            if right.is_some() {
                self.stack.push(right);
            }

            self.stack.push(s);
            node = left;
        }

        let r = self.stack.pop().unwrap().as_ref().unwrap().borrow().val;
        r
    }

    fn has_next(&self) -> bool {
        self.stack.len() > 0
    }
}
