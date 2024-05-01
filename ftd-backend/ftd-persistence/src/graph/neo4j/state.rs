use crate::graph::neo4j::Neo4JStorage;
use async_recursion::async_recursion;
use ftd_types::graph::GraphUpdaterState;
use neo4rs::{query, Node, Txn};

impl Neo4JStorage {
    #[async_recursion]
    pub async fn get_state(&self) -> anyhow::Result<GraphUpdaterState> {
        let mut result = self
            .graph
            .execute(query("MERGE (s:State {id: 1}) ON CREATE SET s.lastProcessedTransferId = 0, s.lastProcessedIdentityChangeId = 0 RETURN s"))
            .await?;
        let row = result.next().await?.unwrap();
        let node = row.get::<Node>("s")?;
        Ok(GraphUpdaterState {
            last_processed_transfer_id: node.get("lastProcessedTransferId")?,
            last_processed_identity_change_id: node.get("lastProcessedIdentityChangeId")?,
        })
    }

    pub async fn update_last_processed_transfer_id(
        &self,
        tx: &mut Txn,
        id: i32,
    ) -> anyhow::Result<()> {
        tx.run(
            query("MATCH (s:State {id: 1}) SET s.lastProcessedTransferId = $id").param("id", id),
        )
        .await?;
        Ok(())
    }

    pub async fn update_last_processed_identity_change_id(
        &self,
        tx: &mut Txn,
        id: i32,
    ) -> anyhow::Result<()> {
        tx.run(
            query("MATCH (s:State {id: 1}) SET s.lastProcessedIdentityChangeId = $id")
                .param("id", id),
        )
        .await?;
        Ok(())
    }
}
