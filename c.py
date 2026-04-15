#!/usr/bin/env python3
"""
StockFair gRPC Scaffold
Generates:
  - proto/          — shared .proto definitions (source of truth)
  - motherlode/     — tonic gRPC server additions
  - speedcrime/     — tonic gRPC client additions

Architecture:
  speedcrime  ──gRPC (native)──►  motherlode  ──►  Supabase / Redis
  speedcrime  ──gRPC-Web────────►  motherlode  (WASM browser clients)

Run from: StockFair/
"""

import os

FILES = [

# ══════════════════════════════════════════════════════════════════════════════
# PROTO DEFINITIONS (shared source of truth)
# ══════════════════════════════════════════════════════════════════════════════

(
"proto/common.proto",
"""syntax = "proto3";
package stockfair.common;

// ── Pagination ────────────────────────────────────────────────────────────────

message PageRequest {
  uint32 page     = 1;
  uint32 per_page = 2;
}

message PageMeta {
  uint32 page       = 1;
  uint32 per_page   = 2;
  uint32 total      = 3;
  uint32 total_pages = 4;
}

// ── Money ─────────────────────────────────────────────────────────────────────
// All amounts in South African Rand cents (integer) to avoid float precision.

message Money {
  int64  cents    = 1;  // e.g. 50000 = R500.00
  string currency = 2;  // always "ZAR" for now
}

// ── Timestamps ────────────────────────────────────────────────────────────────

message Timestamp {
  int64 seconds = 1;
  int32 nanos   = 2;
}

// ── Empty ─────────────────────────────────────────────────────────────────────

message Empty {}
""",
),

(
"proto/auth.proto",
"""syntax = "proto3";
package stockfair.auth;

import "proto/common.proto";

// ── Service ───────────────────────────────────────────────────────────────────

service AuthService {
  // Registration
  rpc RegisterStep1 (RegisterStep1Request)  returns (RegisterStep1Response);
  rpc RegisterStep2 (RegisterStep2Request)  returns (RegisterStep2Response);
  rpc RegisterStep3 (RegisterStep3Request)  returns (CommonResponse);
  rpc RegisterStep4 (RegisterStep4Request)  returns (AuthTokenResponse);

  // OTP
  rpc SendOtp    (SendOtpRequest)    returns (SendOtpResponse);
  rpc VerifyOtp  (VerifyOtpRequest)  returns (VerifyOtpResponse);

  // Session
  rpc SignIn     (SignInRequest)     returns (AuthTokenResponse);
  rpc SignOut    (SignOutRequest)    returns (CommonResponse);
  rpc RefreshToken (RefreshRequest) returns (TokenPairResponse);

  // Password reset
  rpc ForgotPassword (ForgotPasswordRequest) returns (SendOtpResponse);
  rpc ResetPassword  (ResetPasswordRequest)  returns (CommonResponse);

  // Profile
  rpc GetMe      (stockfair.common.Empty)    returns (UserProfile);
  rpc UpdateProfile (UpdateProfileRequest)   returns (UserProfile);
}

// ── Messages ──────────────────────────────────────────────────────────────────

message UserProfile {
  string id              = 1;
  string phone           = 2;
  string email           = 3;
  string full_name       = 4;
  string avatar_url      = 5;
  string language        = 6;
  string theme           = 7;
  bool   is_kyc_verified = 8;
  string created_at      = 9;
}

message RegisterStep1Request {
  string first_name    = 1;
  string last_name     = 2;
  string date_of_birth = 3; // DD/MM/YYYY
  string gender        = 4;
}

message RegisterStep1Response {
  string session_key = 1;
  string message     = 2;
}

message RegisterStep2Request {
  string session_key      = 1;
  string phone            = 2;
  string email            = 3;
  string password         = 4;
  string confirm_password = 5;
}

message RegisterStep2Response {
  string phone   = 1;
  string message = 2;
}

message RegisterStep3Request {
  string phone    = 1;
  string language = 2;
}

message RegisterStep4Request {
  string          phone          = 1;
  repeated string interests      = 2;
  string          theme          = 3;
  bool            terms_accepted = 4;
}

message SendOtpRequest {
  string phone   = 1;
  string purpose = 2; // register | reset_password | withdraw
}

message SendOtpResponse {
  string message            = 1;
  uint32 expires_in_seconds = 2;
  string debug_otp          = 3; // non-empty in dev only
}

message VerifyOtpRequest {
  string phone   = 1;
  string purpose = 2;
  string code    = 3;
}

message VerifyOtpResponse {
  string otp_token = 1;
  string message   = 2;
}

message SignInRequest {
  string identifier = 1; // phone or email
  string password   = 2;
}

message SignOutRequest {
  string refresh_token = 1;
}

message RefreshRequest {
  string refresh_token = 1;
}

message AuthTokenResponse {
  string      access_token  = 1;
  string      refresh_token = 2;
  UserProfile user          = 3;
}

message TokenPairResponse {
  string access_token  = 1;
  string refresh_token = 2;
}

message ForgotPasswordRequest {
  string phone = 1;
}

message ResetPasswordRequest {
  string phone                = 1;
  string otp_token            = 2;
  string new_password         = 3;
  string confirm_new_password = 4;
}

message UpdateProfileRequest {
  string full_name = 1;
  string email     = 2;
  string language  = 3;
  string theme     = 4;
}

message CommonResponse {
  string message = 1;
}
""",
),

(
"proto/stokvels.proto",
"""syntax = "proto3";
package stockfair.stokvels;

import "proto/common.proto";

service StokvelService {
  rpc CreateStokvel    (CreateStokvelRequest)    returns (Stokvel);
  rpc GetStokvel       (GetStokvelRequest)       returns (Stokvel);
  rpc ListMyStokvals   (ListMyStokvelRequest)    returns (StokvelList);
  rpc JoinStokvel      (JoinRequest)             returns (MembershipResponse);
  rpc LeaveStokvel     (LeaveRequest)            returns (stockfair.common.CommonResponse);
  rpc GetMembers       (GetMembersRequest)       returns (MemberList);
  rpc GetGroupPot      (GetStokvelRequest)       returns (GroupPot);

  // Multi-sig approval
  rpc RequestMultiSig  (MultiSigRequest)         returns (MultiSigApproval);
  rpc ApproveMultiSig  (ApproveMultiSigRequest)  returns (MultiSigApproval);

  // Streaming: real-time contribution updates
  rpc WatchContributions (GetStokvelRequest) returns (stream ContributionEvent);
}

// ── Types ─────────────────────────────────────────────────────────────────────

enum StokvelType {
  GROCERY    = 0;
  ROTATING   = 1;
  BURIAL     = 2;
  INVESTMENT = 3;
  SAVINGS    = 4;
}

enum StokvelStatus {
  ACTIVE = 0;
  PAUSED = 1;
  CLOSED = 2;
}

enum MemberRole {
  MEMBER       = 0;
  CHAIRPERSON  = 1;
  SECRETARY    = 2;
  TREASURER    = 3;
}

message Stokvel {
  string              id                  = 1;
  string              name                = 2;
  StokvelType         type                = 3;
  StokvelStatus       status              = 4;
  stockfair.common.Money contribution_amount = 5;
  int32               contribution_day    = 6;
  int32               max_members         = 7;
  string              created_by          = 8;
  string              created_at          = 9;
  int32               member_count        = 10;
  stockfair.common.Money total_savings    = 11;
  string              next_payout_date    = 12;
  double              progress_pct        = 13;
  bool                is_overdue          = 14;
}

message CreateStokvelRequest {
  string   name                = 1;
  StokvelType type             = 2;
  int64    contribution_cents  = 3;
  int32    contribution_day    = 4;
  int32    max_members         = 5;
  string   rules               = 6;
}

message GetStokvelRequest {
  string stokvel_id = 1;
}

message ListMyStokvelRequest {
  stockfair.common.PageRequest page = 1;
}

message StokvelList {
  repeated Stokvel        stokvels = 1;
  stockfair.common.PageMeta meta   = 2;
}

message JoinRequest {
  string stokvel_id = 1;
  string message    = 2;
}

message MembershipResponse {
  string stokvel_id = 1;
  string user_id    = 2;
  string status     = 3; // pending | active
  string message    = 4;
}

message LeaveRequest {
  string stokvel_id = 1;
}

message Member {
  string     user_id    = 1;
  string     full_name  = 2;
  string     phone      = 3;
  MemberRole role       = 4;
  string     status     = 5;
  int32      position   = 6;  // rotation position
  int32      fair_score = 7;
  string     joined_at  = 8;
}

message GetMembersRequest {
  string stokvel_id = 1;
}

message MemberList {
  repeated Member members = 1;
}

message GroupPot {
  string                 stokvel_id    = 1;
  stockfair.common.Money balance       = 2;
  stockfair.common.Money target        = 3;
  double                 progress_pct  = 4;
  string                 next_payout   = 5;
  int32                  paid_count    = 6;
  int32                  pending_count = 7;
  int32                  overdue_count = 8;
}

message MultiSigRequest {
  string stokvel_id    = 1;
  string action_type   = 2; // withdrawal | bulk_order | rule_change
  bytes  action_payload = 3;
}

message MultiSigApproval {
  string id             = 1;
  string stokvel_id     = 2;
  string action_type    = 3;
  int32  approvals      = 4;
  int32  required       = 5;
  string status         = 6; // pending | approved | rejected
}

message ApproveMultiSigRequest {
  string approval_id = 1;
  bool   approved    = 2;
}

// Streaming event
message ContributionEvent {
  string stokvel_id   = 1;
  string user_id      = 2;
  string full_name    = 3;
  string event_type   = 4; // paid | overdue | pending
  int64  amount_cents = 5;
  string timestamp    = 6;
}

// Re-export CommonResponse for use without full path
message CommonResponse {
  string message = 1;
}
""",
),

(
"proto/contributions.proto",
"""syntax = "proto3";
package stockfair.contributions;

import "proto/common.proto";

service ContributionService {
  rpc Contribute        (ContributeRequest)       returns (Contribution);
  rpc ListContributions (ListContribRequest)       returns (ContributionList);
  rpc GetContribution   (GetContribRequest)        returns (Contribution);
  rpc AutoPaySetup      (AutoPayRequest)           returns (AutoPayResponse);

  // Payouts
  rpc GetPayoutSchedule (GetPayoutRequest)         returns (PayoutSchedule);
  rpc ProcessPayout     (ProcessPayoutRequest)     returns (Payout);
}

enum ContributionStatus {
  PENDING  = 0;
  PAID     = 1;
  OVERDUE  = 2;
  WAIVED   = 3;
}

message Contribution {
  string             id          = 1;
  string             stokvel_id  = 2;
  string             user_id     = 3;
  stockfair.common.Money amount  = 4;
  string             due_date    = 5;
  string             paid_at     = 6;
  ContributionStatus status      = 7;
  stockfair.common.Money fine    = 8;
  string             payment_ref = 9;
}

message ContributeRequest {
  string stokvel_id = 1;
  int64  amount_cents = 2;
}

message ListContribRequest {
  string stokvel_id              = 1;
  stockfair.common.PageRequest page = 2;
}

message ContributionList {
  repeated Contribution          contributions = 1;
  stockfair.common.PageMeta meta             = 2;
}

message GetContribRequest {
  string contribution_id = 1;
}

message AutoPayRequest {
  string stokvel_id  = 1;
  bool   is_enabled  = 2;
}

message AutoPayResponse {
  string stokvel_id = 1;
  bool   is_enabled = 2;
  string message    = 3;
}

message GetPayoutRequest {
  string stokvel_id = 1;
}

message Payout {
  string id           = 1;
  string stokvel_id   = 2;
  string recipient_id = 3;
  stockfair.common.Money amount = 4;
  string scheduled_for = 5;
  string paid_at       = 6;
  string status        = 7;
}

message PayoutSchedule {
  repeated Payout payouts = 1;
}

message ProcessPayoutRequest {
  string payout_id = 1;
}
""",
),

(
"proto/wallet.proto",
"""syntax = "proto3";
package stockfair.wallet;

import "proto/common.proto";

service WalletService {
  rpc GetWallet     (stockfair.common.Empty)   returns (Wallet);
  rpc Deposit       (DepositRequest)            returns (Transaction);
  rpc Withdraw      (WithdrawRequest)           returns (Transaction);
  rpc Transfer      (TransferRequest)           returns (Transaction);
  rpc GetHistory    (HistoryRequest)            returns (TransactionList);
  rpc GetTransaction (GetTxRequest)             returns (Transaction);

  // Streaming: real-time wallet updates
  rpc WatchWallet   (stockfair.common.Empty)   returns (stream WalletEvent);
}

message Wallet {
  string                 id       = 1;
  string                 user_id  = 2;
  stockfair.common.Money balance  = 3;
  string                 updated_at = 4;
}

enum TxType {
  DEPOSIT      = 0;
  WITHDRAWAL   = 1;
  CONTRIBUTION = 2;
  PAYOUT       = 3;
  TRANSFER     = 4;
  REFUND       = 5;
  FEE          = 6;
}

enum TxStatus {
  TX_PENDING   = 0;
  TX_COMPLETED = 1;
  TX_FAILED    = 2;
  TX_REVERSED  = 3;
}

message Transaction {
  string                 id         = 1;
  string                 user_id    = 2;
  TxType                 type       = 3;
  stockfair.common.Money amount     = 4;
  stockfair.common.Money fee        = 5;
  TxStatus               status     = 6;
  string                 reference  = 7;
  string                 stokvel_id = 8;
  string                 created_at = 9;
}

message DepositRequest {
  int64  amount_cents = 1;
  string reference    = 2;
}

message WithdrawRequest {
  int64  amount_cents     = 1;
  string bank_account_id  = 2;
  string pin              = 3; // feature lock PIN if enabled
}

message TransferRequest {
  string to_user_id   = 1;
  int64  amount_cents = 2;
  string description  = 3;
}

message HistoryRequest {
  string                    filter = 1; // all | contributions | payouts | transfers
  stockfair.common.PageRequest page = 2;
}

message TransactionList {
  repeated Transaction          transactions = 1;
  stockfair.common.PageMeta meta           = 2;
}

message GetTxRequest {
  string transaction_id = 1;
}

message WalletEvent {
  string                 event_type = 1; // deposit | withdrawal | contribution | payout
  stockfair.common.Money amount     = 2;
  string                 description = 3;
  string                 timestamp  = 4;
  stockfair.common.Money new_balance = 5;
}
""",
),

(
"proto/market.proto",
"""syntax = "proto3";
package stockfair.market;

import "proto/common.proto";

service MarketService {
  rpc ListRetailers    (stockfair.common.Empty)    returns (RetailerList);
  rpc ListProducts     (ListProductsRequest)        returns (ProductList);
  rpc GetProduct       (GetProductRequest)          returns (Product);
  rpc CreateOrder      (CreateOrderRequest)         returns (GroupOrder);
  rpc VoteOnOrder      (VoteRequest)                returns (GroupOrder);
  rpc GetOrder         (GetOrderRequest)            returns (GroupOrder);
  rpc ListGroupOrders  (ListOrdersRequest)          returns (GroupOrderList);
  rpc GetCollectionCode (GetCodeRequest)            returns (CollectionCode);
}

message Retailer {
  string id        = 1;
  string name      = 2;
  string logo_url  = 3;
  bool   is_active = 4;
}

message RetailerList {
  repeated Retailer retailers = 1;
}

message Product {
  string                 id            = 1;
  string                 retailer_id   = 2;
  string                 retailer_name = 3;
  string                 name          = 4;
  string                 description   = 5;
  string                 image_url     = 6;
  stockfair.common.Money price         = 7;
  string                 unit          = 8;
  int32                  min_quantity  = 9;
  double                 discount_pct  = 10;
  string                 category      = 11;
  bool                   is_available  = 12;
}

message ListProductsRequest {
  string retailer_id = 1;
  string category    = 2;
  string search      = 3;
  stockfair.common.PageRequest page = 4;
}

message ProductList {
  repeated Product              products = 1;
  stockfair.common.PageMeta meta        = 2;
}

message GetProductRequest {
  string product_id = 1;
}

message OrderItem {
  string product_id  = 1;
  int32  quantity    = 2;
  int64  unit_price_cents = 3;
}

message CreateOrderRequest {
  string             stokvel_id  = 1;
  string             retailer_id = 2;
  repeated OrderItem items       = 3;
  string             delivery_option = 4; // collection | delivery
}

message GroupOrder {
  string             id          = 1;
  string             stokvel_id  = 2;
  string             retailer_id = 3;
  repeated OrderItem items       = 4;
  stockfair.common.Money total   = 5;
  string             status      = 6; // pending_votes | approved | processing | fulfilled
  int32              yes_votes   = 7;
  int32              no_votes    = 8;
  int32              votes_needed = 9;
  double             vote_pct    = 10;
}

message VoteRequest {
  string order_id = 1;
  bool   vote     = 2; // true = yes, false = no
}

message GetOrderRequest {
  string order_id = 1;
}

message ListOrdersRequest {
  string stokvel_id = 1;
  stockfair.common.PageRequest page = 2;
}

message GroupOrderList {
  repeated GroupOrder orders = 1;
  stockfair.common.PageMeta meta = 2;
}

message GetCodeRequest {
  string order_id = 1;
}

message CollectionCode {
  string order_id   = 1;
  string code       = 2;
  bool   is_used    = 3;
  string expires_at = 4;
}
""",
),

(
"proto/fair_score.proto",
"""syntax = "proto3";
package stockfair.fairscore;

import "proto/common.proto";

service FairScoreService {
  rpc GetMyScore       (stockfair.common.Empty)    returns (FairScore);
  rpc GetScoreHistory  (ScoreHistoryRequest)        returns (ScoreHistory);
  rpc GetPlatformStats (stockfair.common.Empty)    returns (PlatformStats);
}

message FairScore {
  string id              = 1;
  string user_id         = 2;
  int32  score           = 3;
  string tier            = 4; // building | fair | good | very_good | excellent
  int32  payment_history = 5;
  int32  consistency     = 6;
  int32  group_activity  = 7;
  int32  member_tenure   = 8;
  string updated_at      = 9;
}

message ScoreHistoryRequest {
  int32 months = 1; // last N months
}

message ScoreEntry {
  int32  score      = 1;
  int32  delta      = 2;
  string reason     = 3;
  string recorded_at = 4;
}

message ScoreHistory {
  repeated ScoreEntry entries = 1;
}

message PlatformStats {
  double average_score = 1;
  double top_10_pct    = 2;
  int32  your_percentile = 3;
}
""",
),

(
"proto/messages.proto",
"""syntax = "proto3";
package stockfair.messages;

import "proto/common.proto";

service MessageService {
  rpc ListChats        (stockfair.common.Empty)   returns (ChatList);
  rpc GetMessages      (GetMessagesRequest)         returns (MessageList);
  rpc SendMessage      (SendMessageRequest)         returns (Message);

  // Bi-directional streaming for real-time group chat
  rpc ChatStream       (stream SendMessageRequest) returns (stream Message);

  // Pre-order votes in chat
  rpc CreateOrderVote  (CreateVoteRequest)          returns (OrderVoteCard);
  rpc GetOrderVote     (GetVoteRequest)              returns (OrderVoteCard);
}

enum MessageType {
  TEXT        = 0;
  VOICE_NOTE  = 1;
  SYSTEM      = 2;
  ORDER_VOTE  = 3;
  PAYOUT_NOTICE = 4;
}

message Message {
  string      id         = 1;
  string      stokvel_id = 2;
  string      sender_id  = 3;
  string      sender_name = 4;
  MessageType type       = 5;
  string      content    = 6;
  string      media_url  = 7;
  string      created_at = 8;
  OrderVoteCard vote_card = 9; // only set when type = ORDER_VOTE
}

message GetMessagesRequest {
  string stokvel_id                 = 1;
  stockfair.common.PageRequest page = 2;
}

message MessageList {
  repeated Message               messages = 1;
  stockfair.common.PageMeta meta         = 2;
}

message SendMessageRequest {
  string      stokvel_id = 1;
  MessageType type       = 2;
  string      content    = 3;
  string      media_url  = 4;
}

message Chat {
  string  stokvel_id   = 1;
  string  stokvel_name = 2;
  string  last_message = 3;
  string  last_message_at = 4;
  int32   unread_count = 5;
  bool    has_voice_note = 6;
}

message ChatList {
  repeated Chat chats = 1;
}

message CreateVoteRequest {
  string   stokvel_id  = 1;
  string   order_id    = 2;
}

message GetVoteRequest {
  string order_id = 1;
}

message OrderVoteCard {
  string   order_id     = 1;
  string   retailer_name = 2;
  string   title        = 3;
  repeated string items = 4;
  int64    total_cents  = 5;
  int32    yes_votes    = 6;
  int32    no_votes     = 7;
  int32    votes_needed = 8;
  double   vote_pct     = 9;
  string   status       = 10;
}
""",
),

(
"proto/fraud.proto",
"""syntax = "proto3";
package stockfair.fraud;

import "proto/common.proto";

service FraudService {
  rpc GetFraudSettings    (stockfair.common.Empty)       returns (FraudSettings);
  rpc UpdateFraudSettings (UpdateFraudSettingsRequest)    returns (FraudSettings);
  rpc GetAlerts           (stockfair.common.Empty)       returns (AlertList);
  rpc ResolveAlert        (ResolveAlertRequest)           returns (stockfair.common.Empty);
  rpc GetFeatureLock      (stockfair.common.Empty)       returns (FeatureLock);
  rpc UpdateFeatureLock   (UpdateFeatureLockRequest)      returns (FeatureLock);
}

message FraudSettings {
  bool behavioral_analytics = 1;
  bool sim_swap_detection   = 2;
  bool jailbreak_detection  = 3;
  bool geofencing           = 4;
  string updated_at         = 5;
}

message UpdateFraudSettingsRequest {
  bool behavioral_analytics = 1;
  bool sim_swap_detection   = 2;
  bool jailbreak_detection  = 3;
  bool geofencing           = 4;
}

message FraudAlert {
  string id       = 1;
  string type     = 2;     // sim_swap | geo_anomaly | behavior | jailbreak
  string severity = 3;     // low | medium | high | critical
  string detail   = 4;
  bool   resolved = 5;
  string created_at = 6;
}

message AlertList {
  repeated FraudAlert alerts = 1;
}

message ResolveAlertRequest {
  string alert_id = 1;
}

message FeatureLock {
  bool lock_withdrawals     = 1;
  bool lock_payments        = 2;
  bool lock_statements      = 3;
  bool lock_linked_accounts = 4;
  bool lock_investments     = 5;
}

message UpdateFeatureLockRequest {
  bool lock_withdrawals     = 1;
  bool lock_payments        = 2;
  bool lock_statements      = 3;
  bool lock_linked_accounts = 4;
  bool lock_investments     = 5;
  string current_pin        = 6; // required to change lock settings
}
""",
),

# ══════════════════════════════════════════════════════════════════════════════
# MOTHERLODE — gRPC server additions
# ══════════════════════════════════════════════════════════════════════════════

(
"motherlode/build.rs",
"""fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compile all proto files into Rust code
    // Output goes to OUT_DIR, included via tonic::include_proto!

    tonic_build::configure()
        .build_server(true)
        .build_client(false) // motherlode is server-only
        .compile_protos(
            &[
                "../proto/common.proto",
                "../proto/auth.proto",
                "../proto/stokvels.proto",
                "../proto/contributions.proto",
                "../proto/wallet.proto",
                "../proto/market.proto",
                "../proto/fair_score.proto",
                "../proto/messages.proto",
                "../proto/fraud.proto",
            ],
            &["../"], // include root so imports resolve
        )?;

    Ok(())
}
""",
),

(
"motherlode/src/grpc/mod.rs",
"""//! gRPC module — tonic server wiring.
//!
//! Motherlode runs two servers:
//!   :8080 — existing REST/HTTP (Axum) for backwards compat + health checks
//!   :50051 — gRPC (tonic) for speedcrime native client
//!   :50052 — gRPC-Web (tonic + grpc-web) for WASM/browser clients

pub mod proto;
pub mod server;
pub mod interceptors;
pub mod services;
""",
),

(
"motherlode/src/grpc/proto.rs",
"""//! Generated proto types — included from OUT_DIR by tonic_build.

pub mod common {
    tonic::include_proto!("stockfair.common");
}

pub mod auth {
    tonic::include_proto!("stockfair.auth");
}

pub mod stokvels {
    tonic::include_proto!("stockfair.stokvels");
}

pub mod contributions {
    tonic::include_proto!("stockfair.contributions");
}

pub mod wallet {
    tonic::include_proto!("stockfair.wallet");
}

pub mod market {
    tonic::include_proto!("stockfair.market");
}

pub mod fair_score {
    tonic::include_proto!("stockfair.fairscore");
}

pub mod messages {
    tonic::include_proto!("stockfair.messages");
}

pub mod fraud {
    tonic::include_proto!("stockfair.fraud");
}
""",
),

(
"motherlode/src/grpc/interceptors.rs",
"""//! gRPC interceptors — JWT auth, logging, rate limiting.

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
""",
),

(
"motherlode/src/grpc/server.rs",
"""//! gRPC server builder.
//! Called from main.rs alongside the existing Axum HTTP server.

use std::net::SocketAddr;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;

use crate::config::Config;
use crate::grpc::interceptors::auth_interceptor;
use crate::grpc::services::{
    auth::AuthServiceImpl,
    stokvels::StokvelServiceImpl,
    contributions::ContributionServiceImpl,
    wallet::WalletServiceImpl,
    market::MarketServiceImpl,
    fair_score::FairScoreServiceImpl,
    messages::MessageServiceImpl,
    fraud::FraudServiceImpl,
};
use crate::grpc::proto::{
    auth::auth_service_server::AuthServiceServer,
    stokvels::stokvel_service_server::StokvelServiceServer,
    contributions::contribution_service_server::ContributionServiceServer,
    wallet::wallet_service_server::WalletServiceServer,
    market::market_service_server::MarketServiceServer,
    fair_score::fair_score_service_server::FairScoreServiceServer,
    messages::message_service_server::MessageServiceServer,
    fraud::fraud_service_server::FraudServiceServer,
};

pub struct GrpcServers {
    pub db:     sqlx::PgPool,
    pub redis:  redis::aio::ConnectionManager,
    pub config: Config,
}

impl GrpcServers {
    /// Start the native gRPC server on port 50051.
    pub async fn serve_native(self) -> anyhow::Result<()> {
        let addr: SocketAddr = format!("{}:50051", self.config.app_host).parse()?;

        tracing::info!("motherlode gRPC listening on {}", addr);

        let auth_interceptor = auth_interceptor(self.config.clone());

        Server::builder()
            .layer(tower::ServiceBuilder::new()
                .layer(tonic::service::interceptor(crate::grpc::interceptors::logging_interceptor))
            )
            // Auth service — no auth required (handles its own auth)
            .add_service(AuthServiceServer::new(
                AuthServiceImpl::new(self.db.clone(), self.redis.clone(), self.config.clone())
            ))
            // All other services — JWT auth required
            .add_service(StokvelServiceServer::with_interceptor(
                StokvelServiceImpl::new(self.db.clone(), self.redis.clone()),
                auth_interceptor.clone(),
            ))
            .add_service(ContributionServiceServer::with_interceptor(
                ContributionServiceImpl::new(self.db.clone(), self.redis.clone()),
                auth_interceptor.clone(),
            ))
            .add_service(WalletServiceServer::with_interceptor(
                WalletServiceImpl::new(self.db.clone(), self.redis.clone(), self.config.clone()),
                auth_interceptor.clone(),
            ))
            .add_service(MarketServiceServer::with_interceptor(
                MarketServiceImpl::new(self.db.clone()),
                auth_interceptor.clone(),
            ))
            .add_service(FairScoreServiceServer::with_interceptor(
                FairScoreServiceImpl::new(self.db.clone()),
                auth_interceptor.clone(),
            ))
            .add_service(MessageServiceServer::with_interceptor(
                MessageServiceImpl::new(self.db.clone(), self.redis.clone()),
                auth_interceptor.clone(),
            ))
            .add_service(FraudServiceServer::with_interceptor(
                FraudServiceImpl::new(self.db.clone()),
                auth_interceptor.clone(),
            ))
            .serve(addr)
            .await?;

        Ok(())
    }

    /// Start the gRPC-Web server on port 50052.
    /// Wraps the same services with GrpcWebLayer + CORS for browser/WASM clients.
    pub async fn serve_web(self) -> anyhow::Result<()> {
        let addr: SocketAddr = format!("{}:50052", self.config.app_host).parse()?;

        tracing::info!("motherlode gRPC-Web listening on {}", addr);

        let auth_interceptor = auth_interceptor(self.config.clone());

        Server::builder()
            .accept_http1(true) // required for gRPC-Web
            .layer(GrpcWebLayer::new())
            .layer(CorsLayer::permissive()) // restrict in production
            .add_service(AuthServiceServer::new(
                AuthServiceImpl::new(self.db.clone(), self.redis.clone(), self.config.clone())
            ))
            .add_service(StokvelServiceServer::with_interceptor(
                StokvelServiceImpl::new(self.db.clone(), self.redis.clone()),
                auth_interceptor.clone(),
            ))
            .add_service(WalletServiceServer::with_interceptor(
                WalletServiceImpl::new(self.db.clone(), self.redis.clone(), self.config.clone()),
                auth_interceptor.clone(),
            ))
            .add_service(MarketServiceServer::with_interceptor(
                MarketServiceImpl::new(self.db.clone()),
                auth_interceptor.clone(),
            ))
            .serve(addr)
            .await?;

        Ok(())
    }
}
""",
),

(
"motherlode/src/grpc/services/mod.rs",
"""pub mod auth;
pub mod stokvels;
pub mod contributions;
pub mod wallet;
pub mod market;
pub mod fair_score;
pub mod messages;
pub mod fraud;
""",
),

# Auth gRPC service implementation
(
"motherlode/src/grpc/services/auth.rs",
"""//! gRPC AuthService implementation.
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
            gender:        serde_json::from_str(&format!("\"{}\"", req.gender))
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
""",
),

# Stub services
*[
(
f"motherlode/src/grpc/services/{name}.rs",
f"""//! gRPC {name.title()} service — TODO: implement after REST service is complete.

use tonic::{{Request, Response, Status}};
use sqlx::PgPool;

pub struct {name.title().replace('_', '')}ServiceImpl {{
    db: PgPool,
}}

impl {name.title().replace('_', '')}ServiceImpl {{
    pub fn new(db: PgPool{', _redis: redis::aio::ConnectionManager' if name in ['stokvels', 'contributions', 'wallet', 'messages'] else ''}) -> Self {{
        Self {{ db }}
    }}
}}
""",
)
for name in ["stokvels", "contributions", "wallet", "market", "fair_score", "messages", "fraud"]
],

# Cargo.toml additions for motherlode
(
"motherlode/GRPC_DEPS.toml",
"""# Add these to motherlode/Cargo.toml [dependencies]

tonic         = { version = "0.12", features = ["transport", "tls"] }
tonic-web     = "0.12"
prost         = "0.13"
prost-types   = "0.13"

# Add these to motherlode/Cargo.toml [build-dependencies]

tonic-build   = "0.12"
prost-build   = "0.13"
""",
),

# ══════════════════════════════════════════════════════════════════════════════
# SPEEDCRIME — gRPC client additions
# ══════════════════════════════════════════════════════════════════════════════

(
"speedcrime/build.rs",
"""fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)  // speedcrime is client-only
        .build_client(true)
        .compile_protos(
            &[
                "../proto/common.proto",
                "../proto/auth.proto",
                "../proto/stokvels.proto",
                "../proto/contributions.proto",
                "../proto/wallet.proto",
                "../proto/market.proto",
                "../proto/fair_score.proto",
                "../proto/messages.proto",
                "../proto/fraud.proto",
            ],
            &["../"],
        )?;

    Ok(())
}
""",
),

(
"speedcrime/src/grpc/mod.rs",
"""//! gRPC client module for speedcrime.
//! Connects to motherlode on port 50051 (native) or 50052 (grpc-web/WASM).

pub mod proto;
pub mod channel;
pub mod clients;
""",
),

(
"speedcrime/src/grpc/proto.rs",
"""//! Generated proto client types.

pub mod common {
    tonic::include_proto!("stockfair.common");
}

pub mod auth {
    tonic::include_proto!("stockfair.auth");
}

pub mod stokvels {
    tonic::include_proto!("stockfair.stokvels");
}

pub mod contributions {
    tonic::include_proto!("stockfair.contributions");
}

pub mod wallet {
    tonic::include_proto!("stockfair.wallet");
}

pub mod market {
    tonic::include_proto!("stockfair.market");
}

pub mod fair_score {
    tonic::include_proto!("stockfair.fairscore");
}

pub mod messages {
    tonic::include_proto!("stockfair.messages");
}

pub mod fraud {
    tonic::include_proto!("stockfair.fraud");
}
""",
),

(
"speedcrime/src/grpc/channel.rs",
"""//! gRPC channel management.
//! Handles connection, reconnection, and token injection.

use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::transport::{Channel, Endpoint};
use tonic::metadata::MetadataValue;
use tonic::service::interceptor::InterceptedService;
use tonic::{Request, Status};

use crate::state::auth::AuthState;

/// The gRPC endpoint — motherlode's native port.
/// Override with MOTHERLODE_GRPC_URL env var.
pub fn motherlode_endpoint() -> String {
    std::env::var("MOTHERLODE_GRPC_URL")
        .unwrap_or_else(|_| "http://localhost:50051".to_string())
}

/// Build a channel to motherlode.
pub async fn connect() -> anyhow::Result<Channel> {
    let endpoint = Endpoint::from_shared(motherlode_endpoint())?
        .tcp_keepalive(Some(std::time::Duration::from_secs(30)))
        .connect_timeout(std::time::Duration::from_secs(10))
        .timeout(std::time::Duration::from_secs(30));

    Ok(endpoint.connect().await?)
}

/// Build a channel with lazy (on-demand) connection.
pub fn connect_lazy() -> anyhow::Result<Channel> {
    let endpoint = Endpoint::from_shared(motherlode_endpoint())?
        .tcp_keepalive(Some(std::time::Duration::from_secs(30)));

    Ok(endpoint.connect_lazy())
}

/// Interceptor that injects JWT access token into every request.
#[derive(Clone)]
pub struct AuthInterceptor {
    pub auth: Arc<RwLock<AuthState>>,
}

impl tonic::service::Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        // Try to get token synchronously from a cached copy
        // Real implementation: use try_read() and cache token on login
        // For now: placeholder — token injection handled per-call in clients
        Ok(request)
    }
}

/// Inject a bearer token into request metadata.
pub fn with_token<T>(mut req: Request<T>, token: &str) -> Request<T> {
    if let Ok(val) = MetadataValue::try_from(format!("Bearer {}", token)) {
        req.metadata_mut().insert("authorization", val);
    }
    req
}
""",
),

(
"speedcrime/src/grpc/clients/mod.rs",
"""pub mod auth;
pub mod stokvels;
pub mod contributions;
pub mod wallet;
pub mod market;
pub mod fair_score;
pub mod messages;
pub mod fraud;
""",
),

(
"speedcrime/src/grpc/clients/auth.rs",
"""//! Auth gRPC client — wraps generated tonic client with ergonomic methods.

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
""",
),

# Stub clients
*[
(
f"speedcrime/src/grpc/clients/{name}.rs",
f"""//! {name.title()} gRPC client — TODO: implement after proto services are built.

use tonic::transport::Channel;

pub struct {name.title().replace('_', '')}Client {{
    // inner: {name.title().replace('_', '')}ServiceClient<Channel>,
}}

impl {name.title().replace('_', '')}Client {{
    pub fn new(_channel: Channel) -> Self {{
        Self {{}}
    }}
}}
""",
)
for name in ["stokvels", "contributions", "wallet", "market", "fair_score", "messages", "fraud"]
],

# Cargo.toml additions for speedcrime
(
"speedcrime/GRPC_DEPS.toml",
"""# Add these to speedcrime/Cargo.toml [dependencies]

tonic         = { version = "0.12", features = ["transport"] }
prost         = "0.13"
prost-types   = "0.13"
tokio         = { version = "1", features = ["full"] }

# For WASM grpc-web (native stays with tonic transport)
# tonic-web-wasm-client = "0.6"  # enable only for WASM target

# Add these to speedcrime/Cargo.toml [build-dependencies]

tonic-build   = "0.12"
prost-build   = "0.13"
""",
),

# ══════════════════════════════════════════════════════════════════════════════
# WORKSPACE Cargo.toml (ties everything together)
# ══════════════════════════════════════════════════════════════════════════════

(
"Cargo.toml",
"""[workspace]
members = [
    "motherlode",
    "speedcrime",
]
resolver = "2"

# Shared dependency versions — keeps motherlode and speedcrime in sync
[workspace.dependencies]
tonic       = { version = "0.12", features = ["transport"] }
prost       = "0.13"
prost-types = "0.13"
tokio       = { version = "1", features = ["full"] }
serde       = { version = "1", features = ["derive"] }
serde_json  = "1"
anyhow      = "1"
tracing     = "0.1"
uuid        = { version = "1", features = ["v4"] }
chrono      = { version = "0.4", features = ["serde"] }
""",
),

# .env additions
(
"motherlode/.env.grpc.example",
"""# Add to motherlode/.env

# gRPC ports (separate from REST :8080)
GRPC_PORT=50051
GRPC_WEB_PORT=50052
""",
),

(
"speedcrime/.env.grpc.example",
"""# Add to speedcrime/.env

# Motherlode gRPC endpoint
MOTHERLODE_GRPC_URL=http://localhost:50051

# For WASM: use grpc-web port
# MOTHERLODE_GRPC_WEB_URL=http://localhost:50052
""",
),

# README
(
"proto/README.md",
"""# StockFair Protocol Buffers

This directory is the single source of truth for all motherlode ↔ speedcrime communication.

## Services

| Proto                  | Port  | Description                          |
|------------------------|-------|--------------------------------------|
| auth.proto             | 50051 | Registration, OTP, sign in/out       |
| stokvels.proto         | 50051 | Group management, multi-sig, streaming |
| contributions.proto    | 50051 | Contributions, payouts, auto-pay     |
| wallet.proto           | 50051 | Deposits, withdrawals, history       |
| market.proto           | 50051 | Bulk deals, group orders, votes      |
| fair_score.proto       | 50051 | Trust score, history, platform stats |
| messages.proto         | 50051 | Group chat, voice notes, order votes |
| fraud.proto            | 50051 | Fraud Shield, feature lock           |

## Native vs gRPC-Web

- **speedcrime (native)** → port 50051 (tonic transport, HTTP/2)
- **WASM/browser clients** → port 50052 (tonic-web, HTTP/1.1 + CORS)

## Adding a new RPC

1. Add the message + rpc to the relevant .proto file
2. Run `cargo build` in motherlode (tonic_build regenerates server stubs)
3. Run `cargo build` in speedcrime (tonic_build regenerates client stubs)
4. Implement the method in `motherlode/src/grpc/services/{service}.rs`
5. Add the client method in `speedcrime/src/grpc/clients/{service}.rs`

## Why Protobuf?

StockFair handles real money in real-time. Protobuf gives us:
- **Type safety** across the entire stack (Rust → Rust)
- **~10x smaller** payloads than JSON (critical for mobile)
- **Streaming** for real-time contribution updates and group chat
- **Backward compatibility** via field numbers
- **Mathematical impossibility** of type mismatches at the transport layer
""",
),

]

