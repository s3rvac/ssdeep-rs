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
//! To compute the fuzzy hash of a given buffer, use
//! [`hash()`](fn.hash.html):
//!
//! ```
//! extern crate ssdeep;
//!
//! let h = ssdeep::hash(b"Hello there!").unwrap();
//! assert_eq!(h, "3:aNRn:aNRn");
//! ```
//!
//! If you want to obtain the fuzzy hash of a file, you can use
//! [`hash_from_file()`](fn.hash_from_file.html):
//!
//! ```
//! let h = ssdeep::hash_from_file("tests/file.txt").unwrap();
//! ```
//!
//! To compare two fuzzy hashes, use [`compare()`](fn.compare.html), which
//! returns an integer between 0 (no match) and 100:
//!
//! ```
//! let h1 = b"3:AXGBicFlgVNhBGcL6wCrFQEv:AXGHsNhxLsr2C";
//! let h2 = b"3:AXGBicFlIHBGcL6wCrFQEv:AXGH6xLsr2Cx";
//! let score = ssdeep::compare(h1, h2).unwrap();
//! assert_eq!(score, 22);
//! ```
//!
//! Each of these functions returns an
//! [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html), where
//! `None` is returned when the underlying C function fails.

extern crate libc;
extern crate libfuzzy_sys as raw;

use libc::c_char;
use libc::uint32_t;
use std::ffi::CString;
use std::path::Path;

/// Computes the match score between two fuzzy hashes.
///
/// Returns a value from 0 to 100 indicating the match score of the two hashes.
/// A match score of zero indicates that the hashes did not match. When an
/// error occurs, it returns `None`.
///
/// # Examples
///
/// When the hashes are identical, it returns 100:
///
/// ```
/// let h1 = b"3:AXGBicFlgVNhBGcL6wCrFQEv:AXGHsNhxLsr2C";
/// let h2 = b"3:AXGBicFlgVNhBGcL6wCrFQEv:AXGHsNhxLsr2C";
/// assert_eq!(ssdeep::compare(h1, h2), Some(100));
/// ```
///
/// When the hashes are similar, it returns a positive integer:
///
/// ```
/// let h1 = b"3:AXGBicFlgVNhBGcL6wCrFQEv:AXGHsNhxLsr2C";
/// let h2 = b"3:AXGBicFlIHBGcL6wCrFQEv:AXGH6xLsr2Cx";
/// assert_eq!(ssdeep::compare(h1, h2), Some(22));
/// ```
///
/// When the hashes have no similarity at all, it returns zero:
///
/// ```
/// let h1 = b"3:u+N:u+N";
/// let h2 = b"3:OWIXTn:OWQ";
/// assert_eq!(ssdeep::compare(h1, h2), Some(0));
/// ```
///
/// When either of the hashes is invalid, it returns `None`:
///
/// ```
/// let h1 = b"XYZ";
/// let h2 = b"3:tc:u";
/// assert_eq!(ssdeep::compare(h1, h2), None);
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
/// library. The return value `-1` is translated into `None`.
pub fn compare(hash1: &[u8], hash2: &[u8]) -> Option<i8> {
    let h1 = bytes_to_cstring(hash1);
    let h2 = bytes_to_cstring(hash2);
    let score = unsafe {
        raw::fuzzy_compare(h1.as_bytes_with_nul().as_ptr() as *const c_char,
                           h2.as_bytes_with_nul().as_ptr() as *const c_char)
    };
    if score == -1 {
        None
    } else {
        Some(score as i8)
    }
}

/// Computes the fuzzy hash of a buffer.
///
/// Returns the fuzzy hash of the given buffer. When an error occurs, it
/// returns `None`.
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
/// If the size of the buffer is strictly greater than `2^32 - 1` bytes. The
/// reason for this is that the corresponding function from the underlying C
/// library accepts the length of the buffer as an unsigned 32b integer.
///
/// # Implementation details
///
/// Internally, it calls the `fuzzy_hash_buf()` function from the underlying C
/// library. A non-zero return value is translated into `None`.
pub fn hash(buf: &[u8]) -> Option<String> {
    assert!(buf.len() <= uint32_t::max_value() as usize);

    let mut result = create_buffer_for_result();
    let rc = unsafe {
        raw::fuzzy_hash_buf(buf.as_ptr(),
                            buf.len() as uint32_t,
                            result.as_mut_ptr() as *mut c_char)
    };
    result_buffer_to_string(result, rc)
}

/// Computes the fuzzy hash of a file.
///
/// Returns the fuzzy hash of the given file. When an error occurs, it returns
/// `None`.
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
/// If the path to the file cannot be converted into bytes or it contains a
/// null byte.
///
/// # Implementation details
///
/// Internally, it calls the `fuzzy_hash_filename()` function from the
/// underlying C library. A non-zero return value is translated into `None`.
pub fn hash_from_file<P: AsRef<Path>>(file_path: P) -> Option<String> {
    let mut result = create_buffer_for_result();
    let fp = path_as_cstring(file_path);
    let rc = unsafe {
        raw::fuzzy_hash_filename(fp.as_bytes_with_nul().as_ptr() as *const c_char,
                                 result.as_mut_ptr() as *mut c_char)
    };
    result_buffer_to_string(result, rc)
}

fn path_as_cstring<P: AsRef<Path>>(path: P) -> CString {
    // We can unwrap() the result because if the path cannot be converted into
    // a string, we panic, as documented in functions that call this function.
    bytes_to_cstring(path.as_ref().to_str().unwrap().as_bytes())
}

fn bytes_to_cstring(s: &[u8]) -> CString {
    // We can unwrap() the result because if there is a null byte, we panic, as
    // documented in functions that call this function.
    CString::new(s).unwrap()
}

fn create_buffer_for_result() -> Vec<u8> {
    // From fuzzy.h: "The buffer into which the fuzzy hash is stored has to be
    // allocated to hold at least FUZZY_MAX_RESULT bytes."
    Vec::with_capacity(raw::FUZZY_MAX_RESULT)
}

fn result_buffer_to_string(mut result: Vec<u8>, rc: i32) -> Option<String> {
    if rc != 0 {
        // The function from libfuzzy failed, so there is no result.
        return None;
    }

    // Since the resulting vector that holds the fuzzy hash was populated in
    // the underlying C library, we have to adjust its length because at this
    // point, the vector thinks that its length is zero. We do this by finding
    // the first null byte.
    unsafe {
        let mut len = 0;
        for i in 0..raw::FUZZY_MAX_RESULT {
            if *result.get_unchecked(i) == 0 {
                break;
            }
            len += 1;
        }
        result.set_len(len);
    }

    // There should be only ASCII characters in the result, but better be safe
    // than sorry. If there happens to be anything else, return None.
    String::from_utf8(result).ok()
}
