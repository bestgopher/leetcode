pub mod bubble;
pub mod heap;
pub mod insertion;
pub mod merge;
pub mod quick;
pub mod selection;

pub use bubble::sort as bubble_sort;
pub use heap::sort as heap_sort;
pub use insertion::sort as insertion_sort;
pub use merge::sort as merge_sort;
pub use quick::sort as quick_sort;
pub use selection::sort as selection_sort;

#[cfg(test)]
mod tests {
    macro_rules! test_case {
        ($($func_name:ident),+) => {
            $(
                #[test]
                fn $func_name() {
                    use super::$func_name as sort;
                    let s = "The Waker type allows for a loose coupling between the reactor-part and the executor-part of a runtime";
                    let mut m: Vec<String> = s.split(" ").map(|x| x.to_string()).collect();
                    let mut m1 = m.clone();
                    sort(&mut m1);
                    m.sort();

                    assert_eq!(m, m1);

                    let mut m = vec![32,12,243,42,321,543,3213,21,33,5443,5,433];
                    let mut m1 = m.clone();
                    sort(&mut m1);
                    m.sort();

                    assert_eq!(m, m1);
                }
            )*
        };
    }

    test_case! {
        bubble_sort,
        quick_sort,
        selection_sort,
        heap_sort,
        insertion_sort,
        merge_sort
    }
}
