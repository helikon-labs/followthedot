use frame_metadata::v14::StorageHasher;
use ftd_types::substrate::account_id::AccountId;
use jsonrpsee_core::params::ArrayParams;
use jsonrpsee_core::JsonValue;
use sp_core::storage::StorageKey;
use sp_core::Decode;

pub(crate) fn hash(hasher: &StorageHasher, bytes: &[u8]) -> Vec<u8> {
    match hasher {
        StorageHasher::Identity => bytes.to_vec(),
        StorageHasher::Blake2_128 => sp_core::blake2_128(bytes).to_vec(),
        StorageHasher::Blake2_128Concat => sp_core::blake2_128(bytes)
            .iter()
            .chain(bytes)
            .cloned()
            .collect(),
        StorageHasher::Blake2_256 => sp_core::blake2_256(bytes).to_vec(),
        StorageHasher::Twox128 => sp_core::twox_128(bytes).to_vec(),
        StorageHasher::Twox256 => sp_core::twox_256(bytes).to_vec(),
        StorageHasher::Twox64Concat => sp_core::twox_64(bytes)
            .iter()
            .chain(bytes)
            .cloned()
            .collect(),
    }
}

pub(crate) fn get_storage_plain_key(module_name: &str, storage_name: &str) -> String {
    let hasher = StorageHasher::Twox128;
    let mut storage_hash: Vec<u8> = Vec::new();
    let mut module_name_hash = hash(&hasher, module_name.as_bytes());
    storage_hash.append(&mut module_name_hash);
    let mut storage_name_hash = hash(&hasher, storage_name.as_bytes());
    storage_hash.append(&mut storage_name_hash);
    let storage_key_hex: String = hex::encode(storage_hash);
    format!("0x{storage_key_hex}")
}

pub fn get_rpc_paged_keys_params<'a>(
    module: &'a str,
    name: &'a str,
    count: usize,
    start_key: Option<&'a str>,
    block_hash: Option<&'a str>,
) -> ArrayParams {
    let mut params = ArrayParams::new();
    params.insert(get_storage_plain_key(module, name)).unwrap();
    params.insert(count).unwrap();
    if let Some(start_key) = start_key {
        params.insert(start_key).unwrap();
    } else {
        params.insert(JsonValue::Null).unwrap();
    }
    if let Some(block_hash) = block_hash {
        params.insert(block_hash).unwrap();
    } else {
        params.insert(JsonValue::Null).unwrap();
    }
    params
}

pub fn get_rpc_storage_plain_params<'a>(
    module: &'a str,
    name: &'a str,
    block_hash: Option<&'a str>,
) -> ArrayParams {
    //let mut params: Vec<JsonValue> = vec![.into()];
    let mut params = ArrayParams::new();
    params.insert(get_storage_plain_key(module, name)).unwrap();
    if let Some(block_hash) = block_hash {
        //params.push(block_hash.into());
        params.insert(block_hash).unwrap();
    }
    params
}

pub fn decode_hex_string<T>(hex_string: &str) -> anyhow::Result<T>
where
    T: Decode,
{
    let trimmed_hex_string = hex_string.trim_start_matches("0x");
    let mut bytes: &[u8] = &hex::decode(trimmed_hex_string).unwrap();
    let decoded = Decode::decode(&mut bytes)?;
    Ok(decoded)
}

pub fn account_id_from_storage_key(storage_key: &StorageKey) -> AccountId {
    storage_key.0[storage_key.0.len() - 32..]
        .try_into()
        .unwrap()
}
