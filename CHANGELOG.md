# Changelog

## dev

* -

## 0.6.0 (2023-11-14)

* The `compare()` function now returns `Result<u8>` instead of `Result<i8>` as
  the return value is always between 0 and 100.

## 0.5.0 (2023-11-13)

* The `hash()`, `hash_from_file()`, and `compare()` functions now return a
  `Result` instead of `Option`
  ([#4](https://github.com/s3rvac/ssdeep-rs/issues/4)).
* The `compare()` function now accepts `&str` instead of `&[u8]` as `hash()`
  returns `String`, not bytes. This simplifies and unifies the library
  interface.

## 0.4.0 (2023-11-12)

* Updated the used version of the underlying [libfuzzy
  library](https://ssdeep-project.github.io/ssdeep/) to the current `master`
  (last update: 2019-02-21).
* The `configure` script is now executed directly instead of via `sh` when
  building `libfuzzy`.
* Improved the obtaining of the source directory when building `libfuzzy`
  ([#3](https://github.com/s3rvac/ssdeep-rs/issues/3)).
* Added a workaround for forcing `make` not to rebuild any autotools-related
  files when building `libfuzzy`.

## 0.3.0 (2023-11-11)

* Switched from the deprecated [gcc](https://crates.io/crates/gcc) crate to
  the [cc](https://crates.io/crates/cc) crate.
* Updated the used version of the [libc](https://crates.io/crates/libc) crate
  to 0.2 (i.e. dropped the patch version requirement).

## 0.2.0 (2018-04-01)

* Updated the used version of the underlying [libfuzzy
  library](https://ssdeep-project.github.io/ssdeep/) to 2.14.1.
* Updated the used version of the [libc](https://crates.io/crates/libc) crate
  to 0.2.40.
* Updated the used version of the [gcc](https://crates.io/crates/gcc) crate to
  0.3.58.

## 0.1.0 (2016-11-01)

Initial release.
