fn main() {
    assert_eq!(232, Solution::max_score1(vec![11, 49, 100, 20, 86, 29, 72], 4));
}

struct Solution;

impl Solution {
    /// 因为从前或者后取元素，那么剩下的元素必然是连续而且数量为len-k
    /// 因此我们可以定义一个长度len-k的区间，从数组前遍历到后，找到这个区间的和的最小值，剩下的即为最大值
    /// 如果从头开始的话，区间的和为card_points[..k]的和sum1，区间向后移动一位，区间sum1 = sum1+card_points[k]-card_points[0]
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        // 剩余数组的长度
        let other = (card_points.len() - k as usize);
        // 假设剩余数组从0下标开始，和为sum1
        let mut sum1: i32 = card_points[0..other].iter().sum();
        let mut sum = sum1;

        for i in 1..card_points.len() - other + 1 {
            sum = sum + card_points[other + i - 1] - card_points[i - 1];
            if sum1 > sum {
                sum1 = sum;
            }
        }

        card_points.iter().sum::<i32>() - sum1
    }

    /// 正向求解，假设起始和为前面k个元素的和sum
    /// 然后sum-card_points[k-1]+card_points[len(card_points)-1]的和，与sum比较，依次类推，找出最大的和
    pub fn max_score1(card_points: Vec<i32>, k: i32) -> i32 {
        let mut sum = card_points[..k as usize].iter().sum();
        let mut sum1 = sum;

        let mut s = k as usize;
        loop {
            if s == 0 {
                break;
            }

            sum1 = sum1 - card_points[s - 1] + card_points[card_points.len() - (k as usize - s + 1)];
            if sum1 > sum {
                sum = sum1;
            }

            s -= 1;
        }

        sum
    }
}