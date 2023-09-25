#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

struct LinkList {
    head: Option<std::rc::Rc<std::cell::RefCell<Node>>>,
    tail: Option<std::rc::Rc<std::cell::RefCell<Node>>>,
}

struct Node {
    next: Option<std::rc::Rc<std::cell::RefCell<Node>>>,
    prev: Option<std::rc::Weak<std::cell::RefCell<Node>>>,
    key: i32,
    val: i32,
}

impl LinkList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, key: i32, val: i32) -> std::rc::Rc<std::cell::RefCell<Node>> {
        use std::cell::RefCell;
        use std::rc::Rc;

        let node = Rc::new(RefCell::new(Node {
            prev: None,
            next: None,
            key,
            val,
        }));

        if self.head.is_none() {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(Rc::clone(&node));
        } else {
            let mut head = self.head.take();
            head.as_ref()
                .map(|x| x.borrow_mut().prev = Some(Rc::downgrade(&node)));
            node.borrow_mut().next = head.clone();
            self.head = Some(Rc::clone(&node));
        }

        node
    }

    fn remove(&mut self, val: std::rc::Rc<std::cell::RefCell<Node>>) {
        use std::cell::RefCell;
        use std::rc::Rc;

        let prev = val.borrow_mut().prev.take().and_then(|x| x.upgrade());
        let next = val.borrow_mut().next.take();

        prev.as_ref().map(|x| x.borrow_mut().next = None);
        next.as_ref().map(|x| x.borrow_mut().prev = None);

        prev.as_ref().map(|x| x.borrow_mut().next = next.clone());
        next.as_ref()
            .map(|x| x.borrow_mut().prev = prev.clone().map(|x| Rc::downgrade(&x)));

        // 移除节点的前继节点为空，说明此节点为头节点，移除next节点成为新的头部节点
        if prev.is_none() {
            self.head = next.clone();
        }

        if next.is_none() {
            self.tail = prev.clone();
        }
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn pop(&mut self) -> Option<i32> {
        let tail = self.tail.clone();
        if tail.is_none() {
            return None;
        }
        self.remove(tail.clone().unwrap());
        tail.clone().map(|x| x.borrow().key)
    }
}

struct LFUCache {
    /// 容量
    capacity: usize,
    length: usize,
    /// 存放数据的集合, 题目不需要delete，data:(frequency, Node)
    data: std::collections::HashMap<i32, (i32, std::rc::Rc<std::cell::RefCell<Node>>)>,
    /// 存放节点的频率
    frequencies: std::collections::HashMap<i32, LinkList>,
    /// 当前最小的频率
    min_frequency: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
impl LFUCache {
    fn new(capacity: i32) -> Self {
        use std::collections::HashMap;
        Self {
            capacity: capacity as _,
            length: 0,
            data: HashMap::new(),
            frequencies: HashMap::new(),
            min_frequency: 0,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if !self.data.contains_key(&key) {
            return -1;
        }

        self.increment(key, None)
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.data.contains_key(&key) {
            self.increment(key, Some(value));
        } else {
            // 如果长度==容量，先要移除最小频率的值
            if self.length == self.capacity {
                let k = self
                    .frequencies
                    .get_mut(&self.min_frequency)
                    .and_then(|x| x.pop());

                k.map(|x| self.data.remove(&x));
                self.length -= 1;

                if self
                    .frequencies
                    .get(&self.min_frequency)
                    .map(|x| x.is_empty())
                    .unwrap_or(true)
                {
                    self.frequencies.remove(&self.min_frequency);
                }
                {}
            }
            self.length += 1;
            self.min_frequency = 1;

            if self.frequencies.get(&1).is_none() {
                let mut l = LinkList::new();
                self.data.insert(key, (1, l.push_front(key, value)));
                self.frequencies.insert(1, l);
                return;
            }

            let a = if let Some(x) = self.frequencies.get_mut(&1) {
                x.push_front(key, value)
            } else {
                return;
            };
            self.data.insert(key, (1, a));
        }
    }

    /// 增加key的频率，当new_val为Some时，替换key的值，返回旧值
    fn increment(&mut self, key: i32, new_val: Option<i32>) -> i32 {
        let r = self.data.get(&key).unwrap();
        if let Some(val) = new_val {
            r.1.borrow_mut().val = val;
        }

        let val = r.1.borrow().val;
        let fre = r.0;

        self.frequencies
            .get_mut(&fre)
            .map(|x| x.remove(r.1.clone()));

        if self.frequencies[&fre].is_empty() {
            self.frequencies.remove(&fre);

            if fre == self.min_frequency {
                self.min_frequency = fre + 1;
            }
        }

        if self.frequencies.get(&(fre + 1)).is_none() {
            let mut l = LinkList::new();
            self.data.insert(key, (fre + 1, l.push_front(key, val)));
            self.frequencies.insert(fre + 1, l);
            return val;
        }

        let a = if let Some(x) = self.frequencies.get_mut(&(fre + 1)) {
            x.push_front(key, val)
        } else {
            return val;
        };
        self.data.insert(key, (fre + 1, a));
        val
    }
}

impl std::fmt::Debug for LFUCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .entries(self.data.iter().map(|x| (*x.0, x.1 .0)))
            .finish()?;

        f.debug_list()
            .entries(self.frequencies.iter().map(|x| *x.0))
            .finish()?;

        f.debug_map()
            .key(&"min")
            .value(&self.min_frequency)
            .finish()
    }
}
