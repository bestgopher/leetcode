#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

struct LockingTree {
    children: std::collections::HashMap<i32, Vec<i32>>,
    parents: std::collections::HashMap<i32, i32>,
    /// node: user_id
    locked: std::collections::HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your LockingTree object will be instantiated and called as such:
 * let obj = LockingTree::new(parent);
 * let ret_1: bool = obj.lock(num, user);
 * let ret_2: bool = obj.unlock(num, user);
 * let ret_3: bool = obj.upgrade(num, user);
 */

impl LockingTree {
    fn new(parent: Vec<i32>) -> Self {
        let mut children = std::collections::HashMap::<i32, Vec<i32>>::new();
        let mut parents = std::collections::HashMap::new();
        for i in 0..parent.len() {
            children
                .entry(parent[i])
                .and_modify(|x| x.push(i as _))
                .or_insert(vec![i as _]);

            parents.insert(i as _, parent[i]);
        }

        Self {
            children,
            parents,
            locked: std::collections::HashMap::new(),
        }
    }

    fn lock(&mut self, num: i32, user: i32) -> bool {
        if self.locked.contains_key(&num) {
            return false;
        }

        self.locked.insert(num, user);
        true
    }

    fn unlock(&mut self, num: i32, user: i32) -> bool {
        match self.locked.get(&num) {
            Some(&x) if x == user => {}
            _ => return false,
        }

        self.locked.remove(&num);
        true
    }

    fn upgrade(&mut self, num: i32, user: i32) -> bool {
        if self.locked.contains_key(&num) {
            return false;
        }

        if self.check_parent(num) {
            return false;
        }

        if !self.unlock_sons(num) {
            return false;
        }

        self.locked.insert(num, user);
        true
    }

    /// 解锁子孙。如果有一个子孙已加锁，返回true。
    fn unlock_sons(&mut self, num: i32) -> bool {
        let len = if let Some(x) = self.children.get(&num) {
            x.len()
        } else {
            return false;
        };

        let mut result = false;
        for i in 0..len {
            result |= self.locked.remove(&self.children[&num][i]).is_some();
            result |= self.unlock_sons(self.children[&num][i]);
        }

        result
    }

    /// 检查父节点，如果都是未加锁，返回true
    fn check_parent(&self, num: i32) -> bool {
        match self.parents.get(&num) {
            Some(&x) => self.locked.contains_key(&x) || self.check_parent(x),
            None => false,
        }
    }
}
