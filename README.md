num [![Build Status](https://travis-ci.org/nathanfaucett/rs-num.svg?branch=master)](https://travis-ci.org/nathanfaucett/rs-num)
=====

generic number trait

```rust
extern crate num;
use num::Num;

fn add<T: Num>(a: T, b: T) -> T {
  a + b
}
fn length<T: Num>(x: T, y: T) -> T {
  (x * x + y * y).sqrt()
}
```
