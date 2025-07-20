use actix_web::{HttpResponse, Responder, web};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

use crate::{models::PaymentRequest, utils::app_state::AppState};

pub async fn create_payment(
    payment_request: PaymentRequest,
    app_state: web::Data<AppState>,
) -> Result<(), sqlx::Error> {
    Ok(())
}

#[actix_web::post("/payments")]
pub async fn create_payment_handler(
    payment_request: web::Json<PaymentRequest>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    match create_payment(payment_request.into_inner(), app_state).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_payment_handler);
}
