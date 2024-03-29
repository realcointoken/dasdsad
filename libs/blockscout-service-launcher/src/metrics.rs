use actix_web::{App, HttpServer};
use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};
use std::net::SocketAddr;

#[derive(Clone)]
pub struct Metrics {
    metrics_middleware: PrometheusMetrics,
    http_middleware: PrometheusMetrics,
}

impl Metrics {
    pub fn new(service_name: &str, endpoint: &str) -> Self {
        let registry = prometheus::default_registry();
        let metrics_middleware =
            PrometheusMetricsBuilder::new(&(service_name.to_owned() + "_metrics"))
                .registry(registry.clone())
                .endpoint(endpoint)
                .build()
                .unwrap();
        let http_middleware = PrometheusMetricsBuilder::new(service_name)
            .registry(registry.clone())
            .build()
            .unwrap();

        Self {
            metrics_middleware,
            http_middleware,
        }
    }

    pub fn http_middleware(&self) -> &PrometheusMetrics {
        &self.http_middleware
    }

    pub fn run_server(self, addr: SocketAddr) -> actix_web::dev::Server {
        tracing::info!(addr = ?addr, "starting metrics server");
        HttpServer::new(move || App::new().wrap(self.metrics_middleware.clone()))
            .bind(addr)
            .unwrap()
            .run()
    }
}
