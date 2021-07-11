fn main() {
    let x = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    }));

    let r = Solution::reverse_k_group(x, 5);
    println!("{:?}", r);
}

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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k == 1 {
            return head;
        }
        let mut head = head;
        let mut k1 = k;

        let mut v = Vec::with_capacity(k as usize);
        while k1 > 0 {
            k1 -= 1;
            let next = head.as_mut().unwrap().next.take();
            v.push(head);
            head = next;
            if head.is_none() {
                break;
            }
        }

        let x = Self::reverse_k_group(head, k);
        if k1 == 0 {
            v.first_mut().unwrap().as_mut().unwrap().next = x;
            Self::rev_build(&mut v)
        } else {
            v.last_mut().unwrap().as_mut().unwrap().next = x;
            Self::build(&mut v)
        }
    }

    fn build(v: &mut [Option<Box<ListNode>>]) -> Option<Box<ListNode>> {
        if v.len() == 0 {
            return None;
        }

        let mut root = v[0].take();
        let x = Self::build(&mut v[1..]);
        if x.is_some() {
            root.as_mut().unwrap().next = x;
        }
        root
    }

    fn rev_build(v: &mut [Option<Box<ListNode>>]) -> Option<Box<ListNode>> {
        if v.len() == 0 {
            return None;
        }

        let mut root = v[v.len() - 1].take();
        let l = v.len() - 1;
        let x = Self::rev_build(&mut v[..l]);
        if x.is_some() {
            root.as_mut().unwrap().next = x;
        }
        root
    }
}
