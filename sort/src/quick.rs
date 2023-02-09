/// 原地排序的快速排序实现
/// 步骤：
///		1.指定一个基准值p(p为s[begin]), begin为0，end为len(s)-1
///		2.首先从后往前遍历，如果遍历的元素大于基准值，则继续往前遍历, end = end-1
///		3.如果从后往前遍历时的值小于基准值，begin的值赋值为s[end], begin = begin + 1
///		4.然后从前往后遍历，遍历值小于或者等于基准值时，继续向后遍历，begin = begin + 1
///		5.当遍历值大于或者等于基准值时，s[end] = s[begin]
///		6.再次重复2-5步，直到begin == end为止
///		7.最后s[begin] = p
///		8.递归快速排序以begin为分界线的两部分序列
pub fn sort<T: Ord + Clone>(target: &mut [T]) {
    if target.len() < 2 {
        return;
    }

    let (p, mut begin, mut end) = (target[0].clone(), 0, target.len() - 1);

    while begin < end {
        while begin < end {
            match p.cmp(&target[end]) {
                std::cmp::Ordering::Greater => {
                    target[begin] = target[end].clone();
                    begin += 1;
                    break;
                }
                _ => end -= 1,
            }
        }

        while begin < end {
            match p.cmp(&target[begin]) {
                std::cmp::Ordering::Less => {
                    target[end] = target[begin].clone();
                    end -= 1;
                    break;
                }
                _ => begin += 1,
            }
        }
    }

    target[begin] = p;

    sort(&mut target[..begin]);
    sort(&mut target[begin + 1..]);
}
