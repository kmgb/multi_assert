macro_rules! multi_assert {
    () => ();

    ($x:expr $( , $more:expr )*) => (
        assert!($x);
        multi_assert!( $( $more ),* )
    )
}

fn main() {
    println!("Hello, world!");
    multi_assert!(true, true);

    let (a, b) = (3, 2);

    multi_assert!(
        a != b,
        b != a,
        a != 0,
        b != 0,
        1 != 1 // thread 'main' panicked at 'assertion failed: 1 != 1'
    );
}
