// ssdeep-rs: A Rust wrapper for ssdeep.
//
// Copyright (c) 2016 Petr Zemek <s3rvac@petrzemek.net>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

//! A Rust wrapper for [ssdeep by Jesse
//! Kornblum](https://ssdeep-project.github.io/ssdeep/), which is a C library
//! for computing [context triggered piecewise
//! hashes](http://dfrws.org/2006/proceedings/12-Kornblum.pdf) (CTPH). Also
//! called fuzzy hashes, CTPH can match inputs that have homologies. Such
//! inputs have sequences of identical bytes in the same order, although bytes
//! in between these sequences may be different in both content and length. In
//! contrast to standard hashing algorithms, CTPH can be used to identify files
//! that are highly similar but not identical.
//!
//! Usage
//! -----
//!
//! To compute the fuzzy hash of the given bytes, use
//! [`hash()`](fn.hash.html):
//! ```
//! extern crate ssdeep;
//!
//! let h = ssdeep::hash(b"Hello there!").unwrap();
//! assert_eq!(h, "3:aNRn:aNRn");
//! ```
//!
//! To obtain the fuzzy hash of the contents of a file, use
//! [`hash_from_file()`](fn.hash_from_file.html):
//! ```
//! let h = ssdeep::hash_from_file("tests/file.txt").unwrap();
//! ```
//!
//! To compare two fuzzy hashes, use [`compare()`](fn.compare.html), which
//! returns an integer between 0 (no match) and 100:
//! ```
//! let h1 = "3:AXGBicFlgVNhBGcL6wCrFQEv:AXGHsNhxLsr2C";
//! let h2 = "3:AXGBicFlIHBGcL6wCrFQEv:AXGH6xLsr2Cx";
//! let score = ssdeep::compare(h1, h2).unwrap();
//! assert_eq!(score, 22);
//! ```
//!
//! Each of these functions returns a
//! [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html), where an
//! error is returned when the underlying C function fails.

extern crate libc;
extern crate libfuzzy_sys as raw;

use libc::c_char;
use std::error;
use std::ffi::CString;
use std::fmt;
use std::path::Path;

/// An enum containing errors that the library might return.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Error returned when a function from the underlying C library fails.
    CFunctionFailed {
        /// Name of the C function.
        name: String,
        /// Return code of the function.
        return_code: i32,
    },
}

impl error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::CFunctionFailed { name, return_code } => {
                write!(
                    f,
                    "ssdeep C function {}() failed with return code {}",
                    name, return_code
                )
            }
        }
    }
}

/// The result type used by the library.
pub type Result<T> = std::result::Result<T, Error>;

/// Computes the match score between two fuzzy hashes.
///
/// Returns a value from 0 to 100 indicating the match score of the two hashes.
/// A match score of zero indicates that the hashes did not match. When an
/// error occurs, it returns [`Error`](enum.Error.html).
///
/// # Examples
///
/// When the hashes are identical, it returns 100:
/// ```
/// let h1 = "3:AXGBicFlgVNhBGcL6wCrFQEv:AXGHsNhxLsr2C";
/// let h2 = "3:AXGBicFlgVNhBGcL6wCrFQEv:AXGHsNhxLsr2C";
/// assert_eq!(ssdeep::compare(h1, h2), Ok(100));
/// ```
///
/// When the hashes are similar, it returns a positive integer:
/// ```
/// let h1 = "3:AXGBicFlgVNhBGcL6wCrFQEv:AXGHsNhxLsr2C";
/// let h2 = "3:AXGBicFlIHBGcL6wCrFQEv:AXGH6xLsr2Cx";
/// assert_eq!(ssdeep::compare(h1, h2), Ok(22));
/// ```
///
/// When the hashes have no similarity at all, it returns zero:
/// ```
/// let h1 = "3:u+N:u+N";
/// let h2 = "3:OWIXTn:OWQ";
/// assert_eq!(ssdeep::compare(h1, h2), Ok(0));
/// ```
///
/// When either of the hashes is invalid, it returns an error:
/// ```
/// let h1 = "XYZ";
/// let h2 = "3:tc:u";
/// assert_eq!(
///     ssdeep::compare(h1, h2),
///     Err(ssdeep::Error::CFunctionFailed {
///         name: "fuzzy_compare".to_string(),
///         return_code: -1,
///     })
/// );
///
/// ```
///
/// # Panics
///
/// If either of the hashes contain a null byte. Note that
/// [`hash()`](fn.hash.html) never returns a hash with a null byte, so this may
/// happen only if you handcrafted the hashes or obtained them from other
/// sources.
///
/// # Implementation details
///
/// Internally, it calls the `fuzzy_compare()` function from the underlying C
/// library. The return value `-1` is translated into
/// [`Error`](enum.Error.html).
pub fn compare(hash1: &str, hash2: &str) -> Result<u8> {
    let h1 = str_to_cstring(hash1);
    let h2 = str_to_cstring(hash2);
    let score = unsafe {
        raw::fuzzy_compare(
            h1.as_bytes_with_nul().as_ptr() as *const c_char,
            h2.as_bytes_with_nul().as_ptr() as *const c_char,
        )
    };
    if score == -1 {
        Err(Error::CFunctionFailed {
            name: "fuzzy_compare".to_string(),
            return_code: -1,
        })
    } else {
        Ok(score as u8)
    }
}

