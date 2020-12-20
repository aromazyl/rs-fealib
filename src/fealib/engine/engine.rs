//
// engine.rs
// Copyright (C) 2020 zhangyule <zyl2336709@gmail.com>
// Distributed under terms of the MIT license.
//
use crate::feature::{MulFeaDef, FeaDef, MulScore};
use crate::config::{Config};
use std::io::{self, BufRead};

pub trait Engine {
	fn load_model(&mut self, conf: &String, version: &String);
	fn predict(&self, ins: &[MulFeaDef]) -> MulScore;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
	}
}
