# No-frills PCG32 random number generator implementation

[![Crates.io](https://img.shields.io/crates/v/pcg32)](https://crates.io/crates/pcg32)
[![License](https://img.shields.io/crates/l/pcg32)](https://github.com/LiosK/rust-pcg32/blob/main/LICENSE)

It implements the [PCG32 random number generator] (and really only that).

[PCG32 random number generator]: https://www.pcg-random.org/download.html

```rust
let mut g = pcg32::Pcg32::new(0xff30_6525_39eb_eaa9, 0x315b_fae4_8ade_2146);

assert_eq!(g.generate(), 0xf986_95e1);
assert_eq!(g.generate(), 0x7e39_20e2);
```

This crate is `no_std` compatible.
