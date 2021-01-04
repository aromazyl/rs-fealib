//
// build.rs
// Copyright (C) 2020 zhangyule <zyl2336709@gmail.com>
// Distributed under terms of the MIT license.
//
extern crate protoc_rust_grpc;

fn main() {
    // let proto_root = "src/proto";
    protoc_rust_grpc::Codegen::new()
        .out_dir("src/proto")
        .input("src/proto/feature.proto")
        .rust_protobuf(true)
        .run()
        .expect("protoc-rust-grpc");
}
