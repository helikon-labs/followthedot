use ftd_service::Service;
use ftd_subscan_account_fetcher::SubscanAccountFetcher;
use lazy_static::lazy_static;

lazy_static! {
    static ref SERVICE: SubscanAccountFetcher = SubscanAccountFetcher;
}

#[tokio::main]
async fn main() {
    SERVICE.start().await;
}
