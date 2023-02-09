/// 从头开始遍历，遍历到最大的元素，与最后一个元素交换
/// 然后又从头开始遍历，直到遍历完成为止
pub fn sort<T: Ord>(target: &mut [T]) {
    for i in (0..target.len()).rev() {
        let mut max = 0;

        for j in 0..=i {
            if target[j] > target[max] {
                max = j;
            }
        }

        target.swap(max, i);
    }
}
