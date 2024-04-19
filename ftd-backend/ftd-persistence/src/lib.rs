use crate::neo4j::Neo4JStorage;
use crate::postgres::PostgreSQLStorage;
use ftd_config::Config;
use ftd_types::substrate::event::Transfer;
use ftd_types::substrate::{Block, Identity, SubIdentity};
use lazy_static::lazy_static;
use neo4rs::Txn;
use sqlx::{Postgres, Transaction};

pub mod neo4j;
pub mod postgres;

const REDENOMINATION_BLOCK_NUMBER: u64 = 1_205_128;

lazy_static! {
    static ref CONFIG: Config = Config::default();
}

pub struct Storage {
    postgres: PostgreSQLStorage,
    neo4j: Neo4JStorage,
}

impl Storage {
    pub async fn new() -> anyhow::Result<Storage> {
        Ok(Self {
            postgres: PostgreSQLStorage::new(&CONFIG).await?,
            neo4j: Neo4JStorage::new(&CONFIG).await?,
        })
    }

    pub async fn get_max_block_number(&self) -> anyhow::Result<i64> {
        self.postgres.get_max_block_number().await
    }

    pub async fn block_exists_by_number(&self, block_number: u64) -> anyhow::Result<bool> {
        self.postgres.block_exists_by_number(block_number).await
    }

    async fn save_transfer(
        &self,
        block: &Block,
        transfer: &Transfer,
        postgres_tx: &mut Transaction<'_, Postgres>,
        neo4j_tx: &mut Txn,
    ) -> anyhow::Result<()> {
        // save accounts
        self.postgres
            .save_account(&transfer.from, postgres_tx)
            .await?;
        self.neo4j.save_account(&transfer.from, neo4j_tx).await?;
        self.postgres
            .save_account(&transfer.to, postgres_tx)
            .await?;
        self.neo4j.save_account(&transfer.to, neo4j_tx).await?;
        // save transfer
        self.postgres
            .save_transfer(block, transfer, postgres_tx)
            .await?;
        let (volume, count) = self
            .postgres
            .update_transfer_volume(transfer, postgres_tx)
            .await?;
        self.neo4j
            .save_transfer_summary(&transfer.from, &transfer.to, volume, count, neo4j_tx)
            .await?;
        Ok(())
    }

    pub async fn save_block(
        &self,
        block: Block,
        update_identities: Vec<(String, Option<Identity>, Option<SubIdentity>)>,
    ) -> anyhow::Result<()> {
        let mut postgres_tx = self.postgres.begin_tx().await?;
        let mut neo4j_tx = self.neo4j.begin_tx().await?;

        let block = if block.number < REDENOMINATION_BLOCK_NUMBER {
            block.convert_to_old_dot()
        } else {
            block
        };
        self.postgres.save_block(&block, &mut postgres_tx).await?;
        for transfer in block.transfers.iter() {
            self.save_transfer(&block, transfer, &mut postgres_tx, &mut neo4j_tx)
                .await?;
        }
        for update_identity in update_identities.iter() {
            self.postgres
                .save_account_with_identity(
                    update_identity.0.as_str(),
                    &update_identity.1,
                    &update_identity.2,
                    block.number,
                    &mut postgres_tx,
                )
                .await?;
            self.neo4j
                .save_account_with_identity(
                    update_identity.0.as_str(),
                    &update_identity.1,
                    &update_identity.2,
                    &mut neo4j_tx,
                )
                .await?;
        }

        self.neo4j.commit_tx(neo4j_tx).await?;
        self.postgres.commit_tx(postgres_tx).await?;
        Ok(())
    }
}
