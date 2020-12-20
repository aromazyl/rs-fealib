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
use futures::prelude::*;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};
use rs_fealib::engine::Engine;
use rs_fealib::feature::{FeaDef, GalaxyRequest, GalaxyResponse};
use rs_fealib::feature_grpc::{self, Galaxy};
use rs_fealib::ext_ins::{ExtIns};

struct PredictorService {
	engine: Box<dyn Engine>,
}

impl Galaxy for PredictorService {
    fn send(&mut self, ctx: RpcContext, req: GalaxyRequest, sink: UnarySink<GalaxyResponse>) {
		let mut response = GalaxyResponse::new();
		response.set_scores(self.engine.predict(req.get_feas()));
		response.set_sid(req.get_sid().to_string());
		let f = sink.success(response.clone()).map_err(move |err| eprintln!("Failed to reply: {:?}", err));
		ctx.spawn(f)
	}
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
