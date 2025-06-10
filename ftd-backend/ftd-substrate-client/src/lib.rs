use crate::storage_utility::{
    account_id_from_storage_key, decode_hex_string, get_rpc_paged_keys_params,
    get_rpc_storage_plain_params, get_storage_plain_key, hash,
};
use frame_metadata::v14::StorageHasher;
use ftd_types::substrate::account_id::AccountId;
use ftd_types::substrate::balance::Balance;
use ftd_types::substrate::block::BlockHeader;
use ftd_types::substrate::chain::Chain;
use ftd_types::substrate::identity::{Identity, SubIdentity};
use ftd_types::substrate::{AccountData, AccountInfo};
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee_core::client::{Client, ClientT};
use jsonrpsee_core::rpc_params;
use parity_scale_codec::{Decode, Encode};
use rustc_hash::FxHashMap as HashMap;
use sp_core::storage::StorageChangeSet;
use std::str::FromStr;

mod storage_utility;

const KEY_QUERY_PAGE_SIZE: usize = 1000;

pub struct SubstrateClient {
    pub chain: Chain,
    ws_client: Client,
}

impl SubstrateClient {
    pub async fn new(
        rpc_url: &str,
        connection_timeout: u64,
        request_timeout: u64,
    ) -> anyhow::Result<Self> {
        log::info!("Constructing Substrate client.");
        let ws_client = WsClientBuilder::default()
            .connection_timeout(std::time::Duration::from_secs(connection_timeout))
            .request_timeout(std::time::Duration::from_secs(request_timeout))
            .build(rpc_url)
            .await?;
        let chain: String = ws_client.request("system_chain", rpc_params!()).await?;
        let chain = Chain::from_str(chain.as_str())?;
        log::info!("{chain} Substrate connection successful.");
        Ok(SubstrateClient { chain, ws_client })
    }

    async fn get_all_keys_for_storage(
        &self,
        module_name: &str,
        storage_name: &str,
        block_hash: &str,
    ) -> anyhow::Result<Vec<String>> {
        let mut all_keys: Vec<String> = Vec::new();
        let mut total = 0;
        loop {
            let last = all_keys.last();
            let mut keys: Vec<String> = self
                .ws_client
                .request(
                    "state_getKeysPaged",
                    get_rpc_paged_keys_params(
                        module_name,
                        storage_name,
                        KEY_QUERY_PAGE_SIZE,
                        if let Some(last) = last {
                            Some(last.as_str())
                        } else {
                            None
                        },
                        Some(block_hash),
                    ),
                )
                .await?;
            let keys_length = keys.len();
            all_keys.append(&mut keys);
            total += keys_length;
            println!("{total}");
            if keys_length < KEY_QUERY_PAGE_SIZE {
                break;
            }
        }
        Ok(all_keys)
    }

    pub async fn get_current_block_hash(&self) -> anyhow::Result<String> {
        let hash = self
            .ws_client
            .request("chain_getBlockHash", rpc_params!())
            .await?;
        Ok(hash)
    }

    pub async fn get_block_hash(&self, block_number: u64) -> anyhow::Result<String> {
        let hash: String = self
            .ws_client
            .request("chain_getBlockHash", rpc_params!(block_number))
            .await?;
        Ok(format!(
            "0x{}",
            hash.trim_start_matches("0x").to_uppercase()
        ))
    }

    pub async fn get_finalized_block_hash(&self) -> anyhow::Result<String> {
        let hash: String = self
            .ws_client
            .request("chain_getFinalizedHead", rpc_params!())
            .await?;
        Ok(format!(
            "0x{}",
            hash.trim_start_matches("0x").to_uppercase()
        ))
    }

    pub async fn get_block_timestamp(&self, block_hash: &str) -> anyhow::Result<u64> {
        let hex_string: String = self
            .ws_client
            .request(
                "state_getStorage",
                get_rpc_storage_plain_params("Timestamp", "Now", Some(block_hash)),
            )
            .await?;
        decode_hex_string(hex_string.as_str())
    }

    pub async fn get_block_header(&self, block_hash: &str) -> anyhow::Result<BlockHeader> {
        let mut header: BlockHeader = self
            .ws_client
            .request("chain_getHeader", rpc_params!(&block_hash))
            .await?;
        header.parent_hash = format!(
            "0x{}",
            header.parent_hash.trim_start_matches("0x").to_uppercase()
        );
        header.extrinsics_root = format!(
            "0x{}",
            header
                .extrinsics_root
                .trim_start_matches("0x")
                .to_uppercase()
        );
        header.state_root = format!(
            "0x{}",
            header.state_root.trim_start_matches("0x").to_uppercase()
        );
        Ok(header)
    }

