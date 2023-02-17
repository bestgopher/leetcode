#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::vec;

fn main() {
    assert_eq!(
        Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
        vec![2, 1]
    );

    assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    assert_eq!(Solution::top_k_frequent(vec![3, 0, 1, 0], 1), vec![0]);
}

struct Solution;

impl Solution {
    /// 先获取数字对应的次数，再用小顶堆，再遍历堆
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hash = std::collections::HashMap::new();

        for i in nums {
            hash.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut heap = vec![];

        for (x, y) in hash {
            Self::buildHeap(&mut heap, k as usize, (x, y));
        }

        heap.into_iter().map(|x| x.0).collect()
    }

    fn buildHeap(heap: &mut Vec<(i32, i32)>, length: usize, elem: (i32, i32)) {
        if heap.len() == length {
            if heap[0].1 < elem.1 {
                // 先删除掉第一个元素
                heap.swap(0, length - 1);
                heap.remove(length - 1);
                let mut start = 0;
                // 堆顶向下比较
                while start < length {
                    if start * 2 + 1 < heap.len() && start * 2 + 2 < heap.len() {
                        let mut min = start * 2 + 1;
                        if heap[start * 2 + 1].1 > heap[start * 2 + 2].1 {
                            min = start * 2 + 2;
                        }

                        if heap[min].1 < heap[start].1 {
                            heap.swap(start, min);
                            start = min;
                        } else {
                            break;
                        }
                    } else if start * 2 + 1 < heap.len() {
                        if heap[start * 2 + 1].1 < heap[start].1 {
                            heap.swap(start, start * 2 + 1);
                            start = start * 2 + 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            } else {
                return;
            }
        }

        heap.push(elem);
        let mut start = heap.len() - 1;

        while start > 0 {
            if heap[(start - 1) / 2].1 > heap[start].1 {
                heap.swap(start, (start - 1) / 2);
                start = (start - 1) / 2;
            } else {
                return;
            }
        }
    }
}
