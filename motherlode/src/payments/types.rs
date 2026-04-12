use rust_decimal::Decimal;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct DepositRequest {
    pub user_id: Uuid,
    pub amount: Decimal,
    pub reference: String,
    pub phone: String,
}

#[derive(Debug, Clone)]
pub struct WithdrawRequest {
    pub user_id: Uuid,
    pub amount: Decimal,
    pub reference: String,
    pub bank_account_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct TransferRequest {
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub amount: Decimal,
    pub reference: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResponse {
    pub provider_ref: String,
    pub status: TransactionStatus,
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Reversed,
}
