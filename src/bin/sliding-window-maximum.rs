#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {
    println!(
        "{:?}",
        Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3)
    );
    println!("{:?}", Solution::max_sliding_window(vec![1], 1));
    println!("{:?}", Solution::max_sliding_window(vec![1, -1], 1));
    let a = vec![
        7238, 9932, -7015, 6020, 2596, 6189, -7315, 3176, -7751, 7995, 3970, 7008, 4059, 9310,
        -3655, -8628, 3249, 6132, 9022, 8156, 8970, 7702, -8248, 9130, -1393, -6814, -8441, 9879,
        -2811, 3564, 6491, 8875, -200, 8698, -6756, -5946, 2006, 7604, 7379, -4675, 3323, -544,
        544, 130, -1171, 6535, -6825, 4471, 3580, -1876, -5201, 7337, -3992, -3277, -8251, 5427,
        8989, 4481, -298, 5049, 9762, -4932, -7561, -8209, 1343, 2338, -8612, 5181, 95, 8312, 6140,
        9449, 9283, 5812, 2348, -57, -5351, 4471, 3738, 5256, -1644, -8322, -4507, -6337, 821,
        3626, 3804, 3957, 7675, 2195, 5933, 5699, 545, -3593, -760, 199, -7339, -6963, -8857, 5111,
        -2086, -4285, 5260, -6824, -7696, -3032, -1368, -6605, 2119, 5660, 850, 4834, 3333, 7193,
        6465, 1137, -7826, 3972, -4014, -8963, 6244, -5914, 7196, 8119, 4804, -1212, 4780, -5600,
        8125, -5737, -2363, -5635, 3902, 4423, -3962, 7659, -2802, 9953, 6651, 3794, -7302, 5601,
        -6981, -9579, 6382, -1355, 6387, 8293, -4281, 393, 507, 3554, -85, 6148, 9009, 9994, 3835,
        -8033, -985, -9909, -2869, 1453, -1824, -7902, -5402, -4205, -187, -9707, 7666, 4167, 3762,
        -8791, -1256, 9682, -9714, -597, 6671, -8381, -304, -4242, -5095, 6311, -7830, -1480,
        -6470, 6264, 8859, -4593, 9514, 1430, 5248, 6556, 8422, -8424, -4742, -6497, -3416, -4005,
        -4213, -4945, 6129, 4473, -4092, -6352, 490, -5252, -2591, -5388, 9398, -8349, 3329, -5143,
        -5446, 9031, -6319, -4679, -7013, 867, -705, 7882, 5625, 6763, 954, 897, -2191, 4859,
        -4321, 4058, 2535, -1918, -9012, -2708, 500, -5448, -3478, -6758, -935, 7277, 979, -2030,
        -3152, 9066, -6420, 2590, -7793, -3197, 7510, 8948, -4362, 5464, -981, 4541, -6535, -4853,
        -8182, 4128, -4434, 8901, -1384, 1166, -5818, -5866, 3158, -9958, -5805, -959, 4945, -8665,
        -5298, 8831, 5525, 3577, -2783, 7743, 7145, -1839, -2936, -8183, 978, 2578, -6729, -7782,
        135, 7508, 7847,
    ];
    let b = [
        9932, 9932, 9310, 9310, 9310, 9310, 9310, 9310, 9310, 9879, 9879, 9879, 9879, 9879, 9879,
        9879, 9879, 9879, 9879, 9879, 9879, 9879, 9879, 9879, 9879, 9879, 9879, 9879, 8875, 8875,
        8875, 8875, 8698, 8698, 7604, 7604, 7604, 7604, 8989, 8989, 8989, 8989, 9762, 9762, 9762,
        9762, 9762, 9762, 9762, 9762, 9762, 9762, 9762, 9762, 9762, 9762, 9762, 9762, 9762, 9762,
        9762, 9449, 9449, 9449, 9449, 9449, 9449, 9449, 9449, 9449, 9449, 9449, 9283, 7675, 7675,
        7675, 7675, 7675, 7675, 7675, 7675, 7675, 7675, 7675, 7675, 7675, 7675, 7675, 7675, 5933,
        5933, 5699, 5660, 5660, 5660, 7193, 7193, 7193, 7193, 7193, 7193, 7193, 7193, 7193, 7196,
        8119, 8119, 8119, 8119, 8119, 8125, 8125, 8125, 8125, 8125, 8125, 8125, 8125, 8125, 9953,
        9953, 9953, 9953, 9953, 9953, 9953, 9953, 9953, 9953, 9953, 9953, 9953, 9953, 9953, 9953,
        9953, 9953, 9994, 9994, 9994, 9994, 9994, 9994, 9994, 9994, 9994, 9994, 9994, 9994, 9994,
        9994, 9994, 9994, 9994, 9994, 9994, 9682, 9682, 9682, 9682, 9682, 9682, 9682, 9682, 9682,
        9682, 9682, 9682, 9682, 9682, 9682, 9682, 9682, 9682, 9514, 9514, 9514, 9514, 9514, 9514,
        9514, 9514, 9514, 9514, 9514, 9514, 9514, 9514, 9514, 8422, 9398, 9398, 9398, 9398, 9398,
        9398, 9398, 9398, 9398, 9398, 9398, 9398, 9398, 9398, 9398, 9398, 9398, 9398, 9398, 9031,
        9031, 9031, 9031, 9031, 7882, 7882, 7882, 7882, 7882, 7882, 7277, 7277, 7277, 9066, 9066,
        9066, 9066, 9066, 9066, 9066, 9066, 9066, 9066, 9066, 9066, 9066, 9066, 9066, 9066, 9066,
        9066, 9066, 8948, 8948, 8948, 8948, 8948, 8948, 8901, 8901, 8901, 8901, 8901, 8901, 8901,
        8901, 8901, 8901, 8831, 8831, 8831, 8831, 8831, 8831, 8831, 8831, 8831,
    ];

    let c = Solution::max_sliding_window(a, 19);

    assert_eq!(c, b);
}

