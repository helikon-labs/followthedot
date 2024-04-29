use crate::CONFIG;
use ftd_types::graph::GraphUpdaterState;
use ftd_types::substrate::event::Transfer;
use neo4j::Neo4JStorage;

pub mod neo4j;

pub struct GraphStorage {
    neo4j: Neo4JStorage,
}

impl GraphStorage {
    pub async fn new() -> anyhow::Result<GraphStorage> {
        Ok(Self {
            neo4j: Neo4JStorage::new(&CONFIG).await?,
        })
    }

    pub async fn get_state(&self) -> anyhow::Result<GraphUpdaterState> {
        self.neo4j.get_state().await
    }

    pub async fn update_last_processed_transfer_id(&self, id: i32) -> anyhow::Result<()> {
        self.neo4j.update_last_processed_transfer_id(id).await
    }

    pub async fn save_transfer(&self, transfer: &Transfer) -> anyhow::Result<()> {
        self.neo4j.save_account(transfer.from.as_str()).await?;
        self.neo4j.save_account(transfer.to.as_str()).await?;
        self.neo4j.update_transfer_volume(transfer).await?;
        Ok(())
    }
}
