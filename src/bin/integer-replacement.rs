fn main() {
    println!("{}", Solution::integer_replacement(2147483647));
}

struct Solution;

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut h = std::collections::HashMap::new();
        Self::f(n as i64, &mut h) as i32
    }

    fn f(n: i64, h: &mut std::collections::HashMap<i64, i64>) -> i64 {
        if n == 1 {
            return 0;
        }

        if let Some(x) = h.get(&n) {
            return *x;
        }

        let m = if n % 2 == 0 {
            Self::f(n / 2, h) + 1
        } else {
            Self::f(n - 1, h).min(Self::f(n + 1, h)) + 1
        };

        h.insert(n, m);
        m
    }
}
