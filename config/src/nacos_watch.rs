use crate::Result;
use crate::{NacosConfigClient, SafeConfig};

pub struct NacosConfigWatcher {
    config: SafeConfig,
    nacos_client: NacosConfigClient,
}

impl NacosConfigWatcher {
    pub fn new(config: SafeConfig, nacos_client: NacosConfigClient) -> Self {
        Self { config, nacos_client }
    }

    pub async fn start_watch(&self) -> Result<()> {
        let config = self.config.clone();
        let nacos_client = self.nacos_client.clone();

        tokio::spawn(async move {
            loop {
                if let Err(e) = config.reload_from_nacos(&nacos_client).await {
                    log::error!("Failed to reload config from nacos: {}", e);
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            }
        });

        Ok(())
    }
}
