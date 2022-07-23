#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    // println!("{}", Solution::new(1702766719).first_bad_version(2126753390));
    // println!("{}", Solution::new(1).first_bad_version(1));
    println!("{}", Solution::new(2).first_bad_version(2));
}

struct Solution {
    n: i32,
}

// The API isBadVersion is defined for you.
// isBadVersion(versions:i32)-> bool;
// to call it use self.isBadVersion(versions)

impl Solution {
    fn new(n: i32) -> Self {
        Self { n }
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut good: i64 = 0;
        let mut bad: i64 = n as i64;
        while bad - good > 1 {
            let mid = (good + bad + 1) / 2;
            if self.isBadVersion(mid as i32) {
                bad = mid;
            } else {
                good = mid;
            }
        }
        bad as i32
    }

    fn isBadVersion(&self, versions: i32) -> bool {
        self.n <= versions
    }
}
