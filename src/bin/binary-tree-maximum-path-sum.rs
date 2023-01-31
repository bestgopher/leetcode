#![allow(dead_code, unused, unused_variables, non_snake_case)]

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
    /// 递归
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut max = None;
        Self::max(root, &mut max);

        max.unwrap_or_default()
    }

    fn max(root: Option<Rc<RefCell<TreeNode>>>, global_max: &mut Option<i32>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let root = root.unwrap();
        let current = root.borrow().val;

        let left = root.borrow_mut().left.take();
        let right = root.borrow_mut().right.take();

        let left_max = Solution::max(left, global_max);
        let right_max = Solution::max(right, global_max);

        global_max.insert_if(current + left_max.max(0) + right_max.max(0), |x, y| *x > *y);

        current + left_max.max(right_max).max(0) // 当前节点的最大和 = 当前节点+子节点的最大值(且此值必须为正数)
    }
}

trait InsertIf<T> {
    fn insert_if<F>(&mut self, value: T, f: F)
    where
        F: Fn(&T, &T) -> bool;
}

impl<T: PartialEq + Eq> InsertIf<T> for Option<T> {
    fn insert_if<F>(&mut self, value: T, f: F)
    where
        F: Fn(&T, &T) -> bool,
    {
        match self {
            Some(x) => {
                if f(&value, x) {
                    *self = Some(value);
                };
            }
            None => *self = Some(value),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::InsertIf;

    #[test]
    fn insert_if() {
        let mut n = None;

        n.insert_if(10, |x, y| *x > *y);
        assert_eq!(n, Some(10));

        n.insert_if(11, |x, y| *x > *y);
        assert_eq!(n, Some(11));

        n.insert_if(9, |x, y| *x > *y);
        assert_eq!(n, Some(11));
    }
}
