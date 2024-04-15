use crate::SidecarClient;
use ftd_types::err::BlockDataError;
use ftd_types::substrate::Block;
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

fn get_author_address(json: &Value) -> anyhow::Result<String> {
    Ok(json["authorId"]
        .as_str()
        .ok_or(BlockDataError::AuthorIdNotFound)?
        .to_string())
}

fn get_timestamp(json: &Value) -> anyhow::Result<u64> {
    Ok(json["value"]
        .as_str()
        .ok_or(BlockDataError::TimestampNotFound)?
        .parse::<u64>()?)
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

    async fn get_block_from_path(&self, path: &str) -> anyhow::Result<Block> {
        let url = format!("{}{}", self.base_url, path);
        let json = self
            .http_client
            .get(&url)
            .send()
            .await?
            .json::<Value>()
            .await?;
        let number = get_number(&json)?;
        let hash = get_hash(&json)?;
        let parent_hash = get_parent_hash(&json)?;
        let author_address = get_author_address(&json)?;
        let timestamp = self.get_block_timestamp(&hash).await?;
        Ok(Block {
            timestamp,
            number,
            hash,
            parent_hash,
            author_address,
            transfers: Vec::new(),
        })
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
}
