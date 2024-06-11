# Rust std-lib verification

[![Rust Tests](https://github.com/model-checking/verify-rust-std/actions/workflows/rustc.yml/badge.svg)](https://github.com/model-checking/verify-rust-std/actions/workflows/rustc.yml)
[![Build Book](https://github.com/model-checking/verify-rust-std/actions/workflows/book.yml/badge.svg)](https://github.com/model-checking/verify-rust-std/actions/workflows/book.yml)


This repository is a fork of the official Rust programming
language repository, created solely to verify the Rust standard
library. It should not be used as an alternative to the official
Rust releases.

The goal is to have a verified [Rust standard library](https://doc.rust-lang.org/std/) and prove that it is safe.
1. Contributing to the core mechanism of verifying the rust standard library
2. Creating new techniques to perform scalable verification
3. Apply techniques to verify previously unverified parts of the standard library.

## [Kani](https://github.com/model-checking/kani)

The Kani Rust Verifier is a bit-precise model checker for Rust.
Kani verifies:
* Memory safety (e.g., null pointer dereferences)
* User-specified assertions (i.e `assert!(...)`)
* The absence of panics (eg., `unwrap()` on `None` values)
* The absence of some types of unexpected behavior (e.g., arithmetic overflows).

You can find out more about Kani from the [Kani book](https://model-checking.github.io/kani/) or the [Kani repository on Github](https://github.com/model-checking/kani).

## Contact

For questions, suggestions or feedback, feel free to open an [issue here](https://github.com/model-checking/verify-rust-std/issues).

## Security

See [SECURITY](https://github.com/model-checking/kani/security/policy) for more information.

## License

### Kani

Kani is distributed under the terms of both the MIT license and the Apache License (Version 2.0).
See [LICENSE-APACHE](https://github.com/model-checking/kani/blob/main/LICENSE-APACHE) and [LICENSE-MIT](https://github.com/model-checking/kani/blob/main/LICENSE-MIT) for details.

## Rust

Rust is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0), with portions covered by various BSD-like licenses.

See [the Rust repository](https://github.com/rust-lang/rust) for details.
