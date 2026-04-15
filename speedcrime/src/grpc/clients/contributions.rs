//! Contributions gRPC client — TODO: implement after proto services are built.

use tonic::transport::Channel;

pub struct ContributionsClient {
    // inner: ContributionsServiceClient<Channel>,
}

impl ContributionsClient {
    pub fn new(_channel: Channel) -> Self {
        Self {}
    }
}
