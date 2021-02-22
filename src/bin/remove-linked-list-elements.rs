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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut head = head;

        // 去掉前面都是满足值的节点
        while head.is_some() && head.as_ref().unwrap().val == val {
            head = head.as_mut().unwrap().next.take();
        }

        let mut cursor = head.as_mut();
        while cursor.is_some() {
            if let Some(i) = cursor.as_mut().unwrap().next.as_mut() {
                if i.val == val {
                    cursor.as_mut().unwrap().next = i.next.take();
                    continue;
                }
            }

            cursor = cursor.unwrap().next.as_mut();
        }

        head
    }
}
