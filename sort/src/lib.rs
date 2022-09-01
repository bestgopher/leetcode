pub mod quick;

pub use quick::quick_sort;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort() {
        let s = "The Waker type allows for a loose coupling between the reactor-part and the executor-part of a runtime";
        let mut m: Vec<String> = s.split(" ").map(|x| x.to_string()).collect();
        let mut m1 = m.clone();
        quick_sort(&mut m1);
        m.sort();

        assert_eq!(m, m1);
    }
}
