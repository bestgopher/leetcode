fn main() {
    assert_eq!(3, Solution::num_rescue_boats(vec![3, 2, 2, 1], 3));
}

struct Solution;

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort();
        let (mut a, mut b, mut count) = (0usize, people.len() - 1, 0);

        while a <= b {
            if a == b {
                count += 1;
                break;
            }

            if people[a] + people[b] <= limit {
                count += 1;
                a += 1;
                b -= 1;
            } else {
                count += 1;
                b -= 1;
            }
        }

        count
    }
}
