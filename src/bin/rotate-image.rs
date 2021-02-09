fn main() {
    let mut v = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

    Solution::rotate(&mut v);
    println!("{:?}", v);
    println!("{}", "[[7, 4, 1], [8, 5, 2], [9, 6, 3]]");
}

struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let l = matrix.len();
        for i in 0..=l / 2 {
            for j in i..l - i - 1 {
                let mut next = (i, j);
                for k in 0..=3 {
                    println!("{:?}", next);

                    if k == 0 {
                        next = (i, j);
                    } else if k == 1 {
                        next = (j, l - i - 1);
                    } else if k == 2 {
                        next = (l - i - 1, l - j - 1);
                    } else {
                        next = (l - j - 1, i);
                    }

                    let last = matrix[next.0][next.1];
                    matrix[next.0][next.1] = matrix[i][j];
                    matrix[i][j] = last;
                }
            }
        }
    }
}