    pub async fn get_identities(&self, at: &str) -> anyhow::Result<Vec<Identity>> {
        let keys = self
            .get_all_keys_for_storage("Identity", "IdentityOf", at)
            .await?;
        log::info!("Got {} identity keys.", keys.len());
        let values: Vec<StorageChangeSet<String>> = self
            .ws_client
            .request("state_queryStorageAt", rpc_params!(keys, at))
            .await?;
        let mut identities = Vec::new();
        for (storage_key, storage_data) in values[0].changes.iter() {
            let account_id = account_id_from_storage_key(storage_key);
            if let Some(data) = storage_data {
                let bytes: &[u8] = &data.0;
                let identity = Identity::from_bytes(account_id, bytes).unwrap();
                identities.push(identity);
            }
        }
        Ok(identities)
    }

    pub async fn get_sub_identities(&self, at: &str) -> anyhow::Result<Vec<SubIdentity>> {
        let keys = self
            .get_all_keys_for_storage("Identity", "SuperOf", at)
            .await?;
        log::info!("Got {} sub identity keys.", keys.len());
        let mut sub_identities = Vec::new();
        let values: Vec<StorageChangeSet<String>> = self
            .ws_client
            .request("state_queryStorageAt", rpc_params!(keys, at))
            .await?;
        for (storage_key, storage_data) in values[0].changes.iter() {
            let account_id = account_id_from_storage_key(storage_key);
            if let Some(data) = storage_data {
                let bytes: &[u8] = &data.0;
                sub_identities.push(SubIdentity::from_bytes(account_id, bytes)?)
            }
        }
        Ok(sub_identities)
    }

    pub async fn get_balances(
        &self,
        account_ids: &[AccountId],
        maybe_block_hash: Option<&str>,
    ) -> anyhow::Result<HashMap<AccountId, Balance>> {
        let storage_key_hex = get_storage_plain_key("System", "Account");
        let hasher = StorageHasher::Blake2_128Concat;
        let mut keys = Vec::new();
        for account_id in account_ids {
            let key_hash = hash(&hasher, &account_id.encode());
            let key_hex: String = hex::encode(key_hash);
            keys.push(format!("{storage_key_hex}{key_hex}"));
        }
        let mut map = HashMap::default();
        for chunk in keys.chunks(KEY_QUERY_PAGE_SIZE) {
            let rpc_params = if let Some(block_hash) = maybe_block_hash {
                rpc_params!(chunk, block_hash)
            } else {
                rpc_params!(chunk)
            };
            let chunk_values: Vec<StorageChangeSet<String>> = self
                .ws_client
                .request("state_queryStorageAt", rpc_params)
                .await?;
            for (storage_key, data) in &chunk_values[0].changes {
                if let Some(data) = data {
                    let mut bytes: &[u8] = &data.0;
                    let account_info: AccountInfo<u32, AccountData<u128>> =
                        Decode::decode(&mut bytes)?;
                    map.insert(
                        account_id_from_storage_key(storage_key),
                        Balance {
                            free: account_info.data.free,
                            reserved: account_info.data.reserved,
                            frozen: account_info.data.frozen,
                        },
                    );
                }
            }
        }
        Ok(map)
    }

    pub async fn get_balance(
        &self,
        account_id: &AccountId,
        maybe_block_hash: Option<&str>,
    ) -> anyhow::Result<Option<Balance>> {
        let storage_key_hex = get_storage_plain_key("System", "Account");
        let hasher = StorageHasher::Blake2_128Concat;
        let key_hash = hash(&hasher, &account_id.encode());
        let key_hex: String = hex::encode(key_hash);
        let keys = vec![format!("{storage_key_hex}{key_hex}")];
        let rpc_params = if let Some(block_hash) = maybe_block_hash {
            rpc_params!(keys, block_hash)
        } else {
            rpc_params!(keys)
        };
        let values: Vec<StorageChangeSet<String>> = self
            .ws_client
            .request("state_queryStorageAt", rpc_params)
            .await?;
        if let Some(change_set) = values.first() {
            if let Some((_, Some(data))) = change_set.changes.first() {
                let mut bytes: &[u8] = &data.0;
                let account_info: AccountInfo<u32, AccountData<u128>> = Decode::decode(&mut bytes)?;
                let account_data = account_info.data;
                Ok(Some(Balance {
                    free: account_data.free,
                    reserved: account_data.reserved,
                    frozen: account_data.frozen,
                }))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}
