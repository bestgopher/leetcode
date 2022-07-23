#![allow(dead_code, unused, unused_variables)]

fn main() {
    let s = Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]);
    println!("{s:?}");

    let s = Solution::smaller_numbers_than_current(vec![6, 5, 4, 8]);
    println!("{s:?}");

    let s = Solution::smaller_numbers_than_current(vec![7, 7, 7, 7]);
    println!("{s:?}");
}

struct Solution;

impl Solution {
    pub fn smaller_numbers_than_current(mut nums: Vec<i32>) -> Vec<i32> {
        let mut result = nums.clone();
        result.sort();

        'out_loop: for i in nums.iter_mut() {
            if *i <= result[0] {
                *i = 0;
            } else if *i > result[result.len() - 1] {
                *i = result.len() as i32;
            } else {
                let (mut start, mut end) = (0, result.len() - 1);
                let mut middle = (start + end) / 2;

                while start <= end {
                    if *i > result[middle] {
                        start = middle + 1;
                    } else if *i == result[middle] {
                        if middle == 0 || result[middle - 1] < *i {
                            *i = middle as i32;
                            continue 'out_loop;
                        } else if result[middle - 1] == *i {
                            end = middle - 1;
                        } else {
                            start = middle + 1;
                        }
                    } else {
                        end = middle - 1;
                    }
                    middle = (start + end) / 2;
                }
            }
        }
        nums
    }
}
