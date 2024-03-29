#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut hash = std::collections::HashMap::<i32, i32>::new();

        for &i in nums.iter() {
            hash.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut res = 0;

        for &i in nums.iter() {
            let s = hash.get(&(i - 1));
            let s1 = hash.get(&i).unwrap();
            if s.is_some() {
                if s1 + *s.unwrap() > res {
                    res = s1 - *s.unwrap();
                }
            }
        }

        res
    }

    pub fn find_lhs1(nums: Vec<i32>) -> i32 {
        let mut hash = std::collections::HashMap::<i32, i32>::new();
        let mut res = 0;
        for &i in nums.iter() {
            hash.entry(i).and_modify(|x| *x += 1).or_insert(1);

            let s = hash.get(&(i - 1));
            if s.is_some() && res < hash.get(&i).unwrap() + s.unwrap() {
                res = hash.get(&i).unwrap() + s.unwrap()
            }

            let s = hash.get(&(i + 1));
            if s.is_some() && res < hash.get(&i).unwrap() + s.unwrap() {
                res = hash.get(&i).unwrap() + s.unwrap()
            }
        }

        res
    }
}
