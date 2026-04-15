//! gRPC interceptors — JWT auth, logging, rate limiting.

use tonic::{Request, Status};
use crate::utils::jwt::validate_access_token;
use crate::config::Config;

/// JWT auth interceptor.
/// Validates Bearer token in Authorization metadata.
/// Injects user_id and is_kyc_verified into request extensions.
pub fn auth_interceptor(
    config: Config,
) -> impl Fn(Request<()>) -> Result<Request<()>, Status> + Clone {
    move |mut req: Request<()>| {
        let token = req
            .metadata()
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .map(|s| s.to_string());

        match token {
            None => Err(Status::unauthenticated("Missing authorization token")),
            Some(t) => {
                match validate_access_token(&t, &config.jwt_secret) {
                    Err(_) => Err(Status::unauthenticated("Invalid or expired token")),
                    Ok(claims) => {
                        req.extensions_mut().insert(claims);
                        Ok(req)
                    }
                }
            }
        }
    }
}

/// KYC interceptor — rejects requests from unverified users.
pub fn kyc_interceptor(
    mut req: Request<()>,
) -> Result<Request<()>, Status> {
    use crate::utils::jwt::AccessClaims;
    let claims = req.extensions().get::<AccessClaims>().cloned();
    match claims {
        None => Err(Status::unauthenticated("Authentication required")),
        Some(c) if !c.is_kyc_verified => {
            Err(Status::permission_denied("KYC verification required"))
        }
        Some(_) => Ok(req),
    }
}

/// Logging interceptor — traces every gRPC call.
pub fn logging_interceptor(req: Request<()>) -> Result<Request<()>, Status> {
    tracing::info!(
        method = ?req.uri(),
        "gRPC request"
    );
    Ok(req)
}
