# Rust wrapper for ssdeep

[![docs.rs](https://docs.rs/ssdeep/badge.svg)](https://docs.rs/ssdeep)
[![crates.io](https://img.shields.io/crates/v/ssdeep.svg)](https://crates.io/crates/ssdeep)

A Rust wrapper for [ssdeep by Jesse
Kornblum](https://ssdeep-project.github.io/ssdeep/), which is a C library for
computing [context triggered piecewise
hashes](http://dfrws.org/2006/proceedings/12-Kornblum.pdf) (CTPH). Also called
fuzzy hashes, CTPH can match inputs that have homologies. Such inputs have
sequences of identical bytes in the same order, although bytes in between these
sequences may be different in both content and length. In contrast to standard
hashing algorithms, CTPH can be used to identify files that are highly similar
but not identical. For more details, see [this blog
post](https://blog.petrzemek.net/2016/11/01/computing-context-triggered-piecewise-hashes-in-rust/).

## Installation

Add the following lines into your `Cargo.toml` file:
```
[dependencies]
ssdeep = "0.4.0"
```

Then, when you run `cargo build`, it will automatically get the wrapper's
source code from [crates.io](https://crates.io/), compile the underlying C
library, and build the wrapper. The C library is statically linked into the
wrapper.

The build process is known to work under Linux with GCC. If you have a
different operating system or compiler and the build fails, you can
[submit a pull request](https://github.com/s3rvac/ssdeep-rs/pulls) or [open an
issue](https://github.com/s3rvac/ssdeep-rs/issues).

## Usage

To compute the fuzzy hash of a given buffer, use the `hash()` function:
```rust
extern crate ssdeep;

let h = ssdeep::hash(b"Hello there!").unwrap();
assert_eq!(h, "3:aNRn:aNRn");
```

To obtain the fuzzy hash of a file, use `hash_from_file()`:
```rust
let h = ssdeep::hash_from_file("path/to/file").unwrap();
```

To compare two fuzzy hashes, use `compare()`, which returns an integer between
0 (no match) and 100:

```rust
let h1 = "3:AXGBicFlgVNhBGcL6wCrFQEv:AXGHsNhxLsr2C";
let h2 = "3:AXGBicFlIHBGcL6wCrFQEv:AXGH6xLsr2Cx";
let score = ssdeep::compare(h1, h2).unwrap();
assert_eq!(score, 22);
```

Each of these functions returns an
[`Option`](https://doc.rust-lang.org/std/option/enum.Option.html), where `None`
is returned when the underlying C function fails.

## Documentation

An automatically generated API documentation is available here:

* [latest](https://docs.rs/ssdeep/)
* [0.4.0](https://docs.rs/ssdeep/0.4.0/ssdeep/)
* [0.3.0](https://docs.rs/ssdeep/0.3.0/ssdeep/)
* [0.2.0](https://docs.rs/ssdeep/0.2.0/ssdeep/)
* [0.1.0](https://docs.rs/ssdeep/0.1.0/ssdeep/)

## License

The wrapper's code is licensed under the terms of GPLv3.

This wrapper includes the unchanged source distribution of
[ssdeep](https://github.com/ssdeep-project/ssdeep/) (commit `d8705da60`),
which is compiled and statically linked into the wrapper during build. It is
licensed under GPLv2.
