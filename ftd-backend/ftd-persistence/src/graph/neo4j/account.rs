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

    pub async fn _save_account_with_identity(
        &self,
        address: &str,
        identity: &Option<Identity>,
        sub_identity: &Option<SubIdentity>,
    ) -> anyhow::Result<()> {
        self.save_account(address).await?;
        let (display, legal, web, riot, email, twitter, judgement) =
            if let Some(identity) = identity {
                (
                    identity.display.as_deref(),
                    identity.legal.as_deref(),
                    identity.web.as_deref(),
                    identity.riot.as_deref(),
                    identity.email.as_deref(),
                    identity.twitter.as_deref(),
                    identity.judgement.as_deref(),
                )
            } else {
                (None, None, None, None, None, None, None)
            };
        let sub_display = if let Some(sub_identity) = sub_identity {
            sub_identity.sub_display.as_deref()
        } else {
            None
        };
        self.graph.run(
            query(
                r#"
                    MATCH (a:Account)
                    WHERE a.address = $address
                    SET a.display = $display, a.legal = $legal, a.web = $web, a.riot = $riot, a.email = $email, a.twitter = $twitter, a.judgement = $judgement, a.sub_display = $sub_display
                    "#,
            )
            .param("address", address)
            .param("display", display)
            .param("legal", legal)
            .param("web", web)
            .param("riot", riot)
            .param("email", email)
            .param("twitter", twitter)
            .param("judgement", judgement)
            .param("sub_display", sub_display),
        )
        .await?;
        if let Some(sub_identity) = sub_identity {
            if let Some(super_address) = &sub_identity.super_address {
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
                        .param("super_address", super_address.as_str()),
                    )
                    .await?;
            }
        }
        Ok(())
    }
}
