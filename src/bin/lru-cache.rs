#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

fn main() {
    // ["LRUCache","put","put","put","put","get","get","get","get","put","get","get","get","get","get"]
    // [[3],       [1,1],[2,2],[3,3],[4,4],[4],  [3],  [2],   [1], [5,5], [1],  [2],  [3],  [4],  [5]]
    // [null,       null, null, null, null, 4,    3,    2,    -1,   null, -1,   2,     3,    -1,   5]

    let mut lru = LRUCache::new(3);
    lru.put(1, 1);
    lru.put(2, 2);
    lru.put(3, 3);
    lru.put(4, 4);

    assert_eq!(lru.get(4), 4);
    assert_eq!(lru.get(3), 3);
    assert_eq!(lru.get(2), 2);
    assert_eq!(lru.get(1), -1);

    lru.put(5, 5);

    assert_eq!(lru.get(1), -1);
    assert_eq!(lru.get(2), 2);
    assert_eq!(lru.get(3), 3);
    assert_eq!(lru.get(4), -1);
    assert_eq!(lru.get(5), 5);
}

struct Solution;

#[derive(Clone)]
struct KeyValuePair {
    key: i32,
    value: i32,
}

struct LRUCache {
    map: std::collections::HashMap<i32, Rc<RefCell<Node>>>,
    list: List,
    capacity: usize,
}

struct Node {
    value: KeyValuePair,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Rc<RefCell<Node>>>,
}

struct List {
    length: usize,
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl List {
    fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
        }
    }

    // 从链表中移除当前节点
    // 当前节点的prev节点的next节点指向当前节点的next节点
    // 当前节点的next节点的prev节点指向当前节点的prev节点
    fn remove(&mut self, node: Rc<RefCell<Node>>) {
        if self.length == 0 {
            return;
        }

        self.length -= 1;

        let prev = node.borrow_mut().prev.take();
        let next = node.borrow_mut().next.take();

        // 说明移除的是头节点, head要指向next节点
        if prev.is_none() {
            self.head = next.clone();
        }

        // 说明移除的尾节点，tail要指向prev节点
        if next.is_none() {
            self.tail = prev.clone();
        }

        prev.clone().map(|x| x.borrow_mut().next = next.clone());
        next.clone().map(|x| x.borrow_mut().prev = prev.clone());
    }

    /// 向列表头部插入数据
    /// head指向新的数据
    /// 新数据的next指向原列表头部
    fn push_front(&mut self, value: KeyValuePair) -> Rc<RefCell<Node>> {
        self.length += 1;

        let node = Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }));

        // 获取当前头头节点
        let head = self.head.take();

        // 新的头节点的下一个元素指向旧的头节点
        node.borrow_mut().next = head.clone();

        // 旧的头节点的prev指向新的头节点
        head.clone()
            .map(|x| x.borrow_mut().prev = Some(node.clone()));

        // 如果长度为1，尾节点和头节点指向头一个节点
        if self.length == 1 {
            self.tail = Some(node.clone());
        }

        self.head = Some(node.clone());

        node
    }

    /// 移除尾部元素
    /// 获取到tail的值，
    /// 将tail.prev设置为tail
    /// tail.prev.next设置为None
    /// 返回最后节点的key
    fn pop(&mut self) -> Option<i32> {
        match self.tail.take() {
            None => None,
            Some(node) => {
                self.length -= 1;
                let prev = node.borrow().prev.clone();
                prev.clone().map(|x| x.borrow_mut().next = None);
                self.tail = prev.clone(); // 将tail节点设置为 prev
                node.borrow_mut().prev = None;

                if self.length == 0 {
                    self.head = None;
                }

                Some(node.borrow().value.key)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        // 回收所有节点
        while let Some(_) = self.pop() {}
    }
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            map: Default::default(),
            list: List::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        match self._get(key) {
            None => -1,
            Some(x) => x.borrow().value.value,
        }
    }

    fn _get(&mut self, key: i32) -> Option<Rc<RefCell<Node>>> {
        match self.map.get_mut(&key) {
            None => None,
            Some(x) => {
                self.list.remove(x.clone()); // 先移除当前节点
                let new = self.list.push_front(x.borrow().value.clone()); // 新插入节点
                *x = new.clone();
                Some(new)
            }
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        match self._get(key) {
            None => {
                if self.map.len() == self.capacity {
                    if let Some(x) = self.list.pop() {
                        self.map.remove(&x);
                    }
                }

                let x = self.list.push_front(KeyValuePair { key, value });
                self.map.insert(key, x);
            }
            Some(x) => {
                x.borrow_mut().value.value = value;
            }
        }
    }
}
