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
    /// 普通方法，先获取所有的值，然后再组装新的链表返回
    pub fn swap_nodes1(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut v = vec![];

        while let Some(i) = head.take() {
            v.push(i.val);
            head = i.next;
        }
        let n = v.len() - k as usize;
        v.swap(k as usize - 1, n);

        let mut head = None;
        for i in (0..v.len()).rev() {
            let mut node = ListNode::new(v[i]);
            if head.is_none() {
                head = Some(Box::new(node));
            } else {
                node.next = head.take();
                head = Some(Box::new(node));
            }
        }

        head
    }

    /// 快慢指针，先快指针到第k个值的时候，慢指针从第一个元素开始移动
    /// 快指针到达最后一个元素的时候，慢指针刚好在倒数第k个元素的位置
    pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut index = 1;
        let (mut fast, mut slow, mut k_node) = (head.as_ref(), head.as_ref(), None);

        while fast.is_some() {
            if index == k {
                k_node = fast;
            } else if index > k {
                slow = slow.unwrap().next.as_ref();
            }
            fast = fast.unwrap().next.as_ref();
            index += 1;
        }

        let _f = k_node.unwrap().val;
        let _s = slow.unwrap().val;

        head
    }
}
