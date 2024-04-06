#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

struct FindSumPairs {
    nums2_map: std::collections::HashMap<i32, i32>,
    nums1: Vec<i32>,
    nums2: Vec<i32>,
}

impl FindSumPairs {

    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut nums2_map = std::collections::HashMap::new();

        for &i in nums2.iter() {
            nums2_map.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }


        Self {
            nums2_map,
            nums1,
            nums2,
        }
    }
    
    fn add(&mut self, index: i32, val: i32) {
        let mut origin = self.nums2[index as usize];
        self.nums2[index as usize] += val;
        self.nums2_map.entry(origin).and_modify(|x| *x-=1);
        self.nums2_map.entry(origin + val).and_modify(|x| *x += 1).or_insert(1);
    }
    
    fn count(&self, tot: i32) -> i32 {
        let mut result = 0;
        for &i in self.nums1.iter() {
            result += *self.nums2_map.get(&(tot-i)).unwrap_or(&0);
        }

        result
    }
}

/**
 * Your FindSumPairs object will be instantiated and called as such:
 * let obj = FindSumPairs::new(nums1, nums2);
 * obj.add(index, val);
 * let ret_2: i32 = obj.count(tot);
 */
