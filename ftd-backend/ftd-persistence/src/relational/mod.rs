use crate::{CONFIG, REDENOMINATION_BLOCK_NUMBER};
use ftd_types::substrate::block::Block;
use ftd_types::substrate::event::TransferEvent;
use ftd_types::substrate::identity::{Identity, SubIdentity};
use postgres::PostgreSQLStorage;
use sqlx::{Postgres, Transaction};

pub mod postgres;

pub struct RelationalStorage {
    postgres: PostgreSQLStorage,
}

impl RelationalStorage {
    pub async fn new() -> anyhow::Result<RelationalStorage> {
        Ok(Self {
            postgres: PostgreSQLStorage::new(&CONFIG).await?,
        })
    }

    pub async fn get_transfer_volume_updater_last_processed_transfer_id(
        &self,
    ) -> anyhow::Result<i32> {
        self.postgres
            .get_transfer_volume_updater_last_processed_transfer_id()
            .await
    }

    pub async fn set_transfer_volume_updater_last_processed_transfer_id(
        &self,
        id: i32,
    ) -> anyhow::Result<()> {
        self.postgres
            .set_transfer_volume_updater_last_processed_transfer_id(id)
            .await
    }

    pub async fn get_max_transfer_id(&self) -> anyhow::Result<i32> {
        self.postgres.get_max_transfer_id().await
    }

    pub async fn get_transfer_by_id(&self, id: i32) -> anyhow::Result<Option<TransferEvent>> {
        self.postgres.get_transfer_by_id(id).await
    }

    pub async fn update_transfer_volume(
        &self,
        transfer: &TransferEvent,
    ) -> anyhow::Result<(u128, u32)> {
        self.postgres.update_transfer_volume(transfer).await
    }

    pub async fn get_max_block_number_in_range_inclusive(
        &self,
        range: (u64, u64),
    ) -> anyhow::Result<i64> {
        self.postgres
            .get_max_block_number_in_range_inclusive(range)
            .await
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
        transfer: &TransferEvent,
        tx: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<()> {
        // save accounts
        self.postgres.save_account(&transfer.from, tx).await?;
        self.postgres.save_account(&transfer.to, tx).await?;
        // save transfer
        self.postgres.save_transfer(block, transfer, tx).await?;
        Ok(())
    }

    pub async fn save_block(&self, block: Block) -> anyhow::Result<()> {
        let mut tx = self.postgres.begin_tx().await?;
        let block = if block.number < REDENOMINATION_BLOCK_NUMBER {
            block.convert_to_old_dot()
        } else {
            block
        };
        self.postgres.save_block(&block, &mut tx).await?;
        for transfer in block.transfers.iter() {
            self.save_transfer(&block, transfer, &mut tx).await?;
        }
        self.postgres.commit_tx(tx).await?;
        Ok(())
    }

    async fn delete_all_identities(
        &self,
        tx: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<()> {
        self.postgres.delete_all_identities(tx).await
    }

    pub async fn save_identities(
        &self,
        block_hash: &str,
        block_number: u64,
        identities: &[Identity],
    ) -> anyhow::Result<()> {
        let mut tx = self.postgres.begin_tx().await?;
        self.delete_all_identities(&mut tx).await?;
        for identity in identities.iter() {
            self.postgres
                .save_identity(block_hash, block_number, identity, &mut tx)
                .await?;
        }
        self.postgres.commit_tx(tx).await?;
        Ok(())
    }

    async fn delete_all_sub_identities(
        &self,
        tx: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<()> {
        self.postgres.delete_all_sub_identities(tx).await
    }

    pub async fn save_sub_identities(
        &self,
        block_hash: &str,
        block_number: u64,
        sub_identities: &[SubIdentity],
    ) -> anyhow::Result<()> {
        let mut tx = self.postgres.begin_tx().await?;
        self.delete_all_sub_identities(&mut tx).await?;
        for sub_identity in sub_identities.iter() {
            self.postgres
                .save_sub_identity(block_hash, block_number, sub_identity, &mut tx)
                .await?;
        }
        self.postgres.commit_tx(tx).await?;
        Ok(())
    }
}
