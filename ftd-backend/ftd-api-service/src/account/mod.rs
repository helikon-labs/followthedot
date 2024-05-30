use crate::{ResultResponse, ServiceState, CONFIG};
use actix_web::{get, web, HttpResponse};
use ftd_substrate_client::SubstrateClient;
use ftd_types::api::account::{Account, AccountGraph};
use ftd_types::err::ServiceError;
use ftd_types::substrate::account_id::AccountId;
use rustc_hash::FxHashSet as HashSet;
use serde::Deserialize;
use std::str::FromStr;
use std::sync::Arc;

async fn set_account_balances(
    substrate_client: &Arc<SubstrateClient>,
    accounts: &mut [Account],
) -> anyhow::Result<()> {
    let account_ids: Vec<AccountId> = accounts
        .iter()
        .map(|account| AccountId::from_str(account.address.as_str()).unwrap())
        .collect();
    let balance_map = substrate_client.get_balances(&account_ids, None).await?;
    // get balances
    for account in accounts.iter_mut() {
        let account_id = AccountId::from_str(account.address.as_str()).unwrap();
        account.balance = balance_map.get(&account_id).cloned();
    }
    Ok(())
}

#[derive(Deserialize)]
pub(crate) struct AccountSearchParameters {
    query: String,
}

#[get("/account")]
pub(crate) async fn account_search_service(
    query: web::Query<AccountSearchParameters>,
    state: web::Data<ServiceState>,
) -> ResultResponse {
    let query = query.query.trim().to_string();
    if query.is_empty() {
        return Ok(
            HttpResponse::BadRequest().json(ServiceError::from("Query should not be empty."))
        );
    }
    let limit = if query.len() < 5 {
        CONFIG.api.account_search_limit / 2
    } else {
        CONFIG.api.account_search_limit
    };
    // search by display
    let mut accounts: Vec<Account> = state
        .relational_storage
        .search_identities(query.as_str(), limit)
        .await?
        .iter()
        .map(|identity| Account {
            address: identity.address.clone(),
            identity: Some(identity.clone()),
            sub_identity: None,
            super_identity: None,
            balance: None,
            subscan_account: None,
        })
        .collect();
    // get sub accounts
    if (accounts.len() as u16) < limit {
        for i in 0..accounts.len() {
            let account = accounts.get(i).unwrap();
            let mut sub_accounts = Vec::new();
            if let Some(super_identity) = &account.identity {
                state
                    .relational_storage
                    .get_sub_identities(account.address.as_str())
                    .await?
                    .iter()
                    .for_each(|sub_identity| {
                        sub_accounts.push(Account {
                            address: sub_identity.address.clone(),
                            identity: None,
                            sub_identity: Some(sub_identity.clone()),
                            super_identity: Some(super_identity.clone()),
                            balance: None,
                            subscan_account: None,
                        })
                    });
            }
            accounts.append(&mut sub_accounts);
            if (accounts.len() as u16) < CONFIG.api.account_search_limit {
                break;
            }
        }
    }
    // search by sub display
    if (accounts.len() as u16) < limit {
        let limit = limit - (accounts.len() as u16);
        let sub_identities = state
            .relational_storage
            .search_sub_identities(query.as_str(), limit)
            .await?;
        // get super identities
        for sub_identity in sub_identities.iter() {
            let super_identity = state
                .relational_storage
                .get_identity_by_address(sub_identity.super_address.as_str())
                .await?;
            accounts.push(Account {
                address: sub_identity.address.clone(),
                identity: None,
                sub_identity: Some(sub_identity.clone()),
                super_identity,
                balance: None,
                subscan_account: None,
            });
        }
    }
    // search by address
    if (accounts.len() as u16) < limit {
        let limit = limit - (accounts.len() as u16);
        let addresses = state
            .relational_storage
            .search_addresses(query.as_str(), limit)
            .await?;
        addresses.iter().for_each(|address| {
            accounts.push(Account {
                address: address.clone(),
                identity: None,
                sub_identity: None,
                super_identity: None,
                balance: None,
                subscan_account: None,
            });
        })
    }
    set_account_balances(&state.substrate_client, &mut accounts).await?;
    Ok(HttpResponse::Ok().json(accounts))
}

#[derive(Deserialize)]
pub(crate) struct AccountGraphParameters {
    address: String,
}

#[get("/account/{address}/graph")]
pub(crate) async fn account_graph_service(
    path: web::Path<AccountGraphParameters>,
    state: web::Data<ServiceState>,
) -> ResultResponse {
    if AccountId::from_str(path.address.as_str()).is_err() {
        return Ok(HttpResponse::BadRequest().json(ServiceError::from("Invalid address.")));
    }
    let transfer_volumes = state
        .graph_storage
        .get_transfer_volumes_for_account(path.address.as_str(), CONFIG.api.graph_search_limit)
        .await?;
    let mut addresses = HashSet::default();
    transfer_volumes.iter().for_each(|transfer_volume| {
        addresses.insert(transfer_volume.from.clone());
        addresses.insert(transfer_volume.to.clone());
    });
    let mut accounts = Vec::new();
    for address in addresses.iter() {
        let identity = state
            .relational_storage
            .get_identity_by_address(address)
            .await?;
        let sub_identity = state
            .relational_storage
            .get_sub_identity_by_address(address)
            .await?;
        let super_identity = if let Some(sub_identity) = &sub_identity {
            state
                .relational_storage
                .get_identity_by_address(sub_identity.super_address.as_str())
                .await?
        } else {
            None
        };
        let subscan_account = state.subscan_client.get_account(address).await?;
        accounts.push(Account {
            address: address.to_string(),
            identity,
            sub_identity,
            super_identity,
            balance: None,
            subscan_account: Some(subscan_account.data.account)
        })
    }
    set_account_balances(&state.substrate_client, &mut accounts).await?;
    Ok(HttpResponse::Ok().json(AccountGraph {
        accounts,
        transfer_volumes,
    }))
}
