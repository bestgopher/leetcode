fn main() {
    println!("{}, {}", std::i32::MAX, std::i32::MIN);
}

struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let mut x = x;
        let mut m = 0;
        while m == 0 {
            m = x % 10;
            x /= 10;
        }

        while x != 0 {
            let s = m.overflowing_mul(10);
            if s.1 {
                m = 0;
                break;
            }

            let s = s.0.overflowing_add(x % 10);
            if s.1 {
                m = 0;
                break;
            }

            m = s.0;
            x /= 10;
        }

        m
    }
}
