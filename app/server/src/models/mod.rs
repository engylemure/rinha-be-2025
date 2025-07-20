use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentRequest {
  correlation_id: Uuid,
  amount: Decimal
}