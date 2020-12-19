//
// engine.rs
// Copyright (C) 2020 zhangyule <zyl2336709@gmail.com>
// Distributed under terms of the MIT license.
//

pub trait Engine {
	fn new(conf: &String) -> Self;
	fn load_model(&mut self, conf: &String);
	fn save_model(&self, conf: &String);
	fn predict(&self, ins: &Vec<u64>) -> f32;
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
	}
}
