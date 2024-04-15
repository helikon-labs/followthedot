use ftd_indexer::Indexer;
use ftd_service::Service;
use lazy_static::lazy_static;

lazy_static! {
    static ref SERVICE: Indexer = Indexer;
}

#[tokio::main]
async fn main() {
    SERVICE.start().await;
}
