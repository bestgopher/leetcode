fn main() {
    // assert_eq!(3, Solution::num_rescue_boats(vec![3, 2, 2, 1], 3));
    // assert_eq!(1, Solution::num_rescue_boats(vec![2, 2], 6));
    // assert_eq!(2, Solution::num_rescue_boats(vec![3, 1, 7], 7));
    // assert_eq!(5, Solution::num_rescue_boats(vec![1, 3, 4, 3, 3, 5], 5));
    // assert_eq!(5, Solution::num_rescue_boats(vec![1, 3, 4, 3, 3, 5], 5));
    assert_eq!(
        53,
        Solution::num_rescue_boats(
            vec![
                8, 3, 8, 3, 10, 2, 9, 1, 3, 6, 6, 4, 2, 3, 3, 8, 10, 6, 1, 8, 4, 4, 6, 3, 10, 2, 5,
                3, 6, 6, 7, 6, 5, 7, 5, 8, 8, 3, 4, 7, 2, 7, 4, 6, 2, 7, 4, 5, 5, 5, 7, 4, 7, 1, 4,
                8, 1, 7, 1, 5, 9, 1, 6, 1, 9, 7, 8, 7, 1, 1, 7, 10, 9, 7, 8, 3, 8, 3, 2, 5, 4, 2,
                5, 9, 5, 5, 8, 6, 2, 10, 5, 8, 4, 9, 4, 3, 2, 10, 6, 1
            ],
            10
        )
    );
}

struct Solution;

impl Solution {
    pub fn num_rescue_boats1(mut people: Vec<i32>, limit: i32) -> i32 {
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

    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        let mut v = vec![0; (limit + 1) as usize];

        for i in people {
            v[i as usize] += 1;
        }

        let (mut a, mut b, mut count) = (0usize, v.len() - 1, 0);

        while a <= b {
            if a == b {
                if v[a] == 0 {
                    break;
                }

                if a as i32 > limit / 2 {
                    count += v[a];
                } else {
                    count += v[a] / 2 + v[a] % 2;
                }
                break;
            }

            if v[a] == 0 {
                a += 1;
                continue;
            }

            if v[b] == 0 {
                b -= 1;
                continue;
            }

            if a + b <= limit as usize {
                if v[a] < v[b] {
                    count += v[a];
                    v[b] -= v[a];
                    a += 1;
                } else if v[a] > v[b] {
                    count += v[b];
                    v[a] -= v[b];
                    b -= 1;
                } else {
                    count += v[b];
                    b -= 1;
                    a += 1
                }
            } else {
                count += v[b];
                b -= 1;
            }
        }

        count
    }
}
