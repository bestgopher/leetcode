fn main() {
    assert_eq!(false, Solution::can_reach(vec![3, 0, 2, 1, 2], 2));
}

struct Solution;

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut v = vec![false; arr.len()];
        Self::scan(&arr, &mut v, start)
    }

    fn scan(arr: &Vec<i32>, op: &mut Vec<bool>, start: i32) -> bool {
        if arr[start as usize] == 0 {
            return true;
        }

        op[start as usize] = true;
        if start >= arr[start as usize] && ((arr[start as usize] + start) as usize) < arr.len() {
            if op[(start - arr[start as usize]) as usize] && op[(arr[start as usize] + start) as usize] {
                return false;
            }
        } else if start >= arr[start as usize] && ((arr[start as usize] + start) as usize) >= arr.len() {
            if op[(start - arr[start as usize]) as usize] {
                return false;
            }
        } else if start < arr[start as usize] && ((arr[start as usize] + start) as usize) < arr.len() {
            if op[(arr[start as usize] + start) as usize] {
                return false;
            }
        }


        if start >= arr[start as usize] {
            let r = Self::scan(arr, op, start - arr[start as usize]);
            if r {
                return true;
            }
        }

        if ((arr[start as usize] + start) as usize) < arr.len() {
            let r = Self::scan(arr, op, arr[start as usize] + start);
            if r {
                return true;
            }
        }

        false
    }
}
