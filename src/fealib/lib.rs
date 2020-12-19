//
// lib.rs
// Copyright (C) 2020 zhangyule <zyl2336709@gmail.com>
// Distributed under terms of the MIT license.
//

pub mod config;
pub mod fe_extract;

extern crate futures;
extern crate grpcio;
extern crate protobuf;

#[path = "./extract_impls/combine.rs"]
pub mod combine;

#[path = "./extract_impls/direct.rs"]
pub mod direct;

#[path = "./extract_impls/ext_ins.rs"]
pub mod ext_ins;

#[path = "./engine/engine.rs"]
pub mod engine;

#[path = "./proto/feature.rs"]
pub mod feature;

#[path = "./proto/feature_grpc.rs"]
pub mod feature_grpc;


#[macro_use]
extern crate lazy_static;

extern crate regex;

extern crate slog;
extern crate slog_async;
extern crate slog_stdlog;
extern crate slog_scope;
extern crate slog_term;

use std::fs::File;

use self::slog::{Drain, Logger, OwnedKV};
use self::slog_scope::GlobalLoggerGuard;
use self::slog_term::{Decorator, FullFormat, PlainSyncDecorator, TermDecorator};

pub fn init_log(log_file: Option<String>) -> GlobalLoggerGuard {
    fn setup<D: Decorator + Send + 'static>(decorator: D) -> GlobalLoggerGuard {
        let drain = FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();
        let logger = Logger::root(drain, OwnedKV(()));
        let guard = slog_scope::set_global_logger(logger);
        slog_stdlog::init().unwrap();
        guard
    }

    match log_file {
        Some(path) => {
            let file = File::create(path).unwrap();
            setup(PlainSyncDecorator::new(file))
        }
        None => setup(TermDecorator::new().build()),
    }
}