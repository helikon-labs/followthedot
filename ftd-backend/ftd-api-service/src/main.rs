use ftd_api_service::APIService;
use ftd_service::Service;
use lazy_static::lazy_static;

lazy_static! {
    static ref SERVICE: APIService = APIService;
}

#[tokio::main]
async fn main() {
    SERVICE.start().await;
}
