fn main() {
    println!("{:?}", Solution::combination_sum3(3, 9));
    println!("{:?}", Solution::combination_sum3(3, 7));
}

struct Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut v = vec![];

        for i in 1..9 {
            v.append(Self::func(i, k, n).as_mut());
        }

        v
    }

    fn func(start: i32, k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut v = vec![];

        if k <= 0 || n <= 0 {
            return v;
        }

        if k == 1 {
            if n == start {
                v.push(vec![start]);
            }
            return v;
        }

        for i in start + 1..=9 {
            for mut j in Self::func(i, k - 1, n - start) {
                let mut v1 = Vec::with_capacity(k as usize);
                v1.push(start);
                v1.append(j.as_mut());
                v.push(v1);
            }
        }

        v
    }
}
