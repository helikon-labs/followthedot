use crate::PostgreSQLStorage;
use ftd_types::substrate::{Identity, SubIdentity};

impl PostgreSQLStorage {
    pub async fn save_account(
        &self,
        address: &str,
        identity: &Option<Identity>,
        sub_identity: &Option<SubIdentity>,
    ) -> anyhow::Result<Option<String>> {
        let maybe_result: Option<(String,)> = sqlx::query_as(
            r#"
            INSERT INTO ftd_account (address, display, legal, web, riot, email, twitter, judgement, super_address, sub_display)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
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
        .fetch_optional(&self.connection_pool)
        .await?;
        if let Some(result) = maybe_result {
            Ok(Some(result.0))
        } else {
            Ok(None)
        }
    }
}
