//! Fraud gRPC client — TODO: implement after proto services are built.

use tonic::transport::Channel;

pub struct FraudClient {
    // inner: FraudServiceClient<Channel>,
}

impl FraudClient {
    pub fn new(_channel: Channel) -> Self {
        Self {}
    }
}
