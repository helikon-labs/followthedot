use actix_cors::Cors;
use actix_web::{dev::Service as _, web, App, HttpResponse, HttpServer};
use async_trait::async_trait;
use ftd_config::Config;
use ftd_persistence::graph::GraphStorage;
use ftd_persistence::relational::RelationalStorage;
use ftd_service::err::InternalServerError;
use ftd_service::Service;
use ftd_subscan_client::SubscanClient;
use futures_util::future::FutureExt;
use lazy_static::lazy_static;
use std::sync::Arc;

mod account;
mod metrics;
mod transfer;

lazy_static! {
    static ref CONFIG: Config = Config::default();
}

pub(crate) type ResultResponse = Result<HttpResponse, InternalServerError>;

#[derive(Clone)]
pub(crate) struct ServiceState {
    relational_storage: Arc<RelationalStorage>,
    graph_storage: Arc<GraphStorage>,
    subscan_client: Arc<SubscanClient>,
}

async fn on_server_ready() {
    log::info!("HTTP service started.");
}

#[derive(Default)]
pub struct APIService;

#[async_trait(?Send)]
impl Service for APIService {
    fn get_metrics_server_addr() -> (&'static str, u16) {
        (
            CONFIG.metrics.host.as_str(),
            CONFIG.metrics.api_service_port,
        )
    }

    async fn run(&'static self) -> anyhow::Result<()> {
        let graph_storage = Arc::new(GraphStorage::new().await?);
        let relational_storage = Arc::new(RelationalStorage::new().await?);
        let subscan_client = Arc::new(SubscanClient::new(&CONFIG)?);

        log::info!("Starting HTTP service.");
        let server = HttpServer::new(move || {
            let _cors = Cors::default()
                .allow_any_origin()
                .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                .allowed_headers(vec![
                    actix_web::http::header::AUTHORIZATION,
                    actix_web::http::header::CONTENT_TYPE,
                ])
                .supports_credentials();

            App::new()
                .app_data(web::Data::new(ServiceState {
                    relational_storage: relational_storage.clone(),
                    graph_storage: graph_storage.clone(),
                    subscan_client: subscan_client.clone(),
                }))
                //.wrap(cors)
                .wrap_fn(|request, service| {
                    metrics::request_counter().inc();
                    metrics::connection_count().inc();
                    let start = std::time::Instant::now();
                    service.call(request).map(move |result| {
                        match &result {
                            Ok(response) => {
                                let status_code = response.response().status();
                                metrics::response_time_ms()
                                    .observe(start.elapsed().as_millis() as f64);
                                metrics::response_status_code_counter(status_code.as_str()).inc();
                            }
                            Err(error) => {
                                let status_code = error.as_response_error().status_code();
                                metrics::response_time_ms()
                                    .observe(start.elapsed().as_millis() as f64);
                                metrics::response_status_code_counter(status_code.as_str()).inc();
                            }
                        }
                        metrics::connection_count().dec();
                        result
                    })
                })
                .service(account::account_search_service)
                .service(account::account_graph_service)
                .service(transfer::transfer_list_service)
        })
        .workers(10)
        .disable_signals()
        .bind(format!(
            "{}:{}",
            CONFIG.api.service_host, CONFIG.api.api_service_port,
        ))?
        .run();
        let (server_result, _) = tokio::join!(server, on_server_ready());
        Ok(server_result?)
    }
}
