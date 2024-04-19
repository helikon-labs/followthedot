use crate::neo4j::Neo4JStorage;
use ftd_types::substrate::{Identity, SubIdentity};
use neo4rs::{query, Txn};

impl Neo4JStorage {
    pub async fn begin_tx(&self) -> anyhow::Result<Txn> {
        match self.graph.start_txn().await {
            Ok(tx) => Ok(tx),
            Err(err) => Err(err.into()),
        }
    }

    pub async fn commit_tx(&self, tx: Txn) -> anyhow::Result<()> {
        match tx.commit().await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }

    pub async fn save_account(&self, address: &str, tx: &mut Txn) -> anyhow::Result<()> {
        tx.run(query("MERGE (a:Account {address: $address})").param("address", address))
            .await?;
        Ok(())
    }

    pub async fn save_account_with_identity(
        &self,
        address: &str,
        identity: &Option<Identity>,
        sub_identity: &Option<SubIdentity>,
        tx: &mut Txn,
    ) -> anyhow::Result<()> {
        self.save_account(address, tx).await?;
        let display = if let Some(identity) = identity {
            identity.display.as_deref()
        } else {
            None
        };
        let sub_display = if let Some(sub_identity) = sub_identity {
            sub_identity.sub_display.as_deref()
        } else {
            None
        };
        tx.run(
            query(
                r#"
                    MATCH (a:Account)
                    WHERE a.address = $address
                    SET a.display = $display, a.sub_display = $sub_display
                    "#,
            )
            .param("address", address)
            .param("display", display)
            .param("sub_display", sub_display),
        )
        .await?;
        if let Some(sub_identity) = sub_identity {
            if let Some(super_address) = &sub_identity.super_address {
                self.save_account(super_address, tx).await?;
                tx.run(
                    query(
                        r#"
                            MATCH (a:Account {address: $address})-[s:SUB_OF]->(:Account)
                            DELETE s
                            "#,
                    )
                    .param("address", address),
                )
                .await?;
                tx.run(
                    query(
                        r#"
                            MATCH (a:Account {address: $address})
                            MATCH (b:Account {address: $super_address})
                            MERGE (a)-[:SUB_OF]->(b)
                            "#,
                    )
                    .param("address", address)
                    .param("super_address", super_address.as_str()),
                )
                .await?;
            }
        }
        Ok(())
    }
}
