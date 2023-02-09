/// 构建一个大顶堆，然后去掉第一个元素再构建大顶堆
pub fn sort<T: Ord>(target: &mut [T]) {
    if target.is_empty() {
        return;
    }

    for i in (0..(target.len() - 1) / 2).rev() {
        heapfiy(target, i);
    }

    let mut target = target;

    let len = target.len();
    for i in (0..len).rev() {
        target.swap(0, i); // 堆定与最后一个元素交换

        target = &mut target[0..i];
        heapfiy(target, 0);
    }
}

/// 从 n / 2 开始，因为后半部分一定是叶子结点。
fn heapfiy<T: Ord>(target: &mut [T], mut middle: usize) {
    while middle * 2 + 1 < target.len() {
        let mut max = middle * 2 + 1;
        if middle * 2 + 2 < target.len() && target[max] < target[middle * 2 + 2] {
            max = middle * 2 + 2;
        }

        if target[middle] < target[max] {
            target.swap(max, middle);
            middle = max;
        } else {
            break;
        }
    }
}
