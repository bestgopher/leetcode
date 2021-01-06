fn main() {}

struct Solution;

impl Solution {
    pub fn is_ugly(num: i32) -> bool {
        if num == 1 {
            return true;
        }

        if num == 0 {
            return false;
        }

        let mut num = num;

        while num != 1 {
            if num % 2 == 0 {
                num /= 2;
            } else if num % 3 == 0 {
                num /= 3;
            } else if num % 5 == 0 {
                num /= 5;
            } else {
                return false;
            }
        }

        true
    }

    pub fn is_ugly1(num: i32) -> bool {
        if num == 0 {
            return false;
        }

        let mut num = num;

        while num % 5 == 0 {
            num /= 5;
        }

        while num % 3 == 0 {
            num /= 3;
        }

        while num % 2 == 0 {
            num /= 2;
        }

        num == 1
    }
}
