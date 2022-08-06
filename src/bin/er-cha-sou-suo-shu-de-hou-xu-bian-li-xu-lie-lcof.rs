#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert!(!Solution::verify_postorder(vec![1, 6, 3, 2, 5]));
    assert!(Solution::verify_postorder(vec![1, 3, 2, 6, 5]));
    assert!(Solution::verify_postorder(vec![4, 8, 6, 12, 16, 14, 10]));
    assert!(Solution::verify_postorder(vec![5, 4, 3, 2, 1]));
}

struct Solution;

impl Solution {
    pub fn verify_postorder(postorder: Vec<i32>) -> bool {
        Self::recurse(&postorder[..])
    }

    pub fn recurse(postorder: &[i32]) -> bool {
        if postorder.is_empty() {
            return true;
        }

        let mut p = 0;
        while p < postorder.len() && postorder[p] < postorder[postorder.len() - 1] {
            p += 1;
        }

        let mut q = p;
        while q < postorder.len() && postorder[q] > postorder[postorder.len() - 1] {
            q += 1;
        }

        postorder[0..p].len() + postorder[p..q].len() + 1 == postorder.len()
            && Self::recurse(&postorder[0..p])
            && Self::recurse(&postorder[p..q])
    }
}
