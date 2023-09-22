#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn dist_money(money: i32, children: i32) -> i32 {
        if money < children {
            return -1;
        }
        let money = money - children; // 先给个孩子都分一块钱
        let s = children * 7;

        if s == money {
            // 剩下的钱刚好每人7块
            children
        } else if s < money {
            // 剩下的钱大于每人7块，那有个孩子获得每人7块后余下的钱
            children - 1
        } else {
            let mut cnt = children.min(money / 7);
            let money = money - cnt * 7;
            let children = children - cnt;

            if (children == 0 && money > 0) || (children == 1 && money == 3) {
                cnt -= 1;
            }

            cnt
        }
    }
}
