#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn halve_array(nums: Vec<i32>) -> i32 {
        #[derive(Clone, Copy, PartialEq, PartialOrd)]
        struct F64(f64);

        impl Eq for F64 {}

        impl Ord for F64 {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.partial_cmp(other).unwrap()
            }
        }

        use std::collections::binary_heap::BinaryHeap;
        let sum = nums.iter().map(|x| *x as f64).sum::<f64>();
        let mut heap: BinaryHeap<F64> = nums.into_iter().map(|x| F64(x as f64)).collect();
        let mut s = 0f64;
        let mut r = 0;

        loop {
            r += 1;
            let p = heap.pop().unwrap();
            if sum - s - p.0 / 2f64 <= sum / 2f64 {
                return r;
            }
            heap.push(F64(p.0 / 2f64));
            s += p.0 / 2f64;
        }
    }
}
