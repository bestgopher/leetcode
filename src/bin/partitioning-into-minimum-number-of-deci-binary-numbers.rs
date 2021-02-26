fn main() {}

struct Solution;

impl Solution {
    //实际可以分解为：
    // 最大位那一列全是1，其他的补0或者1
    // 例如：82734
    // 第一列全是1
    // 第二列2两个1，其余都是0
    // 第三列7个1，其余是0
    // 第四列3个1，其余是0
    // 第五列4个1，其余是0
    //
    // 1 1 1 1 1
    // 1 1 1 1 1
    // 1 0 1 1 1
    // 1 0 1 0 1
    // 1 0 1 0 0
    // 1 0 1 0 0
    // 1 0 1 0 0
    // 1 0 0 0 0
    //
    // 这样就可以看作找最大位是哪一个数，因此遍历字符串，需要最大数就行了。
    pub fn min_partitions(n: String) -> i32 {
        let mut max = b'0';

        for &i in n.as_bytes().into_iter() {
            max = max.max(i);
            if max == b'9' {
                break;
            }
        }

        (max - b'0') as i32
    }
}
