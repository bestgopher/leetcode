fn main() {
    println!("{:?}", Solution::spiral_matrix_iii(5, 4, 0, 2));
    println!("{}", "[[0, 2], [0, 3], [1, 3], [1, 2], [1, 1], [0, 1], [2, 3], [2, 2], [2, 1], [2, 0], [1, 0], [0, 0], [3, 3], [3, 2], [3, 1], [3, 0], [4, 3], [4, 2], [4, 1], [4, 0]]")
}

struct Solution;

impl Solution {
    // 其实就是以(r0, c0)为中心画正方形，每个正方形的边长加2
    pub fn spiral_matrix_iii(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        result.push(vec![r0, c0]);
        let mut start = (r0, c0);

        let mut length = 1; // 起始的边长的一半为1
        while r0 - length >= 0 || r0 + length < r || c0 - length >= 0 || c0 + length < c {
            start.1 += 1;
            while r0 + length > start.0 {
                if Self::check(r, c, start.0, start.1) {
                    result.push(vec![start.0, start.1]);
                }
                start.0 += 1;
            }

            while c0 - length < start.1 {
                if Self::check(r, c, start.0, start.1) {
                    result.push(vec![start.0, start.1]);
                }
                start.1 -= 1;
            }

            while r0 - length < start.0 {
                if Self::check(r, c, start.0, start.1) {
                    result.push(vec![start.0, start.1]);
                }
                start.0 -= 1;
            }

            while c0 + length > start.1 {
                if Self::check(r, c, start.0, start.1) {
                    result.push(vec![start.0, start.1]);
                }
                start.1 += 1;
            }

            if Self::check(r, c, start.0, start.1) {
                result.push(vec![start.0, start.1]);
            }

            length += 1;
        }

        result
    }

    fn check(r: i32, c: i32, r1: i32, c1: i32) -> bool {
        r1 >= 0 && r1 < r && c1 >= 0 && c1 < c
    }
}
