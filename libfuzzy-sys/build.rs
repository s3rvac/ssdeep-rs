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

extern crate cc;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let cfg = cc::Build::new();
    let compiler = cfg.get_compiler();
    let src = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    info(&format!("src dir: {}", src.display()));
    info(&format!("dst dir: {}", dst.display()));

    println!("cargo:rustc-link-lib=static=fuzzy");
    println!("cargo:rustc-link-search={}/.libs", dst.display());

    let _ = fs::create_dir(&dst);

    let mut cflags = OsString::new();
    for arg in compiler.args() {
        cflags.push(arg);
        cflags.push(" ");
    }
    // Without -fPIC, build on Linux fails with the following link error:
    // "relocation R_X86_64_32 against `.rodata' can not be used when making a
    // shared object; recompile with -fPIC"
    cflags.push("-fPIC");

    run(Command::new(&src.join("libfuzzy/configure"))
        .arg("--enable-shared=no")
        .arg("--enable-static=yes")
        .env("CFLAGS", cflags)
        .current_dir(&dst));

    run(Command::new("make")
        .arg(&format!("-j{}", env::var("NUM_JOBS").unwrap()))
        .current_dir(&dst));
}

fn run(cmd: &mut Command) {
    info(&format!("running command: {:?}", cmd));
    let status = match cmd.status() {
        Ok(status) => status,
        Err(e) => fail(&format!("failed to execute command: {}", e)),
    };
    if !status.success() {
        fail(&format!("command did not execute successfully: {}", status));
    }
    info(&format!("command finished: {}", status));
}

fn info(msg: &str) {
    println!("INFO: {}", msg);
}

fn fail(reason: &str) -> ! {
    panic!("FAIL: {}\n\nbuild script failed", reason)
}
