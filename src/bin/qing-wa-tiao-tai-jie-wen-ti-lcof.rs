#![allow(dead_code, unused, unused_variables)]

fn main() {
    println!("{}", Solution::num_ways(2));
    println!("{}", Solution::num_ways(7));
    println!("{}", Solution::num_ways(0));
}

struct Solution;

impl Solution {
    pub fn num_ways1(n: i32) -> i32 {
        let mut v = vec![0; n as usize + 1];
        Self::calc(n, &mut v);
        *v.last().unwrap()
    }

    fn calc(n: i32, map: &mut [i32]) {
        if n <= 1 {
            map[n as usize] = 1;
            return;
        }

        if map[n as usize - 1] == 0 {
            Solution::calc(n - 1, map);
        }

        if map[n as usize - 2] == 0 {
            Solution::calc(n - 2, map);
        }

        map[n as usize] = (map[n as usize - 1] + map[n as usize - 2]) % 1000000007;
    }

    pub fn num_ways(n: i32) -> i32 {
        if n <= 1 {
            return 1;
        }

        let (mut a, mut b) = (1, 1);

        for _ in 2..=n {
            let x = (a + b) % 1000000007;
            a = b;
            b = x;
        }

        b
    }
}
