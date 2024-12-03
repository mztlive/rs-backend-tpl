use nacos_sdk::api::{
    config::{ConfigService, ConfigServiceBuilder},
    props::ClientProps,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct NacosConfig {
    pub addr: String,
    pub namespace: String,
    pub group: String,
    pub data_id: String,
}

/// Nacos 配置客户端
#[derive(Clone)]
pub struct NacosConfigClient {
    config: NacosConfig,
    nacos_cs: Arc<dyn ConfigService>,
}

impl NacosConfigClient {
    pub fn new(addr: &str, namespace: &str, group: &str, data_id: &str) -> crate::Result<Self> {
        let addr = addr.trim_end_matches('/').to_string();

        let config_service =
            ConfigServiceBuilder::new(ClientProps::new().server_addr(&addr).namespace(namespace)).build()?;

        Ok(Self {
            config: NacosConfig {
                addr,
                namespace: namespace.to_string(),
                group: group.to_string(),
                data_id: data_id.to_string(),
            },
            nacos_cs: Arc::new(config_service),
        })
    }

    pub async fn from_config(config: NacosConfig) -> crate::Result<Self> {
        Self::new(&config.addr, &config.namespace, &config.group, &config.data_id)
    }

    pub async fn get_config(&self) -> crate::Result<String> {
        let config = self
            .nacos_cs
            .get_config(self.config.data_id.clone(), self.config.group.clone())
            .await?;

        Ok(config.content().clone())
    }
}
