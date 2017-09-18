# multi_assert
Compound assert statement macro for Rust

Here's where the magic happens:
(accepts a non-zero-length list of expressions and expands them into individual assert statements)
```
macro_rules! multi_assert {
    ($( $x:expr ),+) => { 
        $(
            assert!($x);
        )*
    }
}
```

It allows you to make statements like:
```
multi_assert!(
  engine.is_some(),
  client.is_some(),
  important_int != 0
);
```

Enabling you to combine assertion statements into one while still retaining specificity of the error, which ```&&```'ing statements together wouldn't allow.
