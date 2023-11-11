Changelog
=========

dev
---

* Updated the used version of the underlying [libfuzzy
  library](https://ssdeep-project.github.io/ssdeep/) to the current `master`
  (last update: 2019-02-21).
* The `configure` script is now executed directly instead of via `sh`.
* Improved the obtaining of the source directory when building `libfuzzy`.

0.3.0 (2023-11-11)
------------------

* Switched from the deprecated [gcc](https://crates.io/crates/gcc) crate to
  the [cc](https://crates.io/crates/cc) crate.
* Updated the used version of the [libc](https://crates.io/crates/libc) crate
  to 0.2 (i.e. dropped the patch version requirement).

0.2.0 (2018-04-01)
------------------

* Updated the used version of the underlying [libfuzzy
  library](https://ssdeep-project.github.io/ssdeep/) to 2.14.1.
* Updated the used version of the [libc](https://crates.io/crates/libc) crate
  to 0.2.40.
* Updated the used version of the [gcc](https://crates.io/crates/gcc) crate to
  0.3.58.

0.1.0 (2016-11-01)
------------------

Initial release.
