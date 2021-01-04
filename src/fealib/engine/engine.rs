//
// engine.rs
// Copyright (C) 2020 zhangyule <zyl2336709@gmail.com>
// Distributed under terms of the MIT license.
//
use crate::feature::{MulFeaDef, MulScore};

pub trait Engine {
	fn load_model(&mut self, conf: &String, version: &String);
	fn predict(&mut self, ins: &[MulFeaDef], sid: &String) -> MulScore;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
	}
}
