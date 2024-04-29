use ftd_graph_updater::GraphUpdater;
use ftd_service::Service;
use lazy_static::lazy_static;

lazy_static! {
    static ref SERVICE: GraphUpdater = GraphUpdater;
}

#[tokio::main]
async fn main() {
    SERVICE.start().await;
}
