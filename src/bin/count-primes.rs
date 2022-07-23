#![allow(dead_code, unused, unused_variables)]

fn main() {
    assert_eq!(4, Solution::count_primes(10));
}

struct Solution;

impl Solution {
    pub fn count_primes1(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return 0;
        }

        // 0代表是质数，1代表非质数
        let mut v = vec![0; (n - 2) as usize];

        for i in 2..n {
            if v[i as usize - 2] == 1 {
                continue;
            }

            if Self::is_prim(i) {
                v[i as usize - 2] = 0;
                let mut m = i * 2;
                while m < n {
                    v[m as usize - 2] = 1;
                    m *= 2;
                }
            } else {
                v[i as usize - 2] = 1;
            }
        }
        println!("{:?}", v);
        v.into_iter().filter(|x| *x == 0).count() as i32
    }

    fn is_prim(n: i32) -> bool {
        if n == 2 || n == 3 {
            return true;
        }

        let mut i = 3;

        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i += 2;
        }

        true
    }

    pub fn count_primes(n: i32) -> i32 {
        if n == 0 || n == 1 {
            return 0;
        }

        // 0代表是质数，1代表非质数
        let mut v = vec![true; n as usize];
        let mut start = 2usize;
        let mut count = 0;
        while start < n as usize {
            if v[start] {
                count += 1;

                let mut start1 = start * 2;
                while start1 < n as usize {
                    v[start1] = false;
                    start1 += start;
                }
            }
            start += 1;
        }

        count
    }
}
