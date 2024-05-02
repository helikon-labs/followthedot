use crate::substrate::event::TransferEvent;
use frame_support::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub timestamp: u64,
    pub number: u64,
    pub hash: String,
    pub parent_hash: String,
    pub author_address: Option<String>,
    pub transfers: Vec<TransferEvent>,
}

impl Block {
    pub fn convert_to_old_dot(&self) -> Block {
        let mut block = self.clone();
        block.transfers = Vec::new();
        for transfer in self.transfers.iter() {
            let mut transfer = transfer.clone();
            transfer.amount /= 100;
            block.transfers.push(transfer);
        }
        block
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventDigest {
    logs: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockHeader {
    pub digest: EventDigest,
    pub extrinsics_root: String,
    pub number: String,
    pub parent_hash: String,
    pub state_root: String,
}

impl BlockHeader {
    pub fn get_number(&self) -> anyhow::Result<u64> {
        let number = u64::from_str_radix(self.number.trim_start_matches("0x"), 16)?;
        Ok(number)
    }
}
