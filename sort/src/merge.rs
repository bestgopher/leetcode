/// 把序列分割成n个小块，然后再合并这n个小块
pub fn sort<T: Ord + Clone + std::fmt::Debug>(target: &mut [T]) {
    merge(target);
}

fn merge<T: Ord + Clone + std::fmt::Debug>(target: &mut [T]) {
    if target.is_empty() || target.len() == 1 {
        return;
    }

    if target.len() == 2 {
        if target[0] > target[1] {
            target.swap(0, 1);
        }
        return;
    }
    let len = target.len();

    merge(&mut target[0..len >> 1]);
    merge(&mut target[len >> 1..len]);

    // 合并两段, 使用双指针
    // p3 是 v的索引
    let (mut p1, mut p2, mut p3) = (0, len >> 1, 0);

    let v: Vec<T> = target[0..len >> 1].into_iter().map(|x| x.clone()).collect();

    while p1 < len {
        if p2 < len && p3 < v.len() {
            if target[p2] > v[p3] {
                target[p1] = v[p3].clone();
                p3 += 1;
            } else {
                target[p1] = target[p2].clone();
                p2 += 1;
            }
        } else if p2 < len {
            target[p1] = target[p2].clone();
            p2 += 1;
        } else if p3 < v.len() {
            target[p1] = v[p3].clone();
            p3 += 1;
        } else {
            return;
        }

        p1 += 1;
    }
}
