fn main() {
    let s = vec![3, 2, 3, 2, 2, 3, 1, 1, 1];
    println!("{:?}", Solution::majority_element1(s));

    let s = vec![3, 3, 3, 2, 2, 3, 1, 1, 1];
    println!("{:?}", Solution::majority_element1(s));
}

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let length = (nums.len() / 3) as i32;
        let mut m = std::collections::HashMap::new();

        for &i in nums.iter() {
            m.entry(i)
                .and_modify(|e| {
                    (*e) += 1;
                })
                .or_insert(1);
        }

        m.iter()
            .filter(|(&_x, &y)| y > length)
            .map(|(&x, &_y)| x)
            .collect()
    }

    // 摩尔投票法
    // 此题最多2个元素。证：因为数量要大于1/3，设比例a > 1/3, 当大于1/3数量的元素有3个时，3 * a > 1,与事实不符合。
    pub fn majority_element1(nums: Vec<i32>) -> Vec<i32> {
        let mut v: Vec<Option<i32>> = vec![None, None];

        // 初始化v的下标0和1的数量为1
        let mut index_0_num: usize = 0;
        let mut index_1_num: usize = 0;

        for &i in nums.iter() {
            if let Some(x1) = v[0] {
                if x1 == i {
                    index_0_num += 1;
                    continue;
                }
            }

            if let Some(x1) = v[1] {
                if x1 == i {
                    index_1_num += 1;
                    continue;
                }
            }

            if v[0].is_none() {
                v[0] = Some(i);
                index_0_num += 1;
                continue;
            }

            if v[1].is_none() {
                v[1] = Some(i);
                index_1_num += 1;
                continue;
            }

            index_0_num -= 1;
            index_1_num -= 1;

            if index_0_num == 0 {
                v[0] = None;
            }

            if index_1_num == 0 {
                v[1] = None;
            }
        }

        // 再次初始化v的下标0和1的数量为1
        let mut index_0_num: usize = 0;
        let mut index_1_num: usize = 0;

        for &i in nums.iter() {
            if let Some(x) = v[0] {
                if x == i {
                    index_0_num += 1;
                }
            }

            if let Some(x) = v[1] {
                if x == i {
                    index_1_num += 1;
                }
            }
        }

        v.iter()
            .enumerate()
            .map(|(x, &y)| {
                if x == 0 && index_0_num > nums.len() / 3 {
                    return y;
                }

                if x == 1 && index_1_num > nums.len() / 3 {
                    return y;
                }
                None
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect()
    }
}
