#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    assert_eq!(
        Solution::reconstruct_queue(vec![
            vec![7, 0],
            vec![4, 4],
            vec![7, 1],
            vec![5, 0],
            vec![6, 1],
            vec![5, 2]
        ]),
        vec![
            vec![5, 0],
            vec![7, 0],
            vec![5, 2],
            vec![6, 1],
            vec![4, 4],
            vec![7, 1]
        ]
    )
}

struct Solution;

impl Solution {
    /// 对于最矮的人来说，前面有n个人就说明他的位置是n+1
    /// 因此先按小到大排序，遍历已经寻找到为止的数组
    /// 当出现空位置或者出现高于的人，说明前面可以放一定数量的人。
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;
        let mut v = vec![vec![]; people.len()];
        people.sort();

        for i in people {
            let mut k = i[1];
            let mut index = 0;

            while index < v.len() - 1 {
                if v[index].is_empty() {
                    if k == 0 {
                        break;
                    }
                    k -= 1;
                } else {
                    if v[index][0] >= i[0] {
                        k -= 1;
                    }
                }

                index += 1;
            }

            v[index] = i;
        }

        v
    }
}
