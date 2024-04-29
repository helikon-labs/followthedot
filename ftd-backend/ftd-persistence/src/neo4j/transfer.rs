use crate::neo4j::Neo4JStorage;
use neo4rs::query;

impl Neo4JStorage {
    pub async fn _save_transfer_summary(
        &self,
        from: &str,
        to: &str,
        volume: u128,
        count: u32,
    ) -> anyhow::Result<()> {
        self._graph
            .run(
                query(
                    r#"
                MATCH (from:Account {address: $from})
                MATCH (to:Account {address: $to})
                MERGE (from)-[t:TRANSFER]->(to)
                SET t.volume = $volume, t.count = $count
                "#,
                )
                .param("from", from)
                .param("to", to)
                .param("volume", volume.to_string())
                .param("count", count),
            )
            .await?;
        Ok(())
    }
}
