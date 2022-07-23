#![allow(dead_code, unused, unused_variables)]

fn main() {
    // assert_eq!(0, Solution::split(&vec![1, 2, 3, 7, 8, 9], 1));
    // assert_eq!(1, Solution::split(&vec![1, 2, 3, 7, 8, 9], 2));
    // assert_eq!(2, Solution::split(&vec![1, 2, 3, 7, 8, 9], 3));
    // assert_eq!(2, Solution::split(&vec![1, 2, 3, 7, 8, 9], 4));
    // assert_eq!(3, Solution::split(&vec![1, 2, 3, 7, 8, 9], 7));
    // assert_eq!(4, Solution::split(&vec![1, 2, 3, 7, 8, 9], 8));
    // assert_eq!(5, Solution::split(&vec![1, 2, 3, 7, 8, 9], 9));
    assert_eq!(1, Solution::split(&vec![1, 2, 2, 2, 2, 2, 3, 3, 3, 3], 2));
    assert_eq!(0, Solution::split(&vec![1, 2, 2, 2, 2, 2, 3, 3, 3, 3], -1));
}

struct Solution;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        if x < arr[0] {
            return arr[..k as usize].to_vec();
        }

        if x < arr[0] {
            return arr[arr.len() - k as usize..].to_vec();
        }

        if arr.len() == 1 {
            return arr;
        }

        let index = Self::split(&arr, x);
        let (mut start, mut end, k) = (index, index, k as usize);
        while end - start < k - 1 {
            if start == 0 {
                end += 1;
            } else if end == arr.len() - 1 {
                start -= 1;
            } else {
                println!("{}, {}, aaa", x - arr[start - 1], arr[end + 1] - x);
                if x - arr[start - 1] > arr[end + 1] - x {
                    end += 1;
                } else {
                    start -= 1;
                }
            }
        }
        arr[start..=end].to_vec()
    }

    // 二分查找找到最接近x的元素的下标
    pub fn split(arr: &Vec<i32>, x: i32) -> usize {
        let (mut start, mut end) = (0, arr.len() - 1);
        loop {
            if end - start <= 1 {
                if arr[end] - x >= x - arr[start] {
                    break start;
                } else {
                    break end;
                }
            }
            let mut index = (start + end) / 2;
            if arr[index] > x {
                end = index;
            } else if arr[index] < x {
                start = index;
            } else {
                while index > 0 {
                    if arr[index - 1] == x {
                        index -= 1;
                    } else {
                        break;
                    }
                }

                break index;
            }
        }
    }
}
