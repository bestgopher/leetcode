#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    /// 有多少个零重复，后面就会被截断多少
    /// 然后从截断开始的位置往前回放，遇到0就放2两个就行了。
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let (mut i, mut j) = (0, 0);
        while j <= arr.len() - 1 {
            if arr[i] == 0 {
                j += 1;
            }
            j += 1;
            i += 1;
        }

        j -= 1;
        i -= 1;

        while i >= 0 {
            if j < arr.len() {
                arr[j] = arr[i];
            }

            if arr[i] == 0 {
                j -= 1;
                arr[j] = 0;
            }

            if i == 0 {
                return;
            }

            j -= 1;
            i -= 1;
        }
    }
}
