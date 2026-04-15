//! Wallet gRPC client — TODO: implement after proto services are built.

use tonic::transport::Channel;

pub struct WalletClient {
    // inner: WalletServiceClient<Channel>,
}

impl WalletClient {
    pub fn new(_channel: Channel) -> Self {
        Self {}
    }
}
