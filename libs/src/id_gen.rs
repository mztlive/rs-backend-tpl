use snowflake::SnowflakeIdGenerator;
use std::sync::LazyLock;
use tokio::sync::Mutex;

static ID_GENERATOR: LazyLock<Mutex<SnowflakeIdGenerator>> =
    LazyLock::new(|| Mutex::new(SnowflakeIdGenerator::new(1, 1)));

pub async fn next_id() -> String {
    ID_GENERATOR.lock().await.generate().to_string()
}
