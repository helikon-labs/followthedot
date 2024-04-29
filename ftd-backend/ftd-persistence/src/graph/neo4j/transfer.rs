use super::Neo4JStorage;
use ftd_types::graph::TransferVolume;
use ftd_types::substrate::event::Transfer;
use neo4rs::{query, Relation};

impl Neo4JStorage {
    async fn get_transfer_volume(&self, from: &str, to: &str) -> anyhow::Result<TransferVolume> {
        let mut result = self
            .graph
            .execute(
                query(
                    r#"
                MATCH (from:Account {address: $from}), (to:Account {address: $to})
                MERGE (from)-[t:TRANSFER]->(to)
                ON CREATE SET t.volume = '0', t.count = '0'
                RETURN t
                "#,
                )
                .param("from", from)
                .param("to", to),
            )
            .await?;
        let row = result.next().await?.unwrap();
        let relation = row.get::<Relation>("t")?;
        Ok(TransferVolume {
            from: from.to_string(),
            to: to.to_string(),
            count: relation.get::<String>("volume")?.parse()?,
            volume: relation.get::<String>("count")?.parse()?,
        })
    }

    pub async fn update_transfer_volume(&self, transfer: &Transfer) -> anyhow::Result<()> {
        let transfer_volume = self
            .get_transfer_volume(transfer.from.as_str(), transfer.to.as_str())
            .await?;
        self.graph
            .run(
                query(
                    r#"
                MATCH (from:Account {address: $from})-[t:TRANSFER]->(to:Account {address: $to})
                SET t.volume = $volume, t.count = $count
                "#,
                )
                .param("from", transfer.from.as_str())
                .param("to", transfer.to.as_str())
                .param(
                    "volume",
                    (transfer_volume.count + transfer.amount).to_string(),
                )
                .param("count", (transfer_volume.volume + 1).to_string()),
            )
            .await?;
        Ok(())
    }
}
