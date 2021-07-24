fn main() {
    println!("{}", Solution::num_squares(12));
    println!("{}", Solution::num_squares(13));
}

struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut h = std::collections::HashMap::new();
        Self::f(n, &mut h)
    }

    fn f(n: i32, h: &mut std::collections::HashMap<i32, i32>) -> i32 {
        if n == 0 {
            return 0;
        }

        if let Some(&x) = h.get(&n) {
            return x;
        }

        let mut s = 0;

        let mut m = 1;
        while m * m <= n {
            let s1 = 1 + Self::f(n - m * m, h);
            if s == 0 {
                s = s1;
            } else {
                s = s.min(s1)
            }
            m += 1;
        }

        h.insert(n, s);

        s
    }
}
