use crate::{ResultResponse, ServiceState};
use actix_web::{get, web, HttpResponse};
use ftd_types::err::ServiceError;
use ftd_types::substrate::account_id::AccountId;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
pub(crate) struct TransferListQueryParameters {
    from: String,
    to: String,
}

#[get("/transfer")]
pub(crate) async fn transfer_list_service(
    query: web::Query<TransferListQueryParameters>,
    state: web::Data<ServiceState>,
) -> ResultResponse {
    if AccountId::from_str(query.from.as_str()).is_err() {
        return Ok(HttpResponse::BadRequest().json(ServiceError::from("Invalid sender address.")));
    }
    if AccountId::from_str(query.to.as_str()).is_err() {
        return Ok(
            HttpResponse::BadRequest().json(ServiceError::from("Invalid recipient address."))
        );
    }
    Ok(HttpResponse::Ok().json(
        state
            .relational_storage
            .get_transfers_by_sender_and_recipient(query.from.as_str(), query.to.as_str())
            .await?,
    ))
}
