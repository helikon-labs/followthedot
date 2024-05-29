use super::PostgreSQLStorage;
use ftd_types::api::identity::{Identity, SubIdentity};
use sqlx::{Postgres, Transaction};

type IdentityRow = (
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    bool,
    bool,
);

fn row_into_identity(row: &IdentityRow) -> Identity {
    Identity {
        address: row.0.clone(),
        display: row.1.clone(),
        legal: row.2.clone(),
        web: row.3.clone(),
        riot: row.4.clone(),
        email: row.5.clone(),
        twitter: row.6.clone(),
        is_confirmed: row.7,
        is_invalid: row.8,
    }
}

type SubIdentityRow = (String, String, Option<String>);

fn row_into_sub_identity(row: &SubIdentityRow) -> SubIdentity {
    SubIdentity {
        address: row.0.clone(),
        super_address: row.1.clone(),
        sub_display: row.2.clone(),
    }
}

impl PostgreSQLStorage {
    pub async fn save_account(
        &self,
        address: &str,
        tx: &mut Transaction<'_, Postgres>,
    ) -> anyhow::Result<Option<String>> {
        let maybe_result: Option<(String,)> = sqlx::query_as(
            r#"
            INSERT INTO ftd_account (address)
            VALUES ($1)
            ON CONFLICT (address) DO NOTHING
            RETURNING address
            "#,
        )
        .bind(address)
        .fetch_optional(&mut **tx)
        .await?;
        if let Some(result) = maybe_result {
            Ok(Some(result.0))
        } else {
            Ok(None)
        }
    }

    pub async fn search_identities_by_display(
        &self,
        query: &str,
        limit: u16,
    ) -> anyhow::Result<Vec<Identity>> {
        let rows: Vec<IdentityRow> = sqlx::query_as(
            r#"
            SELECT address, display, legal, web, riot, email, twitter, is_confirmed, is_invalid
            FROM ftd_identity
            WHERE display ILIKE $1
            LIMIT $2
            "#,
        )
        .bind(format!("%{query}%"))
        .bind(limit as i32)
        .fetch_all(&self.connection_pool)
        .await?;

        let identities = rows.iter().map(row_into_identity).collect();
        Ok(identities)
    }

    pub async fn get_identity_by_address(&self, address: &str) -> anyhow::Result<Option<Identity>> {
        let maybe_row: Option<IdentityRow> = sqlx::query_as(
            r#"
            SELECT address, display, legal, web, riot, email, twitter, is_confirmed, is_invalid
            FROM ftd_identity
            WHERE address = $1
            "#,
        )
        .bind(address)
        .fetch_optional(&self.connection_pool)
        .await?;
        Ok(maybe_row.as_ref().map(row_into_identity))
    }

    pub async fn get_sub_identity_by_address(
        &self,
        address: &str,
    ) -> anyhow::Result<Option<SubIdentity>> {
        let maybe_row: Option<SubIdentityRow> = sqlx::query_as(
            r#"
            SELECT address, super_address, sub_display
            FROM ftd_sub_identity
            WHERE address = $1
            "#,
        )
        .bind(address)
        .fetch_optional(&self.connection_pool)
        .await?;
        Ok(maybe_row.as_ref().map(row_into_sub_identity))
    }

    pub async fn get_sub_identities(
        &self,
        super_address: &str,
    ) -> anyhow::Result<Vec<SubIdentity>> {
        let rows: Vec<SubIdentityRow> = sqlx::query_as(
            r#"
            SELECT address, super_address, sub_display
            FROM ftd_sub_identity
            WHERE super_address = $1
            "#,
        )
        .bind(super_address)
        .fetch_all(&self.connection_pool)
        .await?;
        Ok(rows.iter().map(row_into_sub_identity).collect())
    }

    pub async fn search_sub_identities_by_sub_display(
        &self,
        query: &str,
        limit: u16,
    ) -> anyhow::Result<Vec<SubIdentity>> {
        let rows: Vec<SubIdentityRow> = sqlx::query_as(
            r#"
            SELECT address, super_address, sub_display
            FROM ftd_sub_identity
            WHERE sub_display ILIKE $1
            LIMIT $2
            "#,
        )
        .bind(format!("%{query}%"))
        .bind(limit as i32)
        .fetch_all(&self.connection_pool)
        .await?;
        Ok(rows.iter().map(row_into_sub_identity).collect())
    }

    pub async fn search_addresses(&self, query: &str, limit: u16) -> anyhow::Result<Vec<String>> {
        let rows: Vec<(String,)> = sqlx::query_as(
            r#"
            SELECT address
            FROM ftd_account
            WHERE address ILIKE $1
            LIMIT $2
            "#,
        )
        .bind(format!("%{query}%"))
        .bind(limit as i32)
        .fetch_all(&self.connection_pool)
        .await?;
        Ok(rows.iter().map(|row| row.0.clone()).collect())
    }
}
