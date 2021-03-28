fn main() {}

struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort();

        let mut result = vec![vec![intervals[0][0], intervals[0][1]]];

        for i in intervals.into_iter().skip(1) {
            if let Some(v) = result.last_mut() {
                if i[0] >= v[0] && i[0] <= v[1] {
                    v[1] = v[1].max(i[1]);
                } else {
                    result.push(i);
                }
            }
        }

        result
    }
}
