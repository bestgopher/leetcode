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
    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut h = ListNode { val: 0, next: None };
        let mut current = &mut h.next;
        let mut head = head;
        let mut next = head.as_mut().and_then(|x| x.next.take());

        loop {
            match (head, next) {
                (Some(mut x), Some(mut y)) => {
                    next = y.next.take();
                    let (max, min) = if x.val > y.val {
                        (x.val, y.val)
                    } else {
                        (y.val, x.val)
                    };

                    let gdc = Self::gdc(max, min);
                    current.insert(x);
                    current = &mut current.as_mut().unwrap().next;
                    current.insert(Box::new(ListNode::new(gdc)));
                    current = &mut current.as_mut().unwrap().next;
                    head = Some(y);
                }
                (Some(x), None) => {
                    current.insert(x);
                    break;
                }
                _ => break,
            }
        }

        h.next.take()
    }

    /// 辗转相除法， 又名欧几里德算法（Euclidean algorithm），是求最大公约数的一种方法。方法是：
    /// 用较大数除以较小数，再用出现的余数（第一余数）去除除数，再用出现的余数（第二余数）去除第一余数，
    /// 如此反复，直到最后余数是0为止。那么最后的除数就是这两个数的最大公约数。
    pub fn gdc(max: i32, min: i32) -> i32 {
        let r = max % min;
        if r == 0 {
            return min;
        }

        Self::gdc(min, r)
    }
}

#[test]
fn test_gdc() {
    assert_eq!(6, Solution::gdc(30, 18));
    assert_eq!(6, Solution::gdc(123456, 7890));
}
