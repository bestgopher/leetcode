fn main() {
    println!(
        "{:?}",
        Solution::prefix_table("aabaabaaa".to_string().as_bytes())
    );
    println!(
        "{:?}",
        Solution::prefix_table("abcdabca".to_string().as_bytes())
    );
    println!(
        "{:?}",
        Solution::prefix_table("ababcaabc".to_string().as_bytes())
    );
    println!(
        "{:?}",
        Solution::prefix_table("aabaaac".to_string().as_bytes())
    );
    println!(
        "{}",
        Solution::str_str("abxabcabcaby".to_string(), "abcaby".to_string())
    );
    println!(
        "{}",
        Solution::str_str("ababcaababcaabc".to_string(), "ababcaabc".to_string())
    );
    println!(
        "{}",
        Solution::str_str("aabaaabaaac".to_string(), "aabaaac".to_string())
    );
}

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() == 0 {
            return 0;
        }
        let (nums, s) = (needle.as_bytes(), haystack.as_bytes());
        let next = Self::prefix_table(&nums[..]);
        let (mut i, mut j) = (0, 0); // i是s的下标，j是next的下标
        while i < s.len() {
            if s[i] == nums[j] {
                if j == nums.len() - 1 {
                    return (i - j) as i32;
                }
                i += 1;
                j += 1;
            } else {
                if j != 0 {
                    j = next[j - 1];
                } else {
                    i += 1;
                }
            }
        }
        -1
    }

    /// 获取到前缀表
    fn prefix_table(nums: &[u8]) -> Vec<usize> {
        let mut tables = vec![0usize; nums.len()];
        tables[0] = 0; // 第一个元素的前缀表为0
        let mut j = 0;
        for i in 1..nums.len() {
            if nums[j] == nums[i] {
                tables[i] = tables[i - 1] + 1;
                j += 1;
            } else {
                while j > 0 {
                    j = tables[j - 1];
                    if nums[j] == nums[i] {
                        tables[i] = tables[j] + 1;
                        j += 1;
                        break;
                    } else {
                        tables[i] = 0;
                    }
                }
            }
        }

        tables
    }
}
