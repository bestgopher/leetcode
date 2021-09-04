use rand::Rng;

fn main() {
    let mut r = RandomizedCollection::new();
    assert!(r.insert(1));
    assert!(!r.insert(1));
    assert!(r.insert(2));
    assert!(r.remove(1));
    println!("{}", r.get_random());
    println!("{}", r.get_random());
    println!("{}", r.get_random());
    println!("{}", rand::thread_rng().gen_range(0..10));
    println!("{}", rand::thread_rng().gen_range(0..10));
    println!("{}", rand::thread_rng().gen_range(0..10));
    println!("{}", rand::thread_rng().gen_range(0..10));
}

/**
 * Your RandomizedCollection object will be instantiated and called as such:
 * let obj = RandomizedCollection::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
struct RandomizedCollection {
    data: Vec<i32>,
    indexes: std::collections::HashMap<i32, std::collections::HashMap<usize, ()>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedCollection {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            data: Vec::new(),
            indexes: std::collections::HashMap::new(),
        }
    }

    /** Inserts a value to the collection. Returns true if the collection did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        let has = self.indexes.get(&val).is_some();
        self.data.push(val);
        let index = self.data.len() - 1;
        self.indexes.entry(val)
            .and_modify(|i| { i.insert(index, ()); })
            .or_insert_with(|| {
                let mut h = std::collections::HashMap::new();
                h.insert(index, ());
                h
            });

        !has
    }

    /** Removes a value from the collection. Returns true if the collection contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        if self.indexes.get(&val).is_none() { return false; }

        let index = {
            let i = *self.indexes.get(&val).unwrap().keys().next().unwrap();
            self.indexes.get_mut(&val).unwrap().remove(&i);
            i
        };
        let l = self.data.len() - 1;
        let last = self.data[l];
        self.data.swap(index, l);

        if l != index {
            self.indexes.entry(last).and_modify(|x| {
                x.remove(&l);
                x.insert(index, ());
            });
        }

        self.data.pop();

        if self.indexes.get(&val).unwrap().is_empty() {
            self.indexes.remove(&val);
        }

        true
    }

    /** Get a random element from the collection. */
    fn get_random(&self) -> i32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.data.len());
        self.data[index]
    }
}
