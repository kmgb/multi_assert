macro_rules! multi_assert {
    ($( $x:expr ),+) => { 
        $(
            assert!($x);
        )*
    }
}

fn main() {
    multi_assert!(true, 1 != 0); // Expands to assert(true);assert(1 != 0);

    let (a, b) = (3, 2);

    multi_assert!(
        a != b,
        b != a,
        a != 0,
        b != 0,
        1 != 1 // Err: thread 'main' panicked at 'assertion failed: 1 != 1'
    );

}
