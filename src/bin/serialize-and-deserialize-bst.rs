fn main() {}

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
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
struct Codec {}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    /// 当为None是序列化为-1
    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        match root {
            Some(root) => {
                let value = root.borrow().val;
                let left = root.borrow_mut().left.take();
                let right = root.borrow_mut().right.take();
                format!(
                    "{},{},{}",
                    self.serialize(left),
                    value,
                    self.serialize(right),
                )
            }
            None => "-1".to_string()
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        println!("{:?}", data);
        let s = data
            .split(',')
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();

        self.f(&s[..], &mut 0)
    }

    fn f(&self, data: &[i32], index: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
        if data.len() <= *index || data[*index] == -1 {
            return None;
        }

        let mut node = TreeNode::new(data[*index]);
        *index += 1;
        if data[*index] != -1 {
            node.left = self.f(data, index);
        } else {
            node.left = None;
        }

        *index += 1;
        if data[*index] != -1 {
            node.right = self.f(data, index);
        } else {
            node.right = None;
        }

        Some(Rc::new(RefCell::new(node)))
    }
}
