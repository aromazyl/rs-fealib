//
// main.rs
// Copyright (C) 2020 zhangyule <zyl2336709@gmail.com>
// Distributed under terms of the MIT license.
//
#![allow(unknown_lints)]
#![allow(unreadable_literal)]

use ::dubble::DoubleBuffered;
extern crate futures;
use std::net::SocketAddr;
#[macro_use]
extern crate log;
extern crate rs_fealib;
use rs_fealib::{config::Config, init_log, lr_engine::LrEngine, engine::Engine};
use std::thread;
use std::sync::Arc;
use clap::{Arg, App};
use futures::prelude::*;
use grpc::ServerHandlerContext;
use grpc::ServerRequestSingle;
use grpc::ServerResponseUnarySink;
use rs_fealib::feature::{FeaDef, GalaxyRequest, GalaxyResponse};
use rs_fealib::feature_grpc::{self, Galaxy, GalaxyServer};
use rs_fealib::ext_ins::{ExtIns};
use std::sync::Mutex;
use tls_api_stub::TlsAcceptor;
use std::ptr::null;

struct PredictorService {
	engine: Mutex<LrEngine>,
}

impl Galaxy for PredictorService {
	fn send(&self, _: ServerHandlerContext,
		 req: ServerRequestSingle<GalaxyRequest>,
		 sink: ServerResponseUnarySink<GalaxyResponse>) -> grpc::Result<()> {
		let mut response = GalaxyResponse::new();
		let ref mut engine = *self.engine.lock().unwrap();
		let msg = req.message;
		let score = engine.predict(msg.get_feas(), &msg.get_sid().to_string());
		response.set_scores(score);
		response.set_sid(msg.get_sid().to_string());
		sink.finish(response)
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
								.required(true)).get_matches();

	let yaml_config = matches.value_of("config").unwrap_or("service.yaml").to_string();
	let address = matches.value_of("address").unwrap_or("127.0.0.1:5021").to_string();
    let config = Config::new(yaml_config).unwrap();
	let mut ext_ins_helper = ExtIns::new(&config);
	let mut lr_engine = Mutex::new(LrEngine {
		extor: ext_ins_helper,
		fea_weighteds: DoubleBuffered::default(),
		cur_version: "0".to_string(),
	});
	let mut service = PredictorService {
		engine: lr_engine,
	};
	let ref mut svc = *service.engine.lock().unwrap();
	svc.load_model(&"./lr_model".to_string(), &"0".to_string());
	svc = null;
	let _guard = init_log(None);
	let mut server = grpc::ServerBuilder::<TlsAcceptor>::new();
	server.http.set_addr(address.parse::<SocketAddr>().unwrap());
	server.add_service(GalaxyServer::new_service_def(service));
	let _res = server.build().unwrap();
	loop {
		thread::park();
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
	}
}
