fn main() {
    let mut v = vec![3, 2, 2, 3];
    assert_eq!(2, Solution::remove_element(&mut v, 3));
    println!("{:?}", v);

    let mut v = vec![0, 1, 2, 2, 3, 0, 4, 2];
    assert_eq!(5, Solution::remove_element(&mut v, 2));
    println!("{:?}", v);

    let mut v = vec![2];
    assert_eq!(1, Solution::remove_element(&mut v, 3));
    println!("{:?}", v);

    let mut v = vec![3, 2, 2, 3];
    assert_eq!(2, Solution::remove_element(&mut v, 3));
    println!("{:?}", v);
}

struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut j = 0;

        // 快慢指针
        for i in 0..nums.len() {
            if nums[i] != val {
                let x = nums[j];
                nums[j] = nums[i];
                nums[i] = x;
                j += 1;
            }
        }
        j as i32
    }
}
