fn main() {
    println!("{}", Solution::coin_change(vec![1, 2, 5], 11));
    println!("{}", Solution::coin_change(vec![1], 1));
    println!("{}", Solution::coin_change(vec![1, 2, 5], 100));
}

struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }

        let mut h = std::collections::HashMap::new();

        Self::f(&coins[..], &mut h, amount)
    }

    fn f(coins: &[i32], h: &mut std::collections::HashMap<i32, i32>, amount: i32) -> i32 {
        let mut num = -1;

        for &i in coins {
            if i > amount {
                continue;
            } else if i == amount {
                return 1;
            }

            let x = if let Some(x) = h.get(&(amount - i)) {
                *x
            } else {
                Self::f(coins, h, amount - i)
            };

            if x == -1 {
                continue;
            }

            if num == -1 {
                num = x + 1;
            } else {
                num = num.min(x + 1);
            }
        }

        h.insert(amount, num);

        num
    }
}
