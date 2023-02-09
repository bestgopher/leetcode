/// 冒泡排序
/// 从第一个元素开始挨个与后面的元素比较，前面大于后面则交换顺序，到最后一个元素一定是最大的那一个。
/// 重复前一步
/// 优化：如果一段连续到元素到列表末尾都没有交换过，说明此段连续元素都是有序的，因此不需要再次遍历比较这一段。
pub fn sort<T: Ord>(target: &mut [T]) {
    let mut i = target.len();

    while i > 0 {
        let mut sort_index = i - 1;
        for j in 0..i - 1 {
            if target[j] > target[j + 1] {
                sort_index = j + 1;
                target.swap(j, j + 1);
            }
        }

        i = sort_index;
    }
}
