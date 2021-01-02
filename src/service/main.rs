//
// main.rs
// Copyright (C) 2020 zhangyule <zyl2336709@gmail.com>
// Distributed under terms of the MIT license.
//
#![allow(unknown_lints)]
#![allow(unreadable_literal)]

use ::dubble::DoubleBuffered;
extern crate futures;
extern crate grpcio;
extern crate grpcio_proto;
#[macro_use]
extern crate log;
extern crate rs_fealib;
use rs_fealib::{config::Config, init_log, lr_engine::LrEngine};
use std::sync::Arc;
use clap::{Arg, App};
use futures::prelude::*;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink, ResourceQuota, ChannelBuilder};
use rs_fealib::engine::Engine;
use rs_fealib::feature::{FeaDef, GalaxyRequest, GalaxyResponse};
use rs_fealib::feature_grpc::{self, Galaxy, create_galaxy};
use rs_fealib::ext_ins::{ExtIns};

#[derive(Clone)]
struct PredictorService {
	engine: LrEngine,
}

impl Galaxy for PredictorService {
    fn send(&mut self, ctx: RpcContext, req: GalaxyRequest, sink: UnarySink<GalaxyResponse>) {
		let mut response = GalaxyResponse::new();
		response.set_scores(self.engine.predict(req.get_feas(), &req.get_sid().to_string()));
		response.set_sid(req.get_sid().to_string());
		let f = sink.success(response.clone()).map_err(move |err| eprintln!("Failed to reply: {:?}", err));
		ctx.spawn(f)
	}
}

fn main() {
	let matches = App::new("Rs-Fealib")
                       .version("1.0")
                       .author("aromazyl")
                       .about("feature extraction library in rust")
                       .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .help("Sets the feature extraction yaml configuration")
							   .required(true))
						.arg(Arg::with_name("address")
								.short("a")
								.long("address")
								.help("Sets the server's ip address")
								.required(true))
						.arg(Arg::with_name("port")
								.short("p")
								.long("port")
								.help("Set the server's port")
							    .required(true)).get_matches();

	let yaml_config = matches.value_of("config").unwrap_or("service.yaml").to_string();
	let address = matches.value_of("address").unwrap_or("127.0.0.1").to_string();
	let port = matches.value_of("port").unwrap_or("5051").parse::<i32>().unwrap();
    let config = Config::new(yaml_config).unwrap();
	let mut ext_ins_helper = ExtIns::new(&config);
	let mut lr_engine = LrEngine {
		extor: ext_ins_helper,
		fea_weighteds: DoubleBuffered::default(),
		cur_version: 0,
	};
	let mut service = PredictorService {
		engine: lr_engine,
	};
	service.engine.load_model(&"./lr_model".to_string(), "0".to_string());
	let _service = create_galaxy(service);
	let _guard = init_log(None);
	let env = Arc::new(Environment::new(1));
	let quota = ResourceQuota::new(Some("ServerQuota")).resize_memory(1024 * 1024);
	let ch_builder = ChannelBuilder::new(env.clone()).set_resource_quota(quota);
	let mut server = ServerBuilder::new(env)
		.register_service(_service)
		.bind(address, port)
		.channel_args(ch_builder.build_args())
		.build()
		.unwrap();
	server.start();
	for (host, port) in server.bind_addrs() {
		info!("listening on {}:{}", host, port);
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
	}
}
