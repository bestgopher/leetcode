#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::ptr::NonNull;

fn main() {}

struct Solution;

struct NumArray {
    nums: Vec<i32>,
    segment_tree: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut segment_tree = vec![0; nums.len() * 4];
        let mut num_array = Self { nums, segment_tree };

        num_array.build(0, 0, num_array.nums.len() - 1);
        num_array
    }

    fn build(&mut self, tree_index: usize, l: usize, r: usize) {
        if l == r {
            self.segment_tree[tree_index] = self.nums[l];
            return;
        }

        let left_tree_index = tree_index * 2 + 1;
        let right_tree_index = tree_index * 2 + 2;

        let mid = l + (r - l) / 2;
        self.build(left_tree_index, l, mid);
        self.build(right_tree_index, mid + 1, r);

        self.segment_tree[tree_index] =
            self.segment_tree[left_tree_index] + self.segment_tree[right_tree_index];
    }

    fn update(&mut self, index: i32, val: i32) {
        let v = val - self.nums[index as usize];
        self.nums[index as usize] = val;
        self.update_index(0, 0, self.nums.len() - 1, index as usize, v);
    }

    fn update_index(&mut self, tree_index: usize, left_index: usize, right_index: usize, index: usize, val: i32) {
        self.segment_tree[tree_index] += val;
        if left_index == right_index {
            return;
        }

        let mid = left_index + (right_index - left_index) / 2;
        if index <= mid {
            self.update_index(tree_index*2+1, left_index, mid, index, val);
        } else {
            self.update_index(tree_index*2+2, mid + 1, right_index, index, val);
        }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.get_range(0, 0, self.nums.len() - 1, left as usize, right as usize)
    }

    /// tree_index: 线段树的索引
    /// l: tree_index表示的左边界
    /// r: tree_index表示的右边界
    /// left: 查询的左边界
    /// right: 查询的右边界
    fn get_range(&self, tree_index: usize, l: usize, r: usize, left: usize, right: usize) -> i32 {
        if left == l && right == r {
            return self.segment_tree[tree_index];
        }

        let mid = l + (r - l) / 2;
        let left_tree_index = tree_index * 2 + 1;
        let right_tree_index = tree_index * 2 + 2;

        if left > mid {
            return self.get_range(right_tree_index, mid + 1, r, left, right);
        } else if right <= mid {
            return self.get_range(left_tree_index, l, mid, left, right);
        } else {
            self.get_range(left_tree_index, l, mid, left, mid)
                + self.get_range(right_tree_index, mid + 1, r, mid+1, right)
        }
    }
}
