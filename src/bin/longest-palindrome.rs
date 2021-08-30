fn main() {}

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut hash = std::collections::HashMap::new();
        s.as_bytes().into_iter().for_each(|x| {
            hash.entry(x).and_modify(|v| *v += 1).or_insert(1);
        });

        let mut odd = false; // 是否有单数，如果有单数的话，则最后的结果+1

        let mut r = 0;

        hash.into_iter().for_each(|(_, v)| if v % 2 == 0 { r += v; } else {
            odd = true;
            r += v - 1;
        });


        if odd { r + 1 } else { r }
    }
}
