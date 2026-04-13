//! Auth routes — URL definitions for all auth endpoints.
//!
//! Route map:
//!
//! PUBLIC (no auth required):
//!   POST   /auth/register/step1      → personal details
//!   POST   /auth/register/step2      → contact + password
//!   POST   /auth/register/step3      → language preference
//!   POST   /auth/register/step4      → interests + T&Cs → creates account
//!   POST   /auth/otp/send            → send OTP via SMS + WhatsApp
//!   POST   /auth/otp/verify          → verify OTP → returns otp_token
//!   POST   /auth/signin              → sign in → returns token pair
//!   POST   /auth/forgot-password     → initiate password reset
//!   POST   /auth/reset-password      → complete password reset
//!   POST   /auth/refresh             → rotate refresh token
//!   GET    /auth/health              → liveness check
//!
//! PROTECTED (valid JWT required):
//!   POST   /auth/signout             → invalidate refresh token
//!   GET    /auth/me                  → current user profile
//!   GET    /auth/sessions            → list active sessions
//!   DELETE /auth/sessions/:id        → revoke a specific session

use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    errors::AppResult,
    extractors::auth_user::{inject_jwt_secret_middleware, AuthUser},
    handlers::auth as auth_handler,
    repositories::users as user_repo,
    routes::AppState,
};

pub fn router(state: AppState) -> Router<AppState> {
    let public_routes = Router::new()
        // ── Registration ──────────────────────────────────────────────────────
        .route("/auth/register/step1", post(auth_handler::register_step1))
        .route("/auth/register/step2", post(auth_handler::register_step2))
        .route("/auth/register/step3", post(auth_handler::register_step3))
        .route("/auth/register/step4", post(auth_handler::register_step4))
        // ── OTP ───────────────────────────────────────────────────────────────
        .route("/auth/otp/send", post(auth_handler::send_otp))
        .route("/auth/otp/verify", post(auth_handler::verify_otp))
        // ── Sign In ───────────────────────────────────────────────────────────
        .route("/auth/signin", post(auth_handler::sign_in))
        // ── Password Reset ────────────────────────────────────────────────────
        .route("/auth/forgot-password", post(auth_handler::forgot_password))
        .route("/auth/reset-password", post(auth_handler::reset_password))
        // ── Token Rotation ────────────────────────────────────────────────────
        .route("/auth/refresh", post(auth_handler::refresh_tokens))
        // ── Health ────────────────────────────────────────────────────────────
        .route("/auth/health", get(auth_handler::health));

    let protected_routes = Router::new()
        // ── Session management ────────────────────────────────────────────────
        .route("/auth/signout", post(auth_handler::sign_out))
        .route("/auth/me", get(me))
        .route("/auth/sessions", get(list_sessions))
        .route("/auth/sessions/:id", delete(revoke_session))
        // Inject JWT secret so AuthUser extractor can validate tokens
        .layer(middleware::from_fn_with_state(
            state,
            inject_jwt_secret_middleware,
        ));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
}

// ── Protected handlers defined here to keep handlers/auth.rs thin ─────────────

/// GET /auth/me
/// Returns the current authenticated user's profile.
async fn me(
    auth: AuthUser,
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    let user = user_repo::find_by_id(&state.db, auth.user_id)
        .await?
        .ok_or(crate::errors::AppError::Unauthorized)?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "user": crate::models::user::UserProfile::from(user)
        })),
    ))
}

/// GET /auth/sessions
/// Lists all active sessions for the current user.
async fn list_sessions(
    auth: AuthUser,
    State(state): State<AppState>,
) -> AppResult<impl IntoResponse> {
    let sessions = user_repo::get_active_sessions(&state.db, auth.user_id).await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "sessions": sessions,
            "count": sessions.len()
        })),
    ))
}

/// DELETE /auth/sessions/:id
/// Revokes a specific session by ID.
/// Users can only revoke their own sessions.
async fn revoke_session(
    auth: AuthUser,
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    user_repo::revoke_session(&state.db, auth.user_id, session_id).await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Session revoked successfully"
        })),
    ))
}
