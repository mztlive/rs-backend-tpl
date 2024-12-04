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
        
        # 创建模板文件
        cp "$crate/Cargo.toml" "$crate/Cargo.toml.liquid"
        
        # 添加作者和邮箱信息(如果没有的话)
        if ! grep -q "authors" "$crate/Cargo.toml.liquid"; then
            sed -i '/\[package\]/a authors = ["{{author}} <{{email}}>"]\n' "$crate/Cargo.toml.liquid"
        fi
        
        # 替换为workspace依赖
        for dep in "${WORKSPACE_DEPS[@]}"; do
            # 替换常见的依赖模式
            sed -i "s/$dep = { version = \"[0-9.]*\"/$dep = { workspace = true/g" "$crate/Cargo.toml.liquid"
            sed -i "s/$dep = { version = \"[0-9.]*\", features = \[/$dep = { workspace = true, features = \[/g" "$crate/Cargo.toml.liquid"
            sed -i "s/$dep = \"[0-9.]*\"/$dep = { workspace = true }/g" "$crate/Cargo.toml.liquid"
        done
        
        echo "Created $crate/Cargo.toml.liquid"
    else
        echo "Warning: $crate/Cargo.toml not found"
    fi
done

echo "Done!" 