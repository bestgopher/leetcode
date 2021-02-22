struct Solution;

impl Solution {
    /// 二分查找
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut start = n;
        let mut num = 0;
        loop {
            // std::thread::sleep(std::time::Duration::from_secs(1));
            let guess_num = start - (start - num) / 2;
            let s = guess(guess_num);
            println!("{}", guess_num);
            if s == -1 {
                // 我猜的较大时
                start = guess_num;
            } else if s == 1 {
                // 我猜的较小时
                num = guess_num;
            } else {
                break guess_num;
            }
        }
    }
}

// static SELECT: i32 = 1702766719;
// static SELECT: i32 = 6;
static SELECT: i32 = 2147483647;

fn main() {
    // let x = unsafe { Solution::guessNumber(2126753390) };
    // let x = unsafe { Solution::guessNumber(10) };
    let x = unsafe { Solution::guessNumber(2147483647) };
    assert_eq!(x, SELECT);
}

unsafe fn guess(num: i32) -> i32 {
    if SELECT > num {
        1
    } else if SELECT < num {
        -1
    } else {
        0
    }
}
