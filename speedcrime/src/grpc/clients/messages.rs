//! Messages gRPC client — TODO: implement after proto services are built.

use tonic::transport::Channel;

pub struct MessagesClient {
    // inner: MessagesServiceClient<Channel>,
}

impl MessagesClient {
    pub fn new(_channel: Channel) -> Self {
        Self {}
    }
}