/// Computes the fuzzy hash of bytes.
///
/// Returns the fuzzy hash of the given bytes. When an error occurs, it returns
/// [`Error`](enum.Error.html).
///
/// # Examples
///
/// ```
/// let h = ssdeep::hash(b"Hello there!").unwrap();
/// assert_eq!(h, "3:aNRn:aNRn");
/// ```
///
/// # Panics
///
/// * If the length of the bytes is strictly greater than `2^32 - 1` bytes. The
///   reason for this is that the corresponding function from the underlying C
///   library accepts the length of the input buffer as an unsigned 32b
///   integer.
/// * If the function from the underlying C library provides a non-ASCII hash.
///   This would be a bug in the C library.
///
/// # Implementation details
///
/// Internally, it calls the `fuzzy_hash_buf()` function from the underlying C
/// library. A non-zero return value is translated into
/// [`Error`](enum.Error.html).
pub fn hash(buf: &[u8]) -> Result<String> {
    assert!(buf.len() <= u32::max_value() as usize);

    let mut result = create_buffer_for_result();
    let rc = unsafe {
        raw::fuzzy_hash_buf(
            buf.as_ptr(),
            buf.len() as u32,
            result.as_mut_ptr() as *mut c_char,
        )
    };
    result_buffer_to_string("fuzzy_hash_buf", result, rc)
}

/// Computes the fuzzy hash of the contents of a file.
///
/// Returns the fuzzy hash of the given file. When an error occurs, it returns
/// [`Error`](enum.Error.html).
///
/// # Examples
///
/// ```
/// let h = ssdeep::hash_from_file("tests/file.txt").unwrap();
/// assert_eq!(h, "48:9MABzSwnjpDeSrLp8+nagE4f3ZMvcDT0MIhqy6Ic:9XMwnjdeSHS+n5ZfScX0MJ7");
/// ```
///
/// # Panics
///
/// * If the path to the file cannot be converted into a string or it contains
///   a null byte.
/// * If the function from the underlying C library provides a non-ASCII hash.
///   This would be a bug in the C library.
///
/// # Implementation details
///
/// Internally, it calls the `fuzzy_hash_filename()` function from the
/// underlying C library. A non-zero return value is translated into
/// [`Error`](enum.Error.html).
pub fn hash_from_file<P: AsRef<Path>>(file_path: P) -> Result<String> {
    let mut result = create_buffer_for_result();
    let fp = path_as_cstring(file_path);
    let rc = unsafe {
        raw::fuzzy_hash_filename(
            fp.as_bytes_with_nul().as_ptr() as *const c_char,
            result.as_mut_ptr() as *mut c_char,
        )
    };
    result_buffer_to_string("fuzzy_hash_filename", result, rc)
}

fn path_as_cstring<P: AsRef<Path>>(path: P) -> CString {
    // We can unwrap() the result because if the path cannot be converted into
    // a string, we panic, as documented in functions that call this function.
    str_to_cstring(path.as_ref().to_str().unwrap())
}

fn str_to_cstring(s: &str) -> CString {
    // We can unwrap() the result because if there is a null byte, we panic, as
    // documented in functions that call this function.
    CString::new(s).unwrap()
}

fn create_buffer_for_result() -> Vec<u8> {
    // From fuzzy.h: "The buffer into which the fuzzy hash is stored has to be
    // allocated to hold at least FUZZY_MAX_RESULT bytes."
    Vec::with_capacity(raw::FUZZY_MAX_RESULT)
}

fn result_buffer_to_string(libfuzzy_func: &str, mut result: Vec<u8>, rc: i32) -> Result<String> {
    if rc != 0 {
        // The function from libfuzzy failed, so there is no result.
        return Err(Error::CFunctionFailed {
            name: libfuzzy_func.to_string(),
            return_code: rc,
        });
    }

    // Since the resulting vector that holds the fuzzy hash was populated in
    // the underlying C library, we have to adjust its length because at this
    // point, the vector thinks that its length is zero. We do this by finding
    // the first null byte.
    unsafe {
        // Resize the vector to the maximum length before finding the first
        // null byte because slice::get_unchecked() panics when the index is
        // not within the slice. The length will be adjusted shortly.
        result.set_len(raw::FUZZY_MAX_RESULT);

        let mut len = 0;
        for i in 0..raw::FUZZY_MAX_RESULT {
            if *result.get_unchecked(i) == 0 {
                break;
            }
            len += 1;
        }
        result.set_len(len);
    }

    // The result should only be composed of ASCII characters, i.e. the result
    // should be convertible to UTF-8. The presence of non-ASCII character
    // would be a bug in libfuzzy, in which case we panic.
    Ok(String::from_utf8(result).unwrap())
}
