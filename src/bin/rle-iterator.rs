fn main() {
    let mut s = RLEIterator::new(vec![3, 8, 0, 9, 2, 5]);
    println!("{}", s.next(2));
    println!("{}", s.next(1));
    println!("{}", s.next(1));
    println!("{}", s.next(2));
}

struct Solution;

#[derive(Debug)]
struct RLEIterator {
    encoding: Vec<i32>,
    index: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RLEIterator {
    fn new(encoding: Vec<i32>) -> Self {
        Self { encoding, index: 0 }
    }

    fn next(&mut self, n: i32) -> i32 {
        let mut n = n;

        for i in self.encoding.iter().skip(self.index).step_by(2) {
            if *i == 0 {
                self.index += 2;
                continue;
            }

            match n.cmp(i) {
                std::cmp::Ordering::Equal => {
                    let r = self.encoding[self.index + 1];
                    self.index += 2;
                    return r;
                }
                std::cmp::Ordering::Greater => {
                    self.index += 2;
                    n -= *i;
                }
                std::cmp::Ordering::Less => {
                    self.encoding[self.index] -= n;
                    return self.encoding[self.index + 1];
                }
            }
        }

        -1
    }
}
