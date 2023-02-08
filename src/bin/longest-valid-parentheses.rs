#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    for i in vec![
        ("(()", 2),
        (")()())", 4),
        ("", 0),
        ("(()))())(", 4),
        ("))))((()((", 2),
        ("()()))))()()(", 4),
    ] {
        assert_eq!(
            Solution::force_longest_valid_parentheses(i.0.into()),
            i.1,
            "{}",
            i.0
        );
        assert_eq!(
            Solution::longest_valid_parentheses(i.0.into()),
            i.1,
            "{}",
            i.0
        );
    }
}

struct Solution;

impl Solution {
    /// 暴力解法
    /// 从大到小开始求解
    pub fn force_longest_valid_parentheses(s: String) -> i32 {
        let mut len = s.len() / 2 * 2; // 因为括号都是成对出现的，所以从最长的长度开始
        let b = s.as_bytes();

        while len > 0 {
            for i in 0..b.len() {
                if i + len > b.len() {
                    break;
                }
                if b[i] == b')' {
                    continue;
                }

                if b[i + len - 1] != b')' {
                    continue;
                }

                let mut left_num = 0; // 左括号的数量
                let mut invalid = false;
                for j in i..i + len {
                    match b[j] {
                        b'(' => left_num += 1,
                        b')' => {
                            // 出现右括号但是左括号的数量为0时，说明不满足条件
                            if left_num == 0 {
                                invalid = true;
                                break;
                            }

                            left_num -= 1;
                        }
                        _ => unreachable!(),
                    }
                }

                if !invalid && left_num == 0 {
                    return len as i32;
                }
            }
            len -= 2;
        }

        0
    }

    /// 使用dp
    pub fn longest_valid_parentheses(s: String) -> i32 {
        // dp[i] = 2+dp[i-1] + dp[i-dp[i-1]-2]
        // 2+dp[i-1]是内部的
        // dp[i-dp[i-1]-2]是与之相连的外部的
        let mut dp = vec![0; s.len()];
        let mut b = s.as_bytes();
        let mut ans = 0;
        for i in 0..b.len() {
            match b[i] {
                b')' => {
                    if i < 1 {
                        continue;
                    }

                    let inner = dp[i - 1];
                    if i > inner {
                        if b[i - inner - 1] == b'(' {
                            dp[i] += inner + 2;

                            if i > inner + 2 {
                                dp[i] += dp[i - inner - 2];
                            }
                        }
                    }

                    ans = ans.max(dp[i] as i32);
                }
                _ => continue,
            }
        }

        ans
    }
}
