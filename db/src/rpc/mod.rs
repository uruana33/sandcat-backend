use std::sync::Arc;

use tonic::server::NamedService;
use tonic::transport::Server;
use tracing::info;

use abi::config::Config;
use abi::errors::Error;
use abi::message::db_service_server::DbServiceServer;
use abi::message::MsgToDb;
use cache::Cache;
use utils::typos::{GrpcHealthCheck, Registration};

use crate::relation_db;
use crate::relation_db::{GroupStoreRepo, MsgRecBoxRepo, MsgStoreRepo};

pub mod service;
pub use service::*;

/// DbRpcService contains the postgres trait, mongodb trait and redis trait
pub struct DbRpcService {
    db: Arc<Box<dyn MsgStoreRepo>>,
    mongodb: Arc<Box<dyn MsgRecBoxRepo>>,
    group: Arc<Box<dyn GroupStoreRepo>>,
    cache: Arc<Box<dyn Cache>>,
}

impl DbRpcService {
    pub async fn new(config: &Config) -> Self {
        Self {
            db: Arc::new(relation_db::msg_store_repo(config).await),
            mongodb: Arc::new(relation_db::msg_rec_box_repo(config).await),
            group: Arc::new(relation_db::group_repo(config).await),
            cache: Arc::new(cache::cache(config)),
        }
    }

    pub async fn start(config: &Config) -> Result<(), Error> {
        // register service
        Self::register_service(config).await?;
        info!("<db> rpc service health check started");

        // open health check
        let (mut reporter, health_service) = tonic_health::server::health_reporter();
        reporter
            .set_serving::<DbServiceServer<DbRpcService>>()
            .await;
        info!("<db> rpc service register to service register center");

        let db_rpc = DbRpcService::new(config).await;
        let service = DbServiceServer::new(db_rpc);
        info!(
            "<db> rpc service started at {}",
            config.rpc.db.rpc_server_url()
        );

        Server::builder()
            .add_service(health_service)
            .add_service(service)
            .serve(config.rpc.db.rpc_server_url().parse().unwrap())
            .await
            .unwrap();
        Ok(())
    }

    async fn register_service(config: &Config) -> Result<(), Error> {
        // register service to service register center
        let center = utils::service_register_center(config);
        let grpc = format!(
            "{}/{}",
            config.rpc.db.rpc_server_url(),
            <DbServiceServer<DbRpcService> as NamedService>::NAME
        );
        let check = GrpcHealthCheck {
            name: config.rpc.db.name.clone(),
            grpc,
            grpc_use_tls: config.rpc.db.grpc_health_check.grpc_use_tls,
            interval: format!("{}s", config.rpc.db.grpc_health_check.interval),
        };
        let registration = Registration {
            id: format!("{}-{}", utils::get_host_name()?, &config.rpc.db.name),
            name: config.rpc.db.name.clone(),
            address: config.rpc.db.host.clone(),
            port: config.rpc.db.port,
            tags: config.rpc.db.tags.clone(),
            check: Some(check),
        };
        center.register(registration).await?;
        Ok(())
    }

    pub async fn handle_message(&self, message: MsgToDb) -> Result<(), Error> {
        // task 1 save message to postgres
        self.db.save_message(message.clone()).await?;
        // task 2 save message to mongodb
        // todo think about if the collection name should be here
        self.mongodb.save_message(message, "".to_string()).await?;

        Ok(())
    }
}
