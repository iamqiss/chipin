//! Auth gRPC client — wraps generated tonic client with ergonomic methods.

use tonic::transport::Channel;
use crate::grpc::channel::with_token;
use crate::grpc::proto::auth::{
    auth_service_client::AuthServiceClient,
    *,
};
use crate::grpc::proto::common::Empty;

pub struct AuthClient {
    inner: AuthServiceClient<Channel>,
}

impl AuthClient {
    pub fn new(channel: Channel) -> Self {
        Self { inner: AuthServiceClient::new(channel) }
    }

    pub async fn register_step1(
        &mut self,
        first_name: &str,
        last_name: &str,
        date_of_birth: &str,
        gender: &str,
    ) -> Result<RegisterStep1Response, tonic::Status> {
        let req = RegisterStep1Request {
            first_name:    first_name.to_string(),
            last_name:     last_name.to_string(),
            date_of_birth: date_of_birth.to_string(),
            gender:        gender.to_string(),
        };
        self.inner.register_step1(req).await.map(|r| r.into_inner())
    }

    pub async fn send_otp(
        &mut self,
        phone: &str,
        purpose: &str,
    ) -> Result<SendOtpResponse, tonic::Status> {
        self.inner.send_otp(SendOtpRequest {
            phone:   phone.to_string(),
            purpose: purpose.to_string(),
        }).await.map(|r| r.into_inner())
    }

    pub async fn verify_otp(
        &mut self,
        phone: &str,
        purpose: &str,
        code: &str,
    ) -> Result<VerifyOtpResponse, tonic::Status> {
        self.inner.verify_otp(VerifyOtpRequest {
            phone:   phone.to_string(),
            purpose: purpose.to_string(),
            code:    code.to_string(),
        }).await.map(|r| r.into_inner())
    }

    pub async fn sign_in(
        &mut self,
        identifier: &str,
        password: &str,
    ) -> Result<AuthTokenResponse, tonic::Status> {
        self.inner.sign_in(SignInRequest {
            identifier: identifier.to_string(),
            password:   password.to_string(),
        }).await.map(|r| r.into_inner())
    }

    pub async fn sign_out(
        &mut self,
        refresh_token: &str,
        access_token: &str,
    ) -> Result<CommonResponse, tonic::Status> {
        let req = with_token(
            tonic::Request::new(SignOutRequest {
                refresh_token: refresh_token.to_string(),
            }),
            access_token,
        );
        self.inner.sign_out(req).await.map(|r| r.into_inner())
    }

    pub async fn refresh_token(
        &mut self,
        refresh_token: &str,
    ) -> Result<TokenPairResponse, tonic::Status> {
        self.inner.refresh_token(RefreshRequest {
            refresh_token: refresh_token.to_string(),
        }).await.map(|r| r.into_inner())
    }

    pub async fn get_me(
        &mut self,
        access_token: &str,
    ) -> Result<UserProfile, tonic::Status> {
        let req = with_token(tonic::Request::new(Empty {}), access_token);
        self.inner.get_me(req).await.map(|r| r.into_inner())
    }
}
