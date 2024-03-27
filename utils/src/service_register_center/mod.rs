use crate::service_register_center::typos::{Registration, Service};
use abi::config::Config;
use abi::errors::Error;
use axum::async_trait;
use std::collections::HashMap;

mod consul;
mod typos;

type Services = HashMap<String, Service>;
/// the service register discovery center
#[async_trait]
pub trait ServiceRegister {
    /// service register
    async fn register(&self, registration: Registration) -> Result<(), Error>;

    /// service discovery
    async fn discovery(&self) -> Result<Services, Error>;

    /// service deregister
    async fn deregister(&self, service_id: &str) -> Result<(), Error>;

    /// filter
    async fn filter_by_name(&self, name: &str) -> Result<Services, Error>;
}

#[allow(dead_code)]
pub fn service_register_center(config: &Config) -> Box<dyn ServiceRegister> {
    Box::new(consul::Consul::from_config(config))
}
