use ftd_balance_updater::BalanceUpdater;
use ftd_service::Service;
use lazy_static::lazy_static;

lazy_static! {
    static ref SERVICE: BalanceUpdater = BalanceUpdater;
}

#[tokio::main]
async fn main() {
    SERVICE.start().await;
}
