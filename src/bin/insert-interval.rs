#![allow(dead_code, unused, unused_variables)]

fn main() {}

struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        if intervals.len() == 0 {
            result.push(new_interval);
            return result;
        }
        let (mut index0, mut index1) = (new_interval[0], new_interval[1]);

        if intervals[0][0] > new_interval[1] {
            result.push(new_interval);
            result.extend(intervals.into_iter());
            return result;
        }

        if intervals[intervals.len() - 1][1] < new_interval[0] {
            result.extend(intervals.into_iter());
            result.push(new_interval);
            return result;
        }

        let mut flag = false;

        for i in intervals.into_iter() {
            if i[1] < index0 {
                result.push(i);
            } else if i[0] > index1 {
                if !flag {
                    result.push(vec![index0, index1]);
                    flag = true;
                }
                result.push(i);
            } else {
                index0 = i[0].min(index0);
                index1 = i[1].max(index1);

                if !flag {
                    result.push(vec![index0, index1]);
                    flag = true;
                } else {
                    let l = result.len() - 1;
                    result[l][0] = index0;
                    result[l][1] = index1;
                }
            }
        }

        result
    }
}
