#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    let mut lru = LRUCache::new(2);
    lru.put(1, 1);
    // println!("{:?}", lru.list);

    lru.put(2, 2);
    // println!("{:?}", lru.list);
    assert_eq!(lru.get(1), 1);
    // println!("{:?}", lru.list);
    lru.put(3, 3);
    assert_eq!(lru.get(2), -1);
    lru.put(4, 4);
    assert_eq!(lru.get(1), -1);
    assert_eq!(lru.get(3), 3);
    assert_eq!(lru.get(4), 4);
}

struct Solution;

#[derive(Debug)]
struct LinkedList {
    head: Option<std::rc::Rc<std::cell::RefCell<Node>>>,
    tail: Option<std::rc::Rc<std::cell::RefCell<Node>>>,
}

impl LinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn take(&mut self, node: std::rc::Rc<std::cell::RefCell<Node>>) {
        let pre = node.borrow_mut().pre.take();
        let next = node.borrow_mut().next.take();

        pre.clone().map(|x| x.borrow_mut().next = next.clone());
        next.clone().map(|x| x.borrow_mut().pre = pre.clone());

        if next.is_none() {
            self.tail = pre.clone();
        }

        if pre.is_none() {
            self.head = next.clone();
        }
    }

    fn push(&mut self, node: std::rc::Rc<std::cell::RefCell<Node>>) {
        let head = self.head.take();
        node.borrow_mut().next = head.clone();

        head.map(|x| x.borrow_mut().pre = Some(node.clone()));
        self.head = Some(node.clone());

        if self.tail.is_none() {
            self.tail = self.head.clone();
        }
    }
}

#[derive(Debug)]
struct Node {
    key: i32,
    val: i32,
    pre: Option<std::rc::Rc<std::cell::RefCell<Node>>>,
    next: Option<std::rc::Rc<std::cell::RefCell<Node>>>,
}

struct LRUCache {
    capacity: usize,
    map: std::collections::HashMap<i32, std::rc::Rc<std::cell::RefCell<Node>>>,
    list: LinkedList,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            map: std::collections::HashMap::new(),
            list: LinkedList::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(x) = self.map.get(&key) {
            self.list.take(x.clone());
            self.list.push(x.clone());
            x.borrow().val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(x) = self.map.get(&key) {
            self.list.take(x.clone());
            self.list.push(x.clone());
            x.borrow_mut().val = value;
        } else {
            // remove a value
            if self.map.len() == self.capacity {
                let tail = self.list.tail.clone().unwrap();
                self.list.take(tail.clone());
                self.map.remove(&tail.borrow().key);
            }

            let node = std::rc::Rc::new(std::cell::RefCell::new(Node {
                key,
                val: value,
                pre: None,
                next: None,
            }));

            self.list.push(node.clone());
            self.map.insert(key, node);
        }
    }
}
