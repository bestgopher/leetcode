fn main() {
    assert_eq!(6, Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2));
    assert_eq!(10, Solution::longest_ones(vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1], 3));
}

struct Solution;

impl Solution {
    pub fn longest_ones(a: Vec<i32>, mut k: i32) -> i32 {
        let (mut slow, mut fast, mut count) = (0, 0, 0);
        while fast < a.len() {
            if k > 0 {
                if a[fast] == 0 {
                    k -= 1;
                }
                fast += 1;
            } else {
                if a[fast] == 1 {
                    fast += 1;
                } else {
                    if a[slow] == 0 {
                        fast += 1
                    }
                    slow += 1;
                }
            }

            if fast < slow {
                fast = slow;
            }

            if fast - slow > count {
                count = fast - slow;
            }
        }

        count as i32
    }
}