struct Solution;

impl Solution {
    /// 使用大顶堆
    pub fn max_sliding_window1(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut heap = vec![];
        for i in 0..k as usize {
            Self::heap_insert(&mut heap, (nums[i], i));
        }

        let mut r = vec![heap[0].0];

        for i in k as usize..nums.len() {
            Self::heap_remove(&mut heap, i - k as usize);
            Self::heap_insert(&mut heap, (nums[i], i));
            r.push(heap[0].0);
        }

        r
    }

    /// 堆插入新的元素
    fn heap_insert(heap: &mut Vec<(i32, usize)>, val: (i32, usize)) {
        heap.push(val);
        let mut i = heap.len() - 1;

        while i > 0 && heap[(i - 1) / 2].0 < heap[i].0 {
            heap.swap(i, (i - 1) / 2);
            i = (i - 1) / 2;
        }
    }

    /// 堆移除元素中，下标大于k的数
    /// 先找到对应的元素
    fn heap_remove(heap: &mut Vec<(i32, usize)>, k: usize) {
        while !heap.is_empty() && heap[0].1 <= k {
            let last = heap.len() - 1;
            heap.swap(0, last);
            heap.pop();

            let mut i = 0;
            loop {
                match (heap.get(2 * i + 1), heap.get(2 * i + 2)) {
                    (Some(&left), Some(&right)) => {
                        if left.0 >= heap[i].0 && right.0 >= heap[i].0 {
                            if left.0 > right.0 {
                                heap.swap(i, i * 2 + 1);
                                i = i * 2 + 1;
                            } else {
                                heap.swap(i, i * 2 + 2);
                                i = i * 2 + 2;
                            }
                        } else if left.0 >= heap[i].0 {
                            heap.swap(i, i * 2 + 1);
                            i = i * 2 + 1;
                        } else if right.0 >= heap[i].0 {
                            heap.swap(i, i * 2 + 2);
                            i = i * 2 + 2;
                        } else {
                            break;
                        }
                    }
                    (Some(&left), None) if left.0 >= heap[i].0 => {
                        heap.swap(i, 2 * i + 1);
                        i = i * 2 + 1;
                    }

                    (None, Some(&right)) if right.0 >= heap[i].0 => {
                        heap.swap(i, 2 * i + 2);
                        i = i * 2 + 2;
                    }
                    _ => break,
                }
            }
        }
    }

    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut stack = vec![];
        stack.push(0usize);
        for i in 1..k as usize {
            while !stack.is_empty() && nums[stack[stack.len() - 1]] <= nums[i] {
                stack.pop();
            }
            stack.push(i);
        }
        let mut r = vec![nums[stack[0]]];

        for i in k as usize..nums.len() {
            if i - k as usize + 1 > stack[0] {
                stack.remove(0);
            }

            while !stack.is_empty() && nums[stack[stack.len() - 1]] <= nums[i] {
                stack.pop();
            }
            stack.push(i);

            r.push(nums[stack[0]]);
        }

        r
    }
}
