//! gRPC AuthService implementation.
//! Delegates to existing src/services/auth.rs business logic.

use tonic::{Request, Response, Status};
use sqlx::PgPool;
use redis::aio::ConnectionManager;

use crate::config::Config;
use crate::grpc::proto::auth::{
    auth_service_server::AuthService,
    *,
};
use crate::grpc::proto::common::Empty;
use crate::services::auth as auth_svc;
use crate::models::user as user_model;

pub struct AuthServiceImpl {
    db:     PgPool,
    redis:  ConnectionManager,
    config: Config,
}

impl AuthServiceImpl {
    pub fn new(db: PgPool, redis: ConnectionManager, config: Config) -> Self {
        Self { db, redis, config }
    }
}

#[tonic::async_trait]
impl AuthService for AuthServiceImpl {

    async fn register_step1(
        &self,
        request: Request<RegisterStep1Request>,
    ) -> Result<Response<RegisterStep1Response>, Status> {
        let req = request.into_inner();
        let payload = user_model::RegisterStep1Request {
            first_name:    req.first_name,
            last_name:     req.last_name,
            date_of_birth: req.date_of_birth,
            gender:        serde_json::from_str(&format!(""{}"", req.gender))
                .map_err(|_| Status::invalid_argument("Invalid gender"))?,
        };
        let mut redis = self.redis.clone();
        let result = auth_svc::register_step1(&mut redis, payload)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(RegisterStep1Response {
            session_key: result.message.clone(),
            message: "Personal details saved".to_string(),
        }))
    }

    async fn send_otp(
        &self,
        request: Request<SendOtpRequest>,
    ) -> Result<Response<SendOtpResponse>, Status> {
        let req = request.into_inner();
        let purpose = match req.purpose.as_str() {
            "register"       => user_model::OtpPurpose::Register,
            "reset_password" => user_model::OtpPurpose::ResetPassword,
            "withdraw"       => user_model::OtpPurpose::Withdraw,
            _                => return Err(Status::invalid_argument("Invalid OTP purpose")),
        };
        let mut redis = self.redis.clone();
        let result = auth_svc::send_otp(&self.db, &mut redis, &req.phone, purpose, &self.config)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(SendOtpResponse {
            message:            result.message,
            expires_in_seconds: result.expires_in_seconds as u32,
            debug_otp:          result.debug_otp.unwrap_or_default(),
        }))
    }

    async fn verify_otp(
        &self,
        request: Request<VerifyOtpRequest>,
    ) -> Result<Response<VerifyOtpResponse>, Status> {
        let req = request.into_inner();
        let purpose = match req.purpose.as_str() {
            "register"       => user_model::OtpPurpose::Register,
            "reset_password" => user_model::OtpPurpose::ResetPassword,
            "withdraw"       => user_model::OtpPurpose::Withdraw,
            _                => return Err(Status::invalid_argument("Invalid OTP purpose")),
        };
        let mut redis = self.redis.clone();
        let result = auth_svc::verify_otp_code(&mut redis, &req.phone, purpose, &req.code)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(VerifyOtpResponse {
            otp_token: result.otp_token,
            message:   result.message,
        }))
    }

    async fn sign_in(
        &self,
        request: Request<SignInRequest>,
    ) -> Result<Response<AuthTokenResponse>, Status> {
        let req = request.into_inner();
        let payload = user_model::SignInRequest {
            identifier: req.identifier,
            password:   req.password,
        };
        let mut redis = self.redis.clone();
        let result = auth_svc::sign_in(&self.db, &mut redis, payload, None, None, None, &self.config)
            .await
            .map_err(|e| Status::unauthenticated(e.to_string()))?;

        Ok(Response::new(AuthTokenResponse {
            access_token:  result.access_token,
            refresh_token: result.refresh_token,
            user:          Some(map_user(result.user)),
        }))
    }

    async fn sign_out(
        &self,
        request: Request<SignOutRequest>,
    ) -> Result<Response<CommonResponse>, Status> {
        let req = request.into_inner();
        let mut redis = self.redis.clone();
        let result = auth_svc::sign_out(&self.db, &mut redis, &req.refresh_token, &self.config)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CommonResponse { message: result.message }))
    }

    async fn refresh_token(
        &self,
        request: Request<RefreshRequest>,
    ) -> Result<Response<TokenPairResponse>, Status> {
        let req = request.into_inner();
        let mut redis = self.redis.clone();
        let result = auth_svc::refresh_tokens(
            &self.db, &mut redis,
            user_model::RefreshTokenRequest { refresh_token: req.refresh_token },
            &self.config,
        )
        .await
        .map_err(|e| Status::unauthenticated(e.to_string()))?;

        Ok(Response::new(TokenPairResponse {
            access_token:  result.access_token,
            refresh_token: result.refresh_token,
        }))
    }

    async fn get_me(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<UserProfile>, Status> {
        use crate::utils::jwt::AccessClaims;
        let claims = request.extensions().get::<AccessClaims>().cloned()
            .ok_or_else(|| Status::unauthenticated("Not authenticated"))?;

        let user = crate::repositories::users::find_by_id(&self.db, claims.sub)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
            .ok_or_else(|| Status::not_found("User not found"))?;

        Ok(Response::new(map_user(crate::models::user::UserProfile::from(user))))
    }

    // ── Stubs — implement as services are built ────────────────────────────

    async fn register_step2(&self, _: Request<RegisterStep2Request>) -> Result<Response<RegisterStep2Response>, Status> {
        Err(Status::unimplemented("TODO"))
    }
    async fn register_step3(&self, _: Request<RegisterStep3Request>) -> Result<Response<CommonResponse>, Status> {
        Err(Status::unimplemented("TODO"))
    }
    async fn register_step4(&self, _: Request<RegisterStep4Request>) -> Result<Response<AuthTokenResponse>, Status> {
        Err(Status::unimplemented("TODO"))
    }
    async fn forgot_password(&self, _: Request<ForgotPasswordRequest>) -> Result<Response<SendOtpResponse>, Status> {
        Err(Status::unimplemented("TODO"))
    }
    async fn reset_password(&self, _: Request<ResetPasswordRequest>) -> Result<Response<CommonResponse>, Status> {
        Err(Status::unimplemented("TODO"))
    }
    async fn update_profile(&self, _: Request<UpdateProfileRequest>) -> Result<Response<UserProfile>, Status> {
        Err(Status::unimplemented("TODO"))
    }
}

fn map_user(u: crate::models::user::UserProfile) -> UserProfile {
    UserProfile {
        id:              u.id.to_string(),
        phone:           u.phone,
        email:           u.email.unwrap_or_default(),
        full_name:       u.full_name,
        avatar_url:      u.avatar_url.unwrap_or_default(),
        language:        u.language,
        theme:           u.theme,
        is_kyc_verified: u.is_kyc_verified,
        created_at:      u.created_at.to_string(),
    }
}
