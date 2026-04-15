//! Market gRPC client — TODO: implement after proto services are built.

use tonic::transport::Channel;

pub struct MarketClient {
    // inner: MarketServiceClient<Channel>,
}

impl MarketClient {
    pub fn new(_channel: Channel) -> Self {
        Self {}
    }
}
