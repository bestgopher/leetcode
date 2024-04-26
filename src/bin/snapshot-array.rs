#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */

struct SnapshotArray {
    snapshot_id: std::cell::Cell<i32>,
    storage: Vec<Vec<(i32, i32)>>,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        Self {
            snapshot_id: std::cell::Cell::new(0),
            storage: {
                let mut data = Vec::with_capacity(length as usize);
                for _ in 0..length {
                    data.push(vec![]);
                }
                data
            },
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        if let Some(x) = self.storage[index as usize].last_mut() {
            if x.0 == self.snapshot_id.get() {
                x.1 = val;
                return;
            }
        }

        self.storage[index as usize].push((self.snapshot_id.get(), val));
    }

    fn snap(&self) -> i32 {
        self.snapshot_id.set(self.snapshot_id.get() + 1);
        self.snapshot_id.get() - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let s = self.storage.get(index as usize).unwrap();
        let i = s.partition_point(|x| x.0 <= snap_id);
        if i > 0 {
            s[i - 1].1
        } else {
            0
        }
    }
}
