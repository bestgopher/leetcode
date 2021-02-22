fn main() {}

struct Solution;

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut a = vec![0; k as usize];
        for &i in arr.iter() {
            let mut index = i % k;
            if index < 0 {
                index += k;
            }
            a[index as usize] += 1;
        }
        if a[0] % 2 != 0 {
            return false;
        }

        for i in 1..=a.len() / 2 {
            if a[i] != a[a.len() - i] {
                return false;
            }
        }

        true
    }
}
