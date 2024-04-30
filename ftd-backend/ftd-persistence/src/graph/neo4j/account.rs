use super::Neo4JStorage;
use ftd_types::substrate::{Identity, SubIdentity};
use neo4rs::query;

impl Neo4JStorage {
    pub async fn save_account(&self, address: &str) -> anyhow::Result<()> {
        self.graph
            .run(query("MERGE (a:Account {address: $address})").param("address", address))
            .await?;
        Ok(())
    }

    pub async fn save_account_with_identity(
        &self,
        address: &str,
        identity: &Identity,
        sub_identity: &SubIdentity,
    ) -> anyhow::Result<()> {
        self.save_account(address).await?;
        self.graph.run(
            query(
                r#"
                    MATCH (a:Account)
                    WHERE a.address = $address
                    SET a.display = $display, a.legal = $legal, a.web = $web, a.riot = $riot, a.email = $email, a.twitter = $twitter, a.judgement = $judgement, a.sub_display = $sub_display
                    "#,
            )
            .param("address", address)
            .param("display", identity.display.as_deref())
            .param("legal", identity.legal.as_deref())
            .param("web", identity.web.as_deref())
            .param("riot", identity.riot.as_deref())
            .param("email", identity.email.as_deref())
            .param("twitter", identity.twitter.as_deref())
            .param("judgement", identity.judgement.as_deref())
            .param("sub_display", sub_identity.sub_display.as_deref()),
        )
        .await?;
        if let Some(super_address) = sub_identity.super_address.as_deref() {
            self.save_account(super_address).await?;
            self.graph
                .run(
                    query(
                        r#"
                            MATCH (a:Account {address: $address})-[s:SUB_OF]->(:Account)
                            DELETE s
                            "#,
                    )
                    .param("address", address),
                )
                .await?;
            self.graph
                .run(
                    query(
                        r#"
                            MATCH (a:Account {address: $address})
                            MATCH (b:Account {address: $super_address})
                            MERGE (a)-[:SUB_OF]->(b)
                            "#,
                    )
                    .param("address", address)
                    .param("super_address", super_address),
                )
                .await?;
        }
        Ok(())
    }
}
