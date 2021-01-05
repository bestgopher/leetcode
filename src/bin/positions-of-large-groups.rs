fn main() {
    assert_eq!(vec![vec![3, 6]], Solution::large_group_positions("abbxxxxzzy".to_string()));
    assert_eq!(Vec::<Vec<i32>>::new(), Solution::large_group_positions("abc".to_string()));
    assert_eq!(vec![vec![3, 5], vec![6, 9], vec![12, 14]], Solution::large_group_positions("abcdddeeeeaabbbcd".to_string()));
    assert_eq!(vec![vec![0, 2]], Solution::large_group_positions("aaa".to_string()));
}

struct Solution;

impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        let mut v = Vec::<Vec<i32>>::new();
        let s = s.as_bytes();
        let mut start = 0usize;
        let mut end = 0usize;

        for (index, &value) in s.iter().enumerate() {
            if index == 0 {
                continue;
            }

            if value == s[index - 1] {
                end = index;
                if end != s.len()- 1 {
                    continue;
                }
            }

            if end - start >= 2 {
                v.push(vec![start as i32, end as i32]);
            }
            start = index;
            end = index;
        }

        v
    }
}