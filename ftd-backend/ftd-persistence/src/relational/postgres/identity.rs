use super::PostgreSQLStorage;
use ftd_types::substrate::identity::{Identity, SubIdentity};
use sqlx::{Postgres, Transaction};

impl PostgreSQLStorage {
    pub async fn delete_all_identities(
        &self,
        tx: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM ftd_identity")
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    pub async fn save_identity(
        &self,
        block_hash: &str,
        block_number: u64,
        identity: &Identity,
        tx: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<String> {
        let address = identity.account_id.to_ss58_check();
        self.save_account(address.as_str(), tx).await?;
        let result: (String,) = sqlx::query_as(
            r#"
            INSERT INTO ftd_identity (address, block_hash, block_number, display, legal, web, riot, email, twitter, is_confirmed)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (address) DO UPDATE
            SET
                block_hash = EXCLUDED.block_hash,
                block_number = EXCLUDED.block_number,
                display = EXCLUDED.display,
                legal = EXCLUDED.legal,
                web = EXCLUDED.web,
                riot = EXCLUDED.riot,
                email = EXCLUDED.email,
                twitter = EXCLUDED.twitter,
                is_confirmed = EXCLUDED.is_confirmed,
                updated_at = now()
            RETURNING address
            "#,
        )
            .bind(&address)
            .bind(block_hash)
            .bind(block_number as i64)
            .bind(&identity.display)
            .bind(&identity.legal)
            .bind(&identity.web)
            .bind(&identity.riot)
            .bind(&identity.email)
            .bind(&identity.twitter)
            .bind(identity.is_confirmed)
            .fetch_one(&mut **tx)
            .await?;
        Ok(result.0)
    }

    pub async fn delete_all_sub_identities(
        &self,
        tx: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM ftd_sub_identity")
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    pub async fn save_sub_identity(
        &self,
        block_hash: &str,
        block_number: u64,
        sub_identity: &SubIdentity,
        tx: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<String> {
        let address = sub_identity.account_id.to_ss58_check();
        let super_address = sub_identity.super_account_id.to_ss58_check();
        self.save_account(address.as_str(), tx).await?;
        self.save_account(super_address.as_str(), tx).await?;
        let result: (String,) = sqlx::query_as(
            r#"
            INSERT INTO ftd_sub_identity (address, block_hash, block_number, super_address, sub_display)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (address) DO UPDATE
            SET
                block_hash = EXCLUDED.block_hash,
                block_number = EXCLUDED.block_number,
                super_address = EXCLUDED.super_address,
                sub_display = EXCLUDED.sub_display,
                updated_at = now()
            RETURNING address
            "#,
        )
            .bind(&address)
            .bind(block_hash)
            .bind(block_number as i64)
            .bind(&super_address)
            .bind(sub_identity.sub_display.as_deref())
            .fetch_one(&mut **tx)
            .await?;
        Ok(result.0)
    }

    /*
    pub async fn get_identity_change_by_id(
        &self,
        id: i32,
    ) -> anyhow::Result<Option<(IdentityChange, Identity, SubIdentity)>> {
        #[allow(clippy::type_complexity)]
        let result: Option<(i32, i32, i32, String, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>)> = sqlx::query_as(
            r#"
            SELECT extrinsic_index, extrinsic_event_index, event_index, address, display, legal, web, riot, email, twitter, judgement, super_address, sub_display
            FROM ftd_identity_change
            WHERE id = $1
            "#,
        )
            .bind(id)
            .fetch_optional(&self.connection_pool)
            .await?;
        if let Some(row) = result {
            let identity_change = IdentityChange {
                extrinsic_index: row.0 as u16,
                extrinsic_event_index: row.1 as u16,
                event_index: row.2 as u16,
                address: row.3.clone(),
            };
            let identity = Identity {
                display: row.4,
                legal: row.5,
                web: row.6,
                riot: row.7,
                email: row.8,
                twitter: row.9,
                judgement: row.10,
            };
            let sub_identity = SubIdentity {
                super_address: row.11,
                sub_display: row.12,
            };
            Ok(Some((identity_change, identity, sub_identity)))
        } else {
            Ok(None)
        }
    }
     */
}
