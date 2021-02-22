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

use std::arch::x86_64::_mm_xor_pd;
use std::cell::RefCell;
use std::cmp::{max, min};
use std::rc::Rc;

impl Solution {
    /// 普通遍历树，时间复杂度为O(n)
    pub fn count_nodes1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let left = Self::count_nodes1(root.as_ref().unwrap().borrow_mut().left.take());
        let right = Self::count_nodes1(root.as_ref().unwrap().borrow_mut().right.take());

        1 + left + right
    }
    /// 二分法。
    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // 获取树的高度
        fn get_level(node: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
            if node.is_none() {
                return 0;
            }
            return 1 + get_level(node.unwrap().borrow().left.as_ref());
        }

        let level = get_level(root.as_ref());
        if level == 0 {
            return 0;
        } else if level == 1 {
            return 1;
        }

        let (mut min_count, mut max_count) = (2i32 << (level - 2), (2i32 << (level - 1)) - 1);

        // 查看中位的节点是否存在
        fn exists(node: Option<&Rc<RefCell<TreeNode>>>, middle: i32, level: i32) -> bool {
            if level == 1 {
                return node.is_some();
            }

            if ((1 << 31) | (middle << (33 - level))) == middle << (33 - level) {
                exists(node.unwrap().borrow().right.as_ref(), middle, level - 1)
            } else {
                exists(node.unwrap().borrow().left.as_ref(), middle, level - 1)
            }
        }

        while max_count - min_count > 1 {
            let mut middle = (min_count + max_count) / 2;

            let e = exists(root.as_ref(), middle, level);
            if e {
                min_count = middle;
            } else {
                max_count = middle - 1;
            }
        }

        if max_count - min_count == 1 && exists(root.as_ref(), max_count, level) {
            max_count
        } else {
            min_count
        }
    }
}
