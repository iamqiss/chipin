//! Fair_Score gRPC client — TODO: implement after proto services are built.

use tonic::transport::Channel;

pub struct FairScoreClient {
    // inner: FairScoreServiceClient<Channel>,
}

impl FairScoreClient {
    pub fn new(_channel: Channel) -> Self {
        Self {}
    }
}
