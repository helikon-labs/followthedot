use ftd_identity_updater::IdentityUpdater;
use ftd_service::Service;
use lazy_static::lazy_static;

lazy_static! {
    static ref SERVICE: IdentityUpdater = IdentityUpdater;
}

#[tokio::main]
async fn main() {
    SERVICE.start().await;
}
