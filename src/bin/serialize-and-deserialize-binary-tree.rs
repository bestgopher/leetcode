#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    let mut c = Codec::new();
    println!("{:?}", c.deserialize(r#"{"val": 1, "left": {"val": 2, "left": null, "right": null}, "right": {"val": 3, "left": {"val": 4, "left": null, "right": null}, "right": {"val": 5, "left": null, "right": null}}}"#.into()));
    println!("{:?}", c.deserialize(r#"{"val": 4, "left": {"val": -7, "left": null, "right": null}, "right": {"val": -3, "left": {"val": -9, "left": {"val": 9, "left": {"val": 6, "left": {"val": 0, "left": null, "right": {"val": -1, "left": null, "right": null}}, "right": {"val": 6, "left": {"val": -4, "left": null, "right": null}, "right": null}}, "right": null}, "right": {"val": -7, "left": {"val": -6, "left": {"val": 5, "left": null, "right": null}, "right": null}, "right": {"val": -6, "left": {"val": 9, "left": {"val": -2, "left": null, "right": null}, "right": null}, "right": null}}}, "right": {"val": -3, "left": {"val": -4, "left": null, "right": null}, "right": null}}}"#.into()));
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

use std::cell::RefCell;
use std::rc::Rc;
struct Codec {
    buffer: String,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    fn serialize(&mut self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        self.s(root);
        self.buffer.to_owned()
    }

    fn s(&mut self, root: Option<Rc<RefCell<TreeNode>>>) {
        match root {
            None => {
                self.buffer.push_str("null");
            }
            Some(r) => {
                self.buffer.push_str("{");
                self.buffer.push_str(r#""val": "#);
                self.buffer.push_str(&format!("{}, ", r.borrow().val));
                self.buffer.push_str(r#""left": "#);
                self.s(r.borrow_mut().left.take());
                self.buffer.push_str(", ");
                self.buffer.push_str(r#""right": "#);
                self.s(r.borrow_mut().right.take());
                self.buffer.push_str("}");
            }
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        self.d(&mut data.as_bytes())
    }

    fn d(&self, data: &mut &[u8]) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node = None;
        if data.starts_with(&[b'n', b'u', b'l', b'l']) {
            *data = &data[4..];
            return node;
        }

        loop {
            if data.starts_with(&[b'{']) {
                *data = &data[1..];
            } else if data.starts_with(&[b'"', b'l', b'e', b'f', b't', b'"', b':', b' ']) {
                *data = &data[8..];
                node.get_or_insert(Rc::new(RefCell::new(TreeNode::new(-1))))
                    .borrow_mut()
                    .left = self.d(data);
            } else if data.starts_with(&[b'"', b'r', b'i', b'g', b'h', b't', b'"', b':', b' ']) {
                *data = &data[9..];
                node.get_or_insert(Rc::new(RefCell::new(TreeNode::new(-1))))
                    .borrow_mut()
                    .right = self.d(data);
            } else if data.starts_with(&[b'"', b'v', b'a', b'l', b'"', b':', b' ']) {
                *data = &data[7..];
                let mut v = 0;
                let mut sign = 1;
                let mut i = 0usize;
                if data.starts_with(&[b'-']) {
                    *data = &data[1..];
                    sign = -1;
                }

                while data[i] >= b'0' && data[i] <= b'9' {
                    v = v * 10 + (data[i] - b'0') as i32;
                    *data = &data[1..];
                }

                node.get_or_insert(Rc::new(RefCell::new(TreeNode::new(-1))))
                    .borrow_mut()
                    .val = v * sign;
            } else if data.starts_with(&[b'}']) {
                *data = &data[1..];
                return node;
            } else {
                *data = &data[1..];
            }
        }
    }
}
