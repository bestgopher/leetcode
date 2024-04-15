#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */

struct Node {
    key: i32,
    value: i32,
}
struct MyHashMap {
    array: Vec<std::collections::LinkedList<Node>>,
}

impl MyHashMap {
    fn new() -> Self {
        Self {
            array: (0..1001)
                .map(|_| std::collections::LinkedList::new())
                .collect(),
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let index = key as usize / 1000;
        let mut iter = self.array[index].iter_mut();

        while let Some(n) = iter.next() {
            if n.key == key {
                n.value = value;
                return;
            }
        }

        self.array[index].push_back(Node { key, value });
    }

    fn get(&self, key: i32) -> i32 {
        let index = key as usize / 1000;
        let mut iter = self.array[index].iter();

        while let Some(n) = iter.next() {
            if n.key == key {
                return n.value;
            }
        }

        -1
    }

    fn remove(&mut self, key: i32) {
        let index = key as usize / 1000;
        let mut iter = self.array[index].iter();
        let mut x = None;

        for (i, n) in iter.enumerate() {
            if n.key == key {
                x = Some(i);
                break;
            }
        }

        if let Some(x) = x {
            let mut m = self.array[index].split_off(x);
            m.pop_front();
            self.array[index].append(&mut m);
        }
    }
}
