fn main() {}

struct Solution;

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut nums = vec![0; spells.len()];

        let mut potions = potions;
        potions.sort();

        for (i, v) in spells.into_iter().enumerate() {
            let (mut start, mut end) = (0, potions.len() - 1);
            let min = success / v as i64;
            while (start + end) / 2 < potions.len() {
                if potions[(start + end) / 2] as i64 >= min && potions[(start + end) / 2 - 1] as i64 < min {
                    nums[i]+=1;
                    break;
                } else if potions[(start + end) / 2] as i64 < min {
                    start = (start + end) / 2;
                } else if potions[(start + end) / 2] as i64 < min {
                    end = (start + end) / 2;
                }
            }
        }
        nums 
    }
}
