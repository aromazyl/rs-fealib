// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_GALAXY_SEND: ::grpcio::Method<super::feature::GalaxyRequest, super::feature::GalaxyResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/galaxy.Galaxy/Send",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct GalaxyClient {
    client: ::grpcio::Client,
}

impl GalaxyClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        GalaxyClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn send_opt(&self, req: &super::feature::GalaxyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::feature::GalaxyResponse> {
        self.client.unary_call(&METHOD_GALAXY_SEND, req, opt)
    }

    pub fn send(&self, req: &super::feature::GalaxyRequest) -> ::grpcio::Result<super::feature::GalaxyResponse> {
        self.send_opt(req, ::grpcio::CallOption::default())
    }

    pub fn send_async_opt(&self, req: &super::feature::GalaxyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::feature::GalaxyResponse>> {
        self.client.unary_call_async(&METHOD_GALAXY_SEND, req, opt)
    }

    pub fn send_async(&self, req: &super::feature::GalaxyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::feature::GalaxyResponse>> {
        self.send_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Galaxy {
    fn send(&mut self, ctx: ::grpcio::RpcContext, req: super::feature::GalaxyRequest, sink: ::grpcio::UnarySink<super::feature::GalaxyResponse>);
}

pub fn create_galaxy<S: Galaxy + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_GALAXY_SEND, move |ctx, req, resp| {
        instance.send(ctx, req, resp)
    });
    builder.build()
}
