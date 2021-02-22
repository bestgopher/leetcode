fn main() {}

struct Solution;

impl Solution {
    pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        if colsum.iter().sum::<i32>() != upper + lower {
            return Vec::<Vec<i32>>::new();
        }

        let mut v = vec![
            Vec::<i32>::with_capacity(colsum.len()),
            Vec::<i32>::with_capacity(colsum.len()),
        ];
        let mut v1_sum = 0; //第一个vec的和
        let mut v2_sum = 0; //第二个vec的和

        // 当colsum为2时，都加1，
        // 当colsum为1时，先都加给v[0]
        for i in colsum.into_iter() {
            if i == 2 {
                v[0].push(1);
                v[1].push(1);
                v1_sum += 1;
                v2_sum += 1;
            } else if i == 0 {
                v[0].push(0);
                v[1].push(0);
            } else {
                v[0].push(1);
                v[1].push(0);
                v1_sum += 1;
            }
        }

        for i in 0..v[0].len() {
            if v1_sum == upper {
                break;
            }

            if v[0][i] == 1 && v[1][i] == 0 {
                v[0][i] = 0;
                v[1][i] = 1;
                v1_sum -= 1;
            }
        }

        if v1_sum != upper && v2_sum != lower {
            return vec![];
        }

        v
    }
}
