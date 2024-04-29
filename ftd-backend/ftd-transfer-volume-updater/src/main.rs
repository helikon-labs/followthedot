use ftd_service::Service;
use ftd_transfer_volume_updater::TransferVolumeUpdater;
use lazy_static::lazy_static;

lazy_static! {
    static ref SERVICE: TransferVolumeUpdater = TransferVolumeUpdater;
}

#[tokio::main]
async fn main() {
    SERVICE.start().await;
}
