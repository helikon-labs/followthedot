use crate::CONFIG;
use ftd_types::graph::{GraphUpdaterState, TransferVolume};
use ftd_types::substrate::event::TransferEvent;
use neo4j::Neo4JStorage;
use neo4rs::Txn;

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

    pub async fn begin_tx(&self) -> anyhow::Result<Txn> {
        self.neo4j.begin_tx().await
    }

    pub async fn commit_tx(&self, tx: Txn) -> anyhow::Result<()> {
        self.neo4j.commit_tx(tx).await
    }

    pub async fn get_state(&self) -> anyhow::Result<GraphUpdaterState> {
        self.neo4j.get_state().await
    }

    pub async fn update_last_processed_transfer_id(
        &self,
        tx: &mut Txn,
        id: i32,
    ) -> anyhow::Result<()> {
        self.neo4j.update_last_processed_transfer_id(tx, id).await
    }

    pub async fn update_last_processed_identity_change_id(
        &self,
        tx: &mut Txn,
        id: i32,
    ) -> anyhow::Result<()> {
        self.neo4j
            .update_last_processed_identity_change_id(tx, id)
            .await
    }

    pub async fn save_transfer(
        &self,
        tx: &mut Txn,
        transfer: &TransferEvent,
    ) -> anyhow::Result<()> {
        self.neo4j.save_account(tx, transfer.from.as_str()).await?;
        self.neo4j.save_account(tx, transfer.to.as_str()).await?;
        self.neo4j.update_transfer_volume(tx, transfer).await?;
        Ok(())
    }

    pub async fn get_transfer_volumes_for_account(
        &self,
        address: &str,
        limit: u16,
    ) -> anyhow::Result<Vec<TransferVolume>> {
        self.neo4j
            .get_transfer_volumes_for_account(address, limit)
            .await
    }
}
