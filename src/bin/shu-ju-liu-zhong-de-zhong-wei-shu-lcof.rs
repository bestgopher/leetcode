#![allow(dead_code, unused, unused_variables, non_snake_case)]

use std::vec;

fn main() {
    let mut finder = MedianFinder::new();
    finder.add_num(1);
    finder.add_num(2);

    println!("{}", finder.find_median());

    finder.add_num(3);

    println!("{}", finder.find_median());
}

enum Type {
    Big,
    Small,
}

struct Heap {
    data: Vec<i32>,
    r#type: Type,
}

impl Heap {
    fn get_top(&self) -> i32 {
        self.data[0]
    }

    fn insert(&mut self, value: i32) {
        self.data.push(value);
        self.heapify();
    }

    fn heapify(&mut self) {
        let mut index = self.len() - 1;
        while index != 0 {
            match self.r#type {
                Type::Big if self.data[index] > self.data[(index - 1) / 2] => {
                    self.data.swap(index, (index - 1) / 2);
                }
                Type::Small if self.data[index] < self.data[(index - 1) / 2] => {
                    self.data.swap(index, (index - 1) / 2);
                }
                _ => {
                    return;
                }
            }

            index = (index - 1) / 2;
        }
    }

    fn down_heap(&mut self) {
        let mut index = 0;
        while index < self.len() {
            match self.r#type {
                Type::Big => {
                    let son_index = if index * 2 + 2 < self.len() {
                        if self.data[index * 2 + 2] > self.data[index * 2 + 1] {
                            index * 2 + 2
                        } else {
                            index * 2 + 1
                        }
                    } else if index * 2 + 1 < self.len() {
                        index * 2 + 1
                    } else {
                        return;
                    };

                    if self.data[son_index] > self.data[index] {
                        self.data.swap(son_index, index);
                        index = son_index;
                    } else {
                        return;
                    }
                }
                Type::Small => {
                    let son_index = if index * 2 + 2 < self.len() {
                        if self.data[index * 2 + 2] < self.data[index * 2 + 1] {
                            index * 2 + 2
                        } else {
                            index * 2 + 1
                        }
                    } else if index * 2 + 1 < self.len() {
                        index * 2 + 1
                    } else {
                        return;
                    };

                    if self.data[son_index] < self.data[index] {
                        self.data.swap(son_index, index);
                        index = son_index;
                    } else {
                        return;
                    }
                }
            }
        }
    }

    fn pop(&mut self) -> i32 {
        let last = self.len() - 1;
        self.data.swap(0, last);
        let pop = self.data.remove(last);
        self.down_heap();
        pop
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

struct MedianFinder {
    low: Heap,  // 大顶推存值小的一半
    high: Heap, // 小顶堆村值大的一半
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            low: Heap {
                data: vec![],
                r#type: Type::Big,
            },
            high: Heap {
                data: vec![],
                r#type: Type::Small,
            },
        }
    }

    fn add_num(&mut self, num: i32) {
        // 先把数据插入到 high，再从 high 中弹出一个元素插入到 low
        if self.low.len() == self.high.len() {
            self.low.insert(num);
            let pop = self.low.pop();
            self.high.insert(pop);
        } else {
            self.high.insert(num);
            let pop = self.high.pop();
            self.low.insert(pop);
        }
    }

    fn find_median(&self) -> f64 {
        assert!(self.high.len() + self.low.len() > 0, "empty");

        if self.high.len() != self.low.len() {
            self.high.get_top() as f64
        } else {
            (self.high.get_top() as f64 + self.low.get_top() as f64) / 2f64
        }
    }
}

// /**
//  * Your MedianFinder object will be instantiated and called as such:
//  * let obj = MedianFinder::new();
//  * obj.add_num(num);
//  * let ret_2: f64 = obj.find_median();
//  */
#[test]
fn test_min_heap() {
    let mut min_heap = Heap {
        data: vec![],
        r#type: Type::Small,
    };

    min_heap.insert(10);
    min_heap.insert(1);
    min_heap.insert(3);
    min_heap.insert(9);
    min_heap.insert(8);
    min_heap.insert(2);
    min_heap.insert(7);

    assert_eq!(min_heap.pop(), 1);
    assert_eq!(min_heap.pop(), 2);
    assert_eq!(min_heap.pop(), 3);
    assert_eq!(min_heap.pop(), 7);
    assert_eq!(min_heap.pop(), 8);
    assert_eq!(min_heap.pop(), 9);
    assert_eq!(min_heap.pop(), 10);
}

#[test]
fn test_max_heap() {
    let mut max_heap = Heap {
        data: vec![],
        r#type: Type::Big,
    };

    max_heap.insert(10);
    println!("{:?}", max_heap.data);
    max_heap.insert(1);
    println!("{:?}", max_heap.data);
    max_heap.insert(3);
    println!("{:?}", max_heap.data);
    max_heap.insert(9);
    println!("{:?}", max_heap.data);
    max_heap.insert(8);
    println!("{:?}", max_heap.data);
    max_heap.insert(2);
    println!("{:?}", max_heap.data);
    max_heap.insert(7);
    println!("{:?}", max_heap.data);

    assert_eq!(max_heap.pop(), 10);
    println!("{:?}", max_heap.data);
    assert_eq!(max_heap.pop(), 9);
    println!("{:?}", max_heap.data);
    assert_eq!(max_heap.pop(), 8);
    println!("{:?}", max_heap.data);
    assert_eq!(max_heap.pop(), 7);
    println!("{:?}", max_heap.data);
    assert_eq!(max_heap.pop(), 3);
    println!("{:?}", max_heap.data);
    assert_eq!(max_heap.pop(), 2);
    println!("{:?}", max_heap.data);
    assert_eq!(max_heap.pop(), 1);
    println!("{:?}", max_heap.data);
}
