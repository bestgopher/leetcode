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
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut root = root;
        let mut h = std::collections::HashMap::new();
        let mut max = 0;
        Self::get_sum(&mut root, &mut h, &mut max);

        h.iter()
            .filter(|&(_, y)| *y == max)
            .map(|(&x, _)| x)
            .collect()
    }

    fn get_sum(
        root: &mut Option<Rc<RefCell<TreeNode>>>,
        h: &mut std::collections::HashMap<i32, i32>,
        max: &mut i32,
    ) {
        if root.is_none() {
            return;
        }

        Self::get_sum(&mut root.as_mut().unwrap().borrow_mut().left, h, max);
        Self::get_sum(&mut root.as_mut().unwrap().borrow_mut().right, h, max);

        let sum = root
            .as_ref()
            .unwrap()
            .borrow()
            .left
            .as_ref()
            .map_or(0, |x| x.borrow().val)
            + root
                .as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .map_or(0, |x| x.borrow().val)
            + root.as_ref().unwrap().borrow().val;

        root.as_mut().unwrap().borrow_mut().val = sum;
        let count = h.entry(sum).and_modify(|x| *x += 1).or_insert(1);
        *max = (*max).max(*count)
    }
}
