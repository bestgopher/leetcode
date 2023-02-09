/// 插入排序，遍历一个数，把它插入到前面合适的位置，其他的数往后移动
pub fn sort<T: Ord>(target: &mut [T]) {
    for mut i in 1..target.len() {
        for j in (0..i).rev() {
            if target[j] > target[i] {
                target.swap(i, j);
                i = j;
            } else {
                break;
            }
        }
    }
}
