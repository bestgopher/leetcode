fn main() {
    println!("{}", Solution::reverse_bits(43261596));
    println!("{}", Solution::reverse_bits(4294967293));
}

struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut r = 0u32;

        for a in 0..32u32 {
            if a <= 15 {
                r |= (x & (1 << a)) << (31 - 2 * a);
            } else {
                r |= (x & (1 << a)) >> (31 - 2 * (31 - a));
            }
        }

        r
    }
}
