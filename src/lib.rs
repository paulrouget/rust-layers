// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "layers"]
#![crate_type = "rlib"]

#![allow(raw_pointer_derive)]
#![feature(vec_push_all, iter_arith)]
#![cfg_attr(target_os="linux", feature(owned_ascii_ext))]
#![cfg_attr(target_os="macos", feature(collections))]

extern crate azure;
extern crate euclid;
extern crate libc;
#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate gleam;
extern crate skia;

#[cfg(target_os="macos")]
extern crate core_foundation;
#[cfg(target_os="macos")]
extern crate io_surface;
#[cfg(target_os="macos")]
extern crate collections;
#[cfg(target_os="macos")]
extern crate cgl;

#[cfg(target_os="linux")]
extern crate x11;
#[cfg(target_os="linux")]
extern crate glx;

#[cfg(target_os="android")]
extern crate egl;

pub mod color;
pub mod geometry;
pub mod layers;
pub mod rendergl;
pub mod scene;
pub mod texturegl;
pub mod tiling;
pub mod util;

pub mod platform {
    #[cfg(target_os="linux")]
    pub mod linux {
        pub mod surface;
    }
    #[cfg(target_os="macos")]
    pub mod macos {
        pub mod surface;
    }
    #[cfg(target_os="android")]
    pub mod android {
        pub mod surface;
    }
    pub mod surface;
}

