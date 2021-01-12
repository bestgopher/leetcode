fn main() {
    assert_eq!(19, Solution::num_water_bottles(15, 4));
}

struct Solution;

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut s = num_bottles; // 喝的酒数量
        let mut a = num_bottles; // 剩余的空瓶

        loop {
            if a < num_exchange {
                break;
            }

            s += a / num_exchange;
            a = a % num_exchange + a / num_exchange;
        }

        s
    }
}
