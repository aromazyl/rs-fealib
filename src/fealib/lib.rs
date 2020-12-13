//
// lib.rs
// Copyright (C) 2020 zhangyule <zyl2336709@gmail.com>
// Distributed under terms of the MIT license.
//

pub mod config;
pub mod fe_extract;

#[path = "./extract_impls/combine.rs"]
pub mod combine;

#[path = "./extract_impls/direct.rs"]
pub mod direct;

#[path = "./extract_impls/ext_ins.rs"]
pub mod ext_ins;

#[macro_use]
extern crate lazy_static;

extern crate regex;
