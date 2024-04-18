use super::PostgreSQLStorage;
use ftd_types::substrate::{Identity, SubIdentity};
use sqlx::{Postgres, Transaction};

impl PostgreSQLStorage {
    pub async fn save_account(
        &self,
        address: &str,
        identity: &Option<Identity>,
        sub_identity: &Option<SubIdentity>,
        block_number: u64,
        transaction: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<Option<String>> {
        let more_recent_identity_update_exists: (bool,) = sqlx::query_as(
            r#"
                SELECT EXISTS(
                    SELECT address
                    FROM ftd_account
                    WHERE address = $1 AND identity_updated_at_block_number >= $2
                )
                "#,
        )
        .bind(address)
        .bind(block_number as i64)
        .fetch_one(&self.connection_pool)
        .await?;
        let identity_updated_at_block_number = if identity.is_some() || sub_identity.is_some() {
            Some(block_number)
        } else {
            None
        };
        let maybe_result: Option<(String,)> = if !more_recent_identity_update_exists.0 {
            sqlx::query_as(
                r#"
            INSERT INTO ftd_account (address, display, legal, web, riot, email, twitter, judgement, super_address, sub_display, identity_updated_at_block_number)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (address) DO UPDATE
            SET
                display = EXCLUDED.display,
                legal = EXCLUDED.legal,
                web = EXCLUDED.web,
                riot = EXCLUDED.riot,
                email = EXCLUDED.email,
                twitter = EXCLUDED.twitter,
                judgement = EXCLUDED.judgement,
                super_address = EXCLUDED.super_address,
                sub_display = EXCLUDED.sub_display,
                identity_updated_at_block_number = EXCLUDED.identity_updated_at_block_number,
                updated_at = now()
            RETURNING address
            "#,
            )
                .bind(address)
                .bind(identity.as_ref().map(|identity| &identity.display))
                .bind(identity.as_ref().map(|identity| &identity.legal))
                .bind(identity.as_ref().map(|identity| &identity.web))
                .bind(identity.as_ref().map(|identity| &identity.riot))
                .bind(identity.as_ref().map(|identity| &identity.email))
                .bind(identity.as_ref().map(|identity| &identity.twitter))
                .bind(identity.as_ref().map(|identity| &identity.judgement))
                .bind(sub_identity.as_ref().map(|sub_identity| &sub_identity.super_address))
                .bind(sub_identity.as_ref().map(|sub_identity| &sub_identity.sub_display))
                .bind(identity_updated_at_block_number.map(|n| n as i64))
                .fetch_optional(&mut **transaction)
                .await?
        } else {
            sqlx::query_as(
                r#"
            INSERT INTO ftd_account (address)
            VALUES ($1)
            ON CONFLICT (address) DO NOTHING
            RETURNING address
            "#,
            )
            .bind(address)
            .fetch_optional(&mut **transaction)
            .await?
        };
        if let Some(result) = maybe_result {
            Ok(Some(result.0))
        } else {
            Ok(None)
        }
    }
}
