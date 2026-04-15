//! Stokvels gRPC client — TODO: implement after proto services are built.

use tonic::transport::Channel;

pub struct StokvelsClient {
    // inner: StokvelsServiceClient<Channel>,
}

impl StokvelsClient {
    pub fn new(_channel: Channel) -> Self {
        Self {}
    }
}
