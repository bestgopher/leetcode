#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::cell::RefCell;
use std::rc::Rc;

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

impl Solution {
    /// hashmap记录与节点所有相连的值
    /// 然后在dfs这个hashmap
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        let mut hash = std::collections::HashMap::<i32, Vec<i32>>::new();

        Self::dfs1(&mut hash, root, None);
        Self::dfs2(&hash, start, -1)
    }

    pub fn dfs1(
        hash: &mut std::collections::HashMap<i32, Vec<i32>>,
        root: Option<Rc<RefCell<TreeNode>>>,
        parent: Option<i32>,
    ) {
        if root.is_none() {
            return;
        }

        let root = root.unwrap();
        let value = root.borrow().val;

        if let Some(i) = parent {
            hash.entry(value).or_insert(vec![]).push(i);
            hash.entry(i).or_insert(vec![]).push(value);
        }

        Self::dfs1(hash, root.borrow_mut().left.take(), Some(value));
        Self::dfs1(hash, root.borrow_mut().right.take(), Some(value));
    }

    pub fn dfs2(hash: &std::collections::HashMap<i32, Vec<i32>>, start: i32, last: i32) -> i32 {
        match hash.get(&start) {
            Some(v) => {
                let mut r = 0;
                for &i in v {
                    if i == last {
                        continue;
                    }
                    r = r.max(1 + Self::dfs2(hash, i, start));
                }

                r
            }
            None => 0,
        }
    }
}
