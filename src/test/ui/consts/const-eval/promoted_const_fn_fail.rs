// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(const_fn, const_fn_union)]

#![allow(const_err)]

union Bar {
    a: &'static u8,
    b: usize,
}

const fn bar() -> u8 {
    unsafe {
        // this will error as long as this test
        // is run on a system whose pointers need more
        // than 8 bits
        Bar { a: &42 }.b as u8
    }
}

fn main() {
    let x: &'static u8 = &(bar() + 1); //~ ERROR does not live long enough
    let y = *x;
    unreachable!();
}
