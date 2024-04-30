use crate::{CONFIG, REDENOMINATION_BLOCK_NUMBER};
use ftd_types::substrate::event::{IdentityChange, Transfer};
use ftd_types::substrate::{Block, Identity, SubIdentity};
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

    pub async fn get_transfer_by_id(&self, id: i32) -> anyhow::Result<Option<Transfer>> {
        self.postgres.get_transfer_by_id(id).await
    }

    pub async fn update_transfer_volume(&self, transfer: &Transfer) -> anyhow::Result<(u128, u32)> {
        self.postgres.update_transfer_volume(transfer).await
    }

    pub async fn get_max_identity_change_id(&self) -> anyhow::Result<i32> {
        self.postgres.get_max_identity_change_id().await
    }

    pub async fn get_identity_change_by_id(
        &self,
        id: i32,
    ) -> anyhow::Result<Option<(IdentityChange, Identity, SubIdentity)>> {
        self.postgres.get_identity_change_by_id(id).await
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
        transfer: &Transfer,
        postgres_tx: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<()> {
        // save accounts
        self.postgres
            .save_account(&transfer.from, postgres_tx)
            .await?;
        self.postgres
            .save_account(&transfer.to, postgres_tx)
            .await?;
        // save transfer
        self.postgres
            .save_transfer(block, transfer, postgres_tx)
            .await?;
        Ok(())
    }

    pub async fn save_block(
        &self,
        block: Block,
        identity_changes: Vec<(IdentityChange, Option<Identity>, Option<SubIdentity>)>,
    ) -> anyhow::Result<()> {
        let mut postgres_tx = self.postgres.begin_tx().await?;
        let block = if block.number < REDENOMINATION_BLOCK_NUMBER {
            block.convert_to_old_dot()
        } else {
            block
        };
        self.postgres.save_block(&block, &mut postgres_tx).await?;
        for transfer in block.transfers.iter() {
            self.save_transfer(&block, transfer, &mut postgres_tx)
                .await?;
        }
        for identity_change in identity_changes.iter() {
            self.postgres
                .save_account_with_identity(
                    identity_change.0.address.as_str(),
                    &identity_change.1,
                    &identity_change.2,
                    block.number,
                    &mut postgres_tx,
                )
                .await?;
            self.postgres
                .save_identity_change(
                    &block,
                    &identity_change.0,
                    &identity_change.1,
                    &identity_change.2,
                    &mut postgres_tx,
                )
                .await?;
        }
        self.postgres.commit_tx(postgres_tx).await?;
        Ok(())
    }
}
