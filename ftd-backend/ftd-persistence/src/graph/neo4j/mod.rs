use ftd_config::Config;
use neo4rs::{ConfigBuilder, Graph, Txn};

pub mod account;
pub mod state;
pub mod transfer;

pub(crate) struct Neo4JStorage {
    graph: Graph,
}

impl Neo4JStorage {
    pub async fn new(config: &Config) -> anyhow::Result<Neo4JStorage> {
        log::info!("Establishing Neo4J connection...");
        let uri = format!("{}:{}", config.neo4j.host, config.neo4j.port);
        let config = ConfigBuilder::new()
            .user(&config.neo4j.username)
            .password(&config.neo4j.password)
            .db(config.neo4j.database_name.as_str())
            .uri(&uri)
            .build()?;
        let graph = Graph::connect(config).await?;
        log::info!("Neo4J connection established.");
        Ok(Neo4JStorage { graph })
    }

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
}
