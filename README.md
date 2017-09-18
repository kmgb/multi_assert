# multi_assert
Compound assert statement macro for Rust

Here's where the magic happens,
```
macro_rules! multi_assert {
    () => ();

    ($x:expr $( , $more:expr )*) => (
        assert!($x);
        multi_assert!( $( $more ),* )
    )
}
```

It allows you to make assert expressions like:
```
multi_assert!(
  engine.is_some(),
  client.is_some(),
  important_int != 0
)
```

Allowing you to combine assertion statements into one while still retaining specificity of the error, which ```&&```ing statements together wouldn't allow.
