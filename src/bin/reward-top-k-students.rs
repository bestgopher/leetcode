#![allow(dead_code, unused, unused_variables, non_snake_case)]

fn main() {}

struct Solution;

impl Solution {
    pub fn top_students(
        positive_feedback: Vec<String>,
        negative_feedback: Vec<String>,
        report: Vec<String>,
        student_id: Vec<i32>,
        k: i32,
    ) -> Vec<i32> {
        let positive_feedback: std::collections::HashSet<_> =
            positive_feedback.iter().map(|x| x.as_str()).collect();

        let negative_feedback: std::collections::HashSet<_> =
            negative_feedback.iter().map(|x| x.as_str()).collect();

        let mut s = (0..student_id.len())
            .map(|x| {
                let mut score = 0;
                for i in report[x].split(' ').into_iter() {
                    if positive_feedback.contains(i) {
                        score += 3;
                    }
                    if negative_feedback.contains(i) {
                        score -= 1;
                    }
                }

                (score, student_id[x])
            })
            .collect::<Vec<(i32, i32)>>();

        s.sort_unstable_by(|x, y| {
            if x.0 != y.0 {
                y.0.cmp(&x.0)
            } else {
                x.1.cmp(&y.1)
            }
        });

        s.into_iter().take(k as usize).map(|x| x.1).collect()
    }
}