def scaffold():
    for path, content in FILES:
        os.makedirs(os.path.dirname(path) if os.path.dirname(path) else ".", exist_ok=True)
        with open(path, "w") as f:
            f.write(content)
        print(f"  ✅  {path}")

    print("""
🦀  StockFair gRPC scaffold complete.

Generated:
  proto/           — 8 service definitions (source of truth)
  motherlode/      — tonic server + build.rs + grpc services
  speedcrime/      — tonic client + build.rs + grpc clients
  Cargo.toml       — workspace tying both together

Next steps:

  1. Install protoc (required by tonic-build):
       sudo apt install -y protobuf-compiler

  2. Add gRPC deps to each Cargo.toml:
       See motherlode/GRPC_DEPS.toml and speedcrime/GRPC_DEPS.toml

  3. Add grpc module to motherlode/src/main.rs:
       mod grpc;
       // Then spawn both servers:
       tokio::spawn(grpc::server::GrpcServers { ... }.serve_native());
       tokio::spawn(grpc::server::GrpcServers { ... }.serve_web());

  4. Build to verify protos compile:
       cargo build -p motherlode
       cargo build -p speedcrime

  5. Add grpc module to speedcrime/src/main.rs:
       mod grpc;
       // Then connect:
       let channel = grpc::channel::connect_lazy()?;
       let mut auth = grpc::clients::auth::AuthClient::new(channel);

  6. Test with grpcurl:
       grpcurl -plaintext localhost:50051 list
       grpcurl -plaintext -d '{"phone":"+27821234567","purpose":"register"}' \\
         localhost:50051 stockfair.auth.AuthService/SendOtp

Priority build order:
  1. auth.proto     ← already 80% implemented
  2. stokvels.proto ← core product
  3. wallet.proto   ← money movement
  4. messages.proto ← streaming (bidirectional)
  5. market.proto   ← retailer integration
""")

if __name__ == "__main__":
    scaffold()
