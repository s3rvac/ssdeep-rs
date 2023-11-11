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

extern crate ssdeep;

use ssdeep::compare;
use ssdeep::hash;
use ssdeep::hash_from_file;

//
// compare()
//

#[test]
fn compare_returns_one_hundred_score_when_hashes_are_equal() {
    let h1 = b"3:AXGBicFlgVNhBGcL6wCrFQEv:AXGHsNhxLsr2C";
    let h2 = b"3:AXGBicFlgVNhBGcL6wCrFQEv:AXGHsNhxLsr2C";
    assert_eq!(compare(h1, h2), Some(100));
}

#[test]
fn compare_returns_nonzero_score_when_hashes_are_similar() {
    let h1 = b"3:AXGBicFlgVNhBGcL6wCrFQEv:AXGHsNhxLsr2C";
    let h2 = b"3:AXGBicFlIHBGcL6wCrFQEv:AXGH6xLsr2Cx";
    assert_eq!(compare(h1, h2), Some(22));
}

#[test]
fn compare_returns_zero_when_hashes_are_not_similar() {
    let h1 = b"3:u+N:u+N";
    let h2 = b"3:OWIXTn:OWQ";
    assert_eq!(compare(h1, h2), Some(0));
}

#[test]
fn compare_returns_none_when_hash_is_invalid() {
    let h1 = b"XYZ";
    let h2 = b"3:tc:u";
    assert_eq!(compare(h1, h2), None);
}

#[test]
fn compare_accepts_strs_as_bytes() {
    let h1 = "3:OWR:OWR";
    let h2 = "3:OWR:OWR";
    assert_eq!(compare(h1.as_bytes(), h2.as_bytes()), Some(100));
}

#[test]
fn compare_accepts_strings_as_bytes() {
    let h1 = "3:OWR:OWR".to_string();
    let h2 = "3:OWR:OWR".to_string();
    assert_eq!(compare(h1.as_bytes(), h2.as_bytes()), Some(100));
}

//
// hash()
//

#[test]
fn hash_returns_correct_hash() {
    let h = hash(b"Hello there!").unwrap();
    assert_eq!(h, "3:aNRn:aNRn");
}

#[test]
fn hash_accepts_str_as_bytes() {
    let h = hash("Hello there!".as_bytes()).unwrap();
    assert_eq!(h, "3:aNRn:aNRn");
}

#[test]
fn hash_accepts_string_as_bytes() {
    let h = hash("Hello there!".to_string().as_bytes()).unwrap();
    assert_eq!(h, "3:aNRn:aNRn");
}

//
// hash_from_file()
//

#[test]
fn hash_from_file_returns_correct_hash() {
    let h = hash_from_file("tests/file.txt").unwrap();
    assert_eq!(
        h,
        "48:9MABzSwnjpDeSrLp8+nagE4f3ZMvcDT0MIhqy6Ic:9XMwnjdeSHS+n5ZfScX0MJ7".to_owned()
    );
}
