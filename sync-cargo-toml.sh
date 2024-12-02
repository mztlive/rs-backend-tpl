#!/bin/bash

# 定义所有的 crate
CRATES=(
    "api"
    "entity-base"
    "entity-derive"
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
        
        # 使用工作空间依赖(如果适用)
        sed -i 's/{ version = "1", features = \["full"\] }/{ workspace = true }/g' "$crate/Cargo.toml.liquid"
        sed -i 's/{ version = "1.0", features = \["derive"\] }/{ workspace = true }/g' "$crate/Cargo.toml.liquid"
        sed -i 's/version = "1.0"/workspace = true/g' "$crate/Cargo.toml.liquid"
        sed -i 's/version = "0.1"/workspace = true/g' "$crate/Cargo.toml.liquid"
        
        echo "Created $crate/Cargo.toml.liquid"
    else
        echo "Warning: $crate/Cargo.toml not found"
    fi
done

echo "Done!" 