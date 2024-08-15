#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut min_distance = i32::MAX;
        let mut pre_value = head.as_ref().unwrap().val;
        let mut pre_index = 0; // 前一个节点的下标
        let mut first_index = None; // 第一个极值的下标
        let mut pre_index_1 = None; // 前一个极值的下标

        let mut current = head.unwrap().next;
        while current.is_some() {
            let mut c = current.unwrap();
            let current_val = c.val;
            let next = c.next.take();
            if next.is_some() {
                let next_value = next.as_ref().unwrap().val;
                if (current_val < pre_value && current_val < next_value)
                    || (current_val > pre_value && current_val > next_value)
                {
                    if first_index.is_none() {
                        first_index = Some(pre_index + 1);
                    }

                    if let Some(x) = pre_index_1 {
                        min_distance = min_distance.min(pre_index + 1 - x);
                    }

                    pre_index_1 = Some(pre_index + 1);
                }
            }

            pre_index += 1;
            current = next;
            pre_value = current_val;
        }

        if first_index != pre_index_1 {
            vec![min_distance, pre_index_1.unwrap() - first_index.unwrap()]
        } else {
            vec![-1, -1]
        }
    }
}
