//
// build.rs
// Copyright (C) 2020 zhangyule <zyl2336709@gmail.com>
// Distributed under terms of the MIT license.
//
extern crate protoc_grpcio;

fn main() {
    let proto_root = "src/proto";
    protoc_grpcio::compile_grpc_protos(
        &["feature.proto"],
        &[proto_root],
        &proto_root,
        None
    ).expect("Failed to compile gRPC definitions!");
}