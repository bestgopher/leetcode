fn main() {}

struct Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut v = (0, 0);

        for i in moves.bytes() {
            match i {
                b'L' => v.0 -= 1,
                b'R' => v.0 += 1,
                b'U' => v.1 += 1,
                b'D' => v.1 -= 1,
                _ => {}
            }
        }

        v == (0, 0)
    }
}
