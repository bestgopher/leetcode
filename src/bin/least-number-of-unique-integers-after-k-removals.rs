fn main() {}

struct Solution;

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut h = std::collections::HashMap::new();

        for i in arr {
            h.entry(i).and_modify(|x| *x += 1).or_insert(1);
        }

        let mut s = h.iter().map(|(x, y)| *y).collect::<Vec<i32>>();
        s.sort();

        let (mut l, mut k) = (h.len(), k);

        for i in s {
            if k == 0 {
                break;
            }

            if i <= k {
                k -= i;
                l -= 1;
            } else {
                k -= k;
            }
        }


        l as i32
    }
}
