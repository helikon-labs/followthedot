use crate::relational::postgres::PostgreSQLStorage;
use ftd_types::subscan::{
    SubscanAccount, SubscanAccountDisplay, SubscanMerkleScienceAccountInfo,
    SubscanParentAccountDisplay,
};

type SubscanAccountRow = (
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<bool>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<bool>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
);

fn row_into_subscan_account(row: &SubscanAccountRow) -> SubscanAccount {
    let merkle = if let Some(merkle_address_type) = &row.9 {
        row.12
            .as_ref()
            .map(|merkle_tag_name| SubscanMerkleScienceAccountInfo {
                address_type: merkle_address_type.clone(),
                tag_type: row.10.clone(),
                tag_subtype: row.11.clone(),
                tag_name: merkle_tag_name.clone(),
            })
    } else {
        None
    };
    let parent = row
        .5
        .as_ref()
        .map(|parent_address| SubscanParentAccountDisplay {
            address: parent_address.clone(),
            display: row.6.clone(),
            sub_symbol: row.7.clone(),
            identity: row.8,
        });

    let account_display = SubscanAccountDisplay {
        address: row.0.clone(),
        account_index: row.2.clone(),
        display: row.3.clone(),
        identity: row.4,
        parent,
        merkle,
    };
    SubscanAccount {
        address: row.0.clone(),
        display: row.1.clone(),
        account_display,
    }
}

impl PostgreSQLStorage {
    pub async fn save_subscan_account(&self, account: &SubscanAccount) -> anyhow::Result<String> {
        let parent = account.account_display.parent.as_ref();
        let merkle = account.account_display.merkle.as_ref();
        let account_display = account
            .account_display
            .display
            .as_ref()
            .map(|d| d.replace('\0', ""));
        let result: (String,) = sqlx::query_as(
            r#"
            INSERT INTO ftd_subscan_account (address, display, account_index, account_display, account_identity, parent_address, parent_display, parent_sub_symbol, parent_identity, merkle_science_address_type, merkle_science_tag_type, merkle_science_tag_sub_type, merkle_science_tag_name)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            ON CONFLICT (address) DO UPDATE
            SET
                display = EXCLUDED.display,
                account_index = EXCLUDED.account_index,
                account_display = EXCLUDED.account_display,
                account_identity = EXCLUDED.account_identity,
                parent_address = EXCLUDED.parent_address,
                parent_display = EXCLUDED.parent_display,
                parent_sub_symbol = EXCLUDED.parent_sub_symbol,
                parent_identity = EXCLUDED.parent_identity,
                merkle_science_address_type = EXCLUDED.merkle_science_address_type,
                merkle_science_tag_type = EXCLUDED.merkle_science_tag_type,
                merkle_science_tag_sub_type = EXCLUDED.merkle_science_tag_sub_type,
                merkle_science_tag_name = EXCLUDED.merkle_science_tag_name,
                updated_at = now()
            RETURNING address
            "#,
        )
            .bind(&account.address)
            .bind(&account.display)
            .bind(account.account_display.account_index.as_ref())
            .bind(account_display)
            .bind(account.account_display.identity)
            .bind(parent.as_ref().map(|parent| &parent.address))
            .bind(parent.as_ref().map(|parent| &parent.display))
            .bind(parent.as_ref().map(|parent| &parent.sub_symbol))
            .bind(parent.as_ref().map(|parent| parent.identity))
            .bind(merkle.as_ref().map(|merkle| &merkle.address_type))
            .bind(merkle.as_ref().map(|merkle| &merkle.tag_type))
            .bind(merkle.as_ref().map(|merkle| &merkle.tag_subtype))
            .bind(merkle.map(|merkle| &merkle.tag_name))
            .fetch_one(&self.connection_pool)
            .await?;
        Ok(result.0)
    }

    pub async fn get_subscan_account(
        &self,
        address: &str,
    ) -> anyhow::Result<Option<SubscanAccount>> {
        let maybe_row: Option<SubscanAccountRow> = sqlx::query_as(
            r#"
            SELECT address, display, account_index, account_display, account_identity, parent_address, parent_display, parent_sub_symbol, parent_identity, merkle_science_address_type, merkle_science_tag_type, merkle_science_tag_sub_type, merkle_science_tag_name
            FROM ftd_subscan_account
            WHERE address = $1
            "#,
        )
            .bind(address)
            .fetch_optional(&self.connection_pool)
            .await?;
        Ok(maybe_row.as_ref().map(row_into_subscan_account))
    }
}
