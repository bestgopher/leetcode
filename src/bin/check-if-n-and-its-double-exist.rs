fn main() {}

struct Solution;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut map = std::collections::HashMap::<i32, ()>::new();

        // for (index, value) in arr.iter().enumerate() {
        //     map.insert(*value, index);
        // }
        //
        // for (index, value) in arr.iter().map(|x| x * 2).enumerate() {
        //     if let Some(i) = map.get(&value) {
        //         if *i != index {
        //             return true;
        //         }
        //     }
        // }

        for &i in arr.iter() {

            // 首先判断map中是否存在i的两倍的值，存在则返回true
            if let Some(_) = map.get(&(i * 2)) {
                return true;
            }

            // 然后当i为偶数时，在查看map中是否存在i/2的值
            if i % 2 == 0 {
                if let Some(_) = map.get(&(i / 2)) {
                    return true;
                }
            }

            map.insert(i, ());
        }

        false
    }
}