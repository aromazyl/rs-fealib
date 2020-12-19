//
// main.rs
// Copyright (C) 2020 zhangyule <zyl2336709@gmail.com>
// Distributed under terms of the MIT license.
//
#![allow(unknown_lints)]
#![allow(unreadable_literal)]

extern crate futures;
extern crate grpcio;
extern crate grpcio_proto;
#[macro_use]
extern crate log;
extern crate rs_fealib;
use futures::Future;
use futures::sync::oneshot;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use rs_fealib::engine::Engine;
use proto::feature::FeaDef;

struct PredictorService {

}
pub mod feature {
}

fn main() {
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
	}
}
