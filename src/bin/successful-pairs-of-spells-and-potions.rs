fn main() {
    let s = Solution::successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7);
    println!("{s:?}");

    let s = Solution::successful_pairs(vec![3, 1, 2], vec![8, 5, 8], 16);
    println!("{s:?}");
    let s = Solution::successful_pairs(vec![1, 2, 3, 4, 5, 6, 7], vec![1, 2, 3, 4, 5, 6, 7], 25);
    println!("{s:?}");
}

struct Solution;

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut nums = vec![0; spells.len()];

        let mut potions = potions;
        potions.sort();

        for (i, v) in spells.into_iter().enumerate() {
            let (mut start, mut end) = (0, potions.len() - 1);
            let min = if success % v as i64 == 0 {
                success / v as i64
            } else {
                success / v as i64 + 1
            };

            while start <= end && (start + end) / 2 < potions.len() {
                let middile = (start + end) / 2;

                if potions[middile] as i64 >= min {
                    if middile == 0 {
                        nums[i] = potions.len() as i32;
                        break;
                    } else if (potions[middile - 1] as i64) < min {
                        nums[i] = (potions.len() - middile) as i32;
                        break;
                    } else {
                        end = middile;
                    }
                } else {
                    start = middile + 1;
                }
            }
        }
        nums
    }
}
