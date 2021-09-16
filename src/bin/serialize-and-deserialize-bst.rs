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
        let mut v = vec![root];
        let mut high = 0u32;
        let mut index = 0usize;
        let mut has_more = true;
        let mut s = vec![];
        while has_more {
            let mut i = 1i32;
            has_more = false;
            while i <= 2i32.pow(high) {
                let mut x = v[index].take();
                if let Some(x) = x {
                    has_more = true;
                    s.push(x.borrow().val.to_string());
                    v.push(x.borrow_mut().left.take());
                    v.push(x.borrow_mut().right.take());
                } else {
                    s.push((-1).to_string());
                    v.push(None);
                    v.push(None);
                }
                i += 1;
                index += 1;
            }
            high += 1;
        }

        s.join(",")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let s = data.split(',').into_iter().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
        self.d(&s[..], 0)
    }

    fn d(&self, v: &[i32], index: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if index >= v.len() || v[index] == -1 {
            return None;
        }

        let mut node = TreeNode::new(v[index]);
        node.left = self.d(v, 2 * index + 1);
        node.right = self.d(v, 2 * index + 2);

        Some(Rc::new(RefCell::new(node)))
    }
}
