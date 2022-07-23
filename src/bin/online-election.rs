#![allow(dead_code, unused, unused_variables)]

fn main() {
    let s = TopVotedCandidate::new(vec![0, 1, 1, 0, 0, 1, 0], vec![0, 5, 10, 15, 20, 25, 30]);
    assert_eq!(s.q(3), 0);
    assert_eq!(s.q(12), 1);
    assert_eq!(s.q(25), 1);
    assert_eq!(s.q(15), 0);
    assert_eq!(s.q(24), 0);
    assert_eq!(s.q(8), 1);

    let s = TopVotedCandidate::new(vec![0, 0, 0, 0, 1], vec![0, 6, 39, 52, 75]);
    assert_eq!(s.q(45), 0);
    assert_eq!(s.q(49), 0);
    assert_eq!(s.q(59), 0);
    assert_eq!(s.q(68), 0);
    assert_eq!(s.q(42), 0);
    assert_eq!(s.q(37), 0);
    assert_eq!(s.q(99), 0);
    assert_eq!(s.q(26), 0);
    assert_eq!(s.q(78), 0);
    assert_eq!(s.q(43), 0);
}

/**
 * Your TopVotedCandidate object will be instantiated and called as such:
 * let obj = TopVotedCandidate::new(persons, times);
 * let ret_1: i32 = obj.q(t);
 */
struct TopVotedCandidate {
    count: Vec<i32>,
    times: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        Self {
            count: Self::build(persons),
            times,
        }
    }

    fn q(&self, t: i32) -> i32 {
        let (mut start, mut end) = (0usize, self.times.len() - 1);
        let mut middle = (start + end) / 2;
        // binary search
        while start < end {
            if self.times[middle] == t || middle == 0 || middle == self.times.len() - 1 {
                break;
            } else if self.times[middle] > t && self.times[middle - 1] < t {
                middle -= 1;
                break;
            } else if self.times[middle] < t && self.times[middle + 1] > t {
                break;
            } else if self.times[middle] > t {
                end = middle;
                middle = (start + end) / 2;
            } else {
                start = middle + 1;
                middle = (start + end) / 2;
            }
        }

        self.count[middle]
    }

    fn build(persons: Vec<i32>) -> Vec<i32> {
        let mut v = Vec::with_capacity(persons.len());
        let mut counts = std::collections::HashMap::new();
        let mut max_count = 0;
        let mut max_value = 0;

        for i in persons {
            let c = counts.entry(i).and_modify(|x| *x += 1).or_insert(1);
            if *c >= max_count {
                v.push(i);
                max_value = i;
                max_count = *c;
            } else {
                v.push(max_value);
            }
        }

        v
    }
}
