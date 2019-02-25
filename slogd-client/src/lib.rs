extern crate futures;
extern crate grpcio;
extern crate protobuf;
extern crate slogd_shared;

use std::sync::Arc;

use grpcio::*;

use slogd_shared::protos::rpc_grpc::StructuredLogClient;

pub fn new(host: &str) -> StructuredLogClient {
    let env = Arc::new(EnvBuilder::new().build());
    let channel = ChannelBuilder::new(env).connect(host);

    StructuredLogClient::new(channel)
}