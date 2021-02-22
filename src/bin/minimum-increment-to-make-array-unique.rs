fn main() {
    assert_eq!(1, Solution::min_increment_for_unique(vec![1, 2, 2]));
    assert_eq!(
        6,
        Solution::min_increment_for_unique(vec![3, 2, 1, 2, 1, 7])
    );
    assert_eq!(1, Solution::min_increment_for_unique(vec![40000, 40000]));
}

struct Solution;

impl Solution {
    pub fn min_increment_for_unique1(a: Vec<i32>) -> i32 {
        let mut array = vec![0; 40000 * 2];
        let mut count = 0;
        for &i in a.iter() {
            let mut i = i as usize;
            while array[i] != 0 {
                i += 1;
                count += 1;
            }
            array[i] = 1;
        }

        count
    }

    /// 贪心算法在于每个子问题的局部最优解会指向全局最优解。
    /// 显然在对数组排序之后，可以通过保证数组的最后一个元素，
    /// 经过+1操作后比前面所有元素大即可,此时子问题的最优解会收敛于全局最优解
    pub fn min_increment_for_unique(mut a: Vec<i32>) -> i32 {
        a.sort();
        let mut count = 0;
        for i in 1..a.len() {
            if a[i] <= a[i - 1] {
                count += a[i - 1] - a[i] + 1;
                a[i] = a[i - 1] + 1;
            }
        }

        count
    }
}
