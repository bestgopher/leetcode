fn main() {}

struct Solution;

impl Solution {
    pub fn is_monotonic1(a: Vec<i32>) -> bool {
        if a.len() <= 1 {
            return true;
        }

        let mut f = None;

        for i in 1..a.len() {
            if a[i] == a[i - 1] {
                continue;
            }

            if f.is_none() {
                f = Some(a[i] > a[i - 1]);
            }
            if let Some(s) = f {
                if s && a[i] < a[i - 1] {
                    return false;
                }

                if !s && a[i] > a[i - 1] {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_monotonic(a: Vec<i32>) -> bool {
        if a.len() <= 1 {
            return true;
        }

        let mut s = 0;

        for i in 1..a.len() {
            if a[i] == a[i - 1] {
                continue;
            }

            if s == 0 && a[i] - a[1] != 0 {
                s = a[i] - a[i - 1];
            }

            if s * a[i] - a[i - 1] < 0 {
                return false;
            }
        }

        true
    }
}
