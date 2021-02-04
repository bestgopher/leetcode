fn main() {
    Solution::construct_from_pre_post(vec![1, 2, 4, 5, 3, 6, 7], vec![4, 5, 2, 6, 7, 3, 1]);
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
    pub fn construct_from_pre_post(pre: Vec<i32>, post: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut v: Vec<Rc<RefCell<TreeNode>>> = vec![];

        let (mut pre_index, mut post_index) = (0usize, 0usize);

        loop {
            if post_index == post.len() {
                break;
            }

            if v.last().is_some() && v.last().unwrap().borrow().val == post[post_index] {
                if let Some(n1) = v.pop() {
                    if let Some(n2) = v.pop() {
                        if n2.as_ref().borrow().left.is_none() {
                            n2.as_ref().borrow_mut().left = Some(n1);
                        } else {
                            n2.as_ref().borrow_mut().right = Some(n1);
                        }

                        v.push(n2);
                    } else {
                        v.push(n1);
                    }
                    post_index += 1;
                }
            } else if pre[pre_index] == post[post_index] {
                let n = Some(Rc::new(RefCell::new(TreeNode::new(post[post_index]))));
                if let Some(node) = v.pop() {
                    if node.as_ref().borrow().left.is_none() {
                        node.as_ref().borrow_mut().left = n;
                    } else {
                        node.as_ref().borrow_mut().right = n;
                    }

                    v.push(node);

                } else {
                    v.push(Rc::new(RefCell::new(TreeNode::new(pre[pre_index]))));
                }
                post_index += 1;
                pre_index += 1;
            } else {
                v.push(Rc::new(RefCell::new(TreeNode::new(pre[pre_index]))));
                pre_index += 1;
            }
        }

        v.pop()
    }
}
