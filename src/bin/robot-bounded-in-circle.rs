fn main() {
    assert_eq!(true, Solution::is_robot_bounded(String::from("LLGRL")));
}

struct Solution;

impl Solution {
    /// 只有(x,y)不是原点，并且方向和原来的方向一致，最后才回不去
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut start = (0, 0);
        let mut direction = 0u8; // 当前的方向，0为向前，1为向左，2为向后，3为向右

        for &i in instructions.as_bytes().iter() {
            if i == b'G' {
                if direction == 0 {
                    start.1 += 1;
                } else if direction == 1 {
                    start.0 -= 1;
                } else if direction == 2 {
                    start.1 -= 1;
                } else {
                    start.0 += 1
                }
            } else if i == b'L' {
                direction = (direction + 1) % 4;
            } else {
                direction = (direction - 1) % 4;
            }
        }

        start == (0, 0) || direction != 0
    }
}
