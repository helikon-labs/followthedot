use crate::SidecarClient;
use ftd_types::err::{BlockDataError, TransferEventDataError};
use ftd_types::substrate::block::Block;
use ftd_types::substrate::event::TransferEvent;
use serde_json::Value;

fn get_number(json: &Value) -> anyhow::Result<u64> {
    Ok(json["number"]
        .as_str()
        .ok_or(BlockDataError::BlockNumberNotFound)?
        .parse::<u64>()?)
}

fn get_hash(json: &Value) -> anyhow::Result<String> {
    Ok(json["hash"]
        .as_str()
        .ok_or(BlockDataError::HashNotFound)?
        .to_lowercase()
        .trim_start_matches("0x")
        .to_string())
}

fn get_parent_hash(json: &Value) -> anyhow::Result<String> {
    Ok(json["parentHash"]
        .as_str()
        .ok_or(BlockDataError::ParentHashNotFound)?
        .to_lowercase()
        .trim_start_matches("0x")
        .to_string())
}

fn get_author_address(json: &Value) -> Option<String> {
    json["authorId"].as_str().map(|str| str.to_string())
}

fn get_timestamp(json: &Value) -> anyhow::Result<u64> {
    Ok(json["value"]
        .as_str()
        .ok_or(BlockDataError::TimestampNotFound)?
        .parse::<u64>()?)
}

fn get_transfer_events(json: &Value) -> anyhow::Result<Vec<TransferEvent>> {
    let mut transfers = Vec::new();
    let extrinsics = json["extrinsics"]
        .as_array()
        .ok_or(BlockDataError::ExtrinsicsNotFound)?;
    let mut event_index: u16 = 0;
    for (extrinsic_index, extrinsic) in extrinsics.iter().enumerate() {
        let events = extrinsic["events"]
            .as_array()
            .ok_or(BlockDataError::ExtrinsicEventsNotFound)?;
        for (extrinsic_event_index, event_json) in events.iter().enumerate() {
            let module = event_json["method"]["pallet"]
                .as_str()
                .ok_or(BlockDataError::EventModuleNotFound)?;
            let event = event_json["method"]["method"]
                .as_str()
                .ok_or(BlockDataError::EventNameNotFound)?;
            if module.to_lowercase() == "balances" && event.to_lowercase() == "transfer" {
                log::info!("Found {module}.{event}.");
                let from = event_json["data"][0]
                    .as_str()
                    .ok_or(TransferEventDataError::FromNotFound)?
                    .to_string();
                let to = event_json["data"][1]
                    .as_str()
                    .ok_or(TransferEventDataError::ToNotFound)?
                    .to_string();
                let amount = event_json["data"][2]
                    .as_str()
                    .ok_or(TransferEventDataError::AmountNotFound)?
                    .parse::<u128>()?;
                transfers.push(TransferEvent {
                    extrinsic_index: extrinsic_index as u16,
                    extrinsic_event_index: extrinsic_event_index as u16,
                    event_index,
                    from,
                    to,
                    amount,
                })
            }
            event_index += 1;
        }
    }
    Ok(transfers)
}

impl SidecarClient {
    async fn get_block_timestamp(&self, hash: &str) -> anyhow::Result<u64> {
        let url = format!(
            "{}/pallets/timestamp/storage/now?at=0x{}",
            self.base_url, hash
        );
        let json = self
            .http_client
            .get(&url)
            .send()
            .await?
            .json::<Value>()
            .await?;
        get_timestamp(&json)
    }

    async fn get_blocks(&self, json: &Value) -> anyhow::Result<Vec<Block>> {
        let blocks_json = json.as_array().ok_or(BlockDataError::BlockArrayNotFound)?;
        let mut blocks = Vec::new();
        for block_json in blocks_json {
            blocks.push(self.get_block(block_json).await?);
        }
        Ok(blocks)
    }

    async fn get_block(&self, json: &Value) -> anyhow::Result<Block> {
        let number = get_number(json)?;
        let hash = get_hash(json)?;
        let parent_hash = get_parent_hash(json)?;
        let author_address = get_author_address(json);
        let timestamp = self.get_block_timestamp(&hash).await?;
        let transfers = get_transfer_events(json)?;
        Ok(Block {
            timestamp,
            number,
            hash,
            parent_hash,
            author_address,
            transfers,
        })
    }

    async fn get_block_from_path(&self, path: &str) -> anyhow::Result<Block> {
        let url = format!("{}{}", self.base_url, path);
        let json = self
            .http_client
            .get(&url)
            .query(&[("finalized", true), ("noFees", true)])
            .send()
            .await?
            .json::<Value>()
            .await?;
        self.get_block(&json).await
    }

    pub async fn get_head(&self) -> anyhow::Result<Block> {
        self.get_block_from_path("/blocks/head").await
    }

    pub async fn get_block_by_number(&self, number: u64) -> anyhow::Result<Block> {
        self.get_block_from_path(&format!("/blocks/{number}")).await
    }

    pub async fn get_block_by_hash(&self, hash: &str) -> anyhow::Result<Block> {
        if hash.starts_with("0x") {
            self.get_block_from_path(&format!("/blocks/{hash}")).await
        } else {
            self.get_block_from_path(&format!("/blocks/0x{hash}")).await
        }
    }

    pub async fn get_range_of_blocks(
        &self,
        start_block_number: u64,
        end_block_number: u64,
    ) -> anyhow::Result<Vec<Block>> {
        let url = format!("{}/blocks", self.base_url);
        let range = format!("{start_block_number}-{end_block_number}");
        let json = self
            .http_client
            .get(&url)
            .query(&[("range", range.as_str()), ("noFees", "true")])
            .send()
            .await?
            .json::<Value>()
            .await?;
        self.get_blocks(&json).await
    }
}
