use super::Neo4JStorage;
use ftd_types::graph::TransferVolume;
use ftd_types::substrate::event::TransferEvent;
use neo4rs::{query, Node, Relation, Txn};

impl Neo4JStorage {
    async fn get_transfer_volume(
        &self,
        tx: &mut Txn,
        from: &str,
        to: &str,
    ) -> anyhow::Result<TransferVolume> {
        let mut result = tx
            .execute(
                query(
                    r#"
                MATCH (from:Account {address: $from}), (to:Account {address: $to})
                MERGE (from)-[t:TRANSFER]->(to)
                ON CREATE SET t.volume = '0', t.count = '0'
                RETURN t, id(t) as t_id
                "#,
                )
                .param("from", from)
                .param("to", to),
            )
            .await?;
        let row = result.next(tx).await?.unwrap();
        let transfer_volume = row.get::<Relation>("t")?;
        let transfer_volume_id = row.get::<u64>("t_id")?;
        let count = transfer_volume.get::<String>("count")?.parse()?;
        let volume = transfer_volume.get::<String>("volume")?.parse()?;
        Ok(TransferVolume {
            id: transfer_volume_id,
            from: from.to_string(),
            to: to.to_string(),
            count,
            volume,
        })
    }

    pub async fn update_transfer_volume(
        &self,
        tx: &mut Txn,
        transfer: &TransferEvent,
    ) -> anyhow::Result<()> {
        let transfer_volume = self
            .get_transfer_volume(tx, transfer.from.as_str(), transfer.to.as_str())
            .await?;
        tx.run(
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
                (transfer_volume.volume + transfer.amount).to_string(),
            )
            .param("count", (transfer_volume.count + 1).to_string()),
        )
        .await?;
        Ok(())
    }

    pub async fn get_transfer_volumes_for_account(
        &self,
        address: &str,
        limit: u16,
    ) -> anyhow::Result<Vec<TransferVolume>> {
        let mut result = self
            .graph
            .execute(
                query(
                    r#"
                MATCH (a:Account)-[t:TRANSFER]-(b:Account)
                WHERE a.address = $address
                RETURN b, t, id(t) as t_id, (startNode(t) = a) as is_from_a
                LIMIT $limit
                "#,
                )
                .param("address", address)
                .param("limit", limit),
            )
            .await?;
        let mut transfer_volumes = Vec::new();
        while let Some(row) = result.next().await? {
            let other = row.get::<Node>("b")?;
            let other_address = other.get::<String>("address")?;
            let transfer_volume = row.get::<Relation>("t")?;
            let transfer_volume_id = row.get::<u64>("t_id")?;
            let is_from_a = row.get::<bool>("is_from_a")?;
            let (from, to) = if is_from_a {
                (address, other_address.as_str())
            } else {
                (other_address.as_str(), address)
            };
            let count = transfer_volume.get::<String>("count")?.parse()?;
            let volume = transfer_volume.get::<String>("volume")?.parse()?;
            transfer_volumes.push(TransferVolume {
                id: transfer_volume_id,
                from: from.to_string(),
                to: to.to_string(),
                count,
                volume,
            });
        }
        Ok(transfer_volumes)
    }
}
