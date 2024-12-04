#!/bin/bash

# 定义所有的 crate
CRATES=(
    "api"
    "entity-core"
    "entity-macros"
    "rbac"
    "database"
    "entities"
    "task"
    "services"
    "storage"
    "libs"
    "config"
    "container"
)

# 定义要替换为workspace依赖的包
WORKSPACE_DEPS=(
    "tokio"
    "serde"
    "serde_json"
    "toml"
    "thiserror"
    "anyhow"
    "mongodb"
    "async-trait"
    "chrono"
    "log"
    "env_logger"
    "validator"
    "hyper"
    "axum"
    "tower"
    "tower-http"
    "md5"
    "sha2"
    "base64"
    "rs-snowflake"
    "casbin"
    "tokio-cron-scheduler"
)

# 遍历每个 crate
for crate in "${CRATES[@]}"; do
    if [ -f "$crate/Cargo.toml" ]; then
        echo "Processing $crate/Cargo.toml..."
        
        # 创建备份
        cp "$crate/Cargo.toml" "$crate/Cargo.toml.bak"
        
        # 替换为workspace依赖
        for dep in "${WORKSPACE_DEPS[@]}"; do
            # 替换常见的依赖模式
            sed -i "s/$dep = { version = \"[0-9.]*\"/$dep = { workspace = true/g" "$crate/Cargo.toml"
            sed -i "s/$dep = { version = \"[0-9.]*\", features = \[/$dep = { workspace = true, features = \[/g" "$crate/Cargo.toml"
            sed -i "s/$dep = \"[0-9.]*\"/$dep = { workspace = true }/g" "$crate/Cargo.toml"
        done
        
        echo "Updated $crate/Cargo.toml (backup created as Cargo.toml.bak)"
    else
        echo "Warning: $crate/Cargo.toml not found"
    fi
done

echo "Done!"

echo "Note: Please check the updated Cargo.toml files and their backups to ensure the changes are correct."
echo "You can use 'diff' to compare the changes:"
echo "  diff Cargo.toml Cargo.toml.bak" 