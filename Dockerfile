# 构建阶段
FROM rust:1.75-slim-bullseye as builder

# 修改apt源为阿里云镜像
RUN sed -i 's/deb.debian.org/mirrors.aliyun.com/g' /etc/apt/sources.list && \
    sed -i 's/security.debian.org/mirrors.aliyun.com/g' /etc/apt/sources.list

# 配置Rust crates.io镜像源
ENV RUSTUP_DIST_SERVER="https://rsproxy.cn"
ENV RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
RUN echo '[source.crates-io]' > $CARGO_HOME/config.toml && \
    echo 'replace-with = "rsproxy"' >> $CARGO_HOME/config.toml && \
    echo '[source.rsproxy]' >> $CARGO_HOME/config.toml && \
    echo 'registry = "https://rsproxy.cn/crates.io-index"' >> $CARGO_HOME/config.toml && \
    echo '[registries.rsproxy]' >> $CARGO_HOME/config.toml && \
    echo 'index = "https://rsproxy.cn/crates.io-index"' >> $CARGO_HOME/config.toml

# 安装必要的构建依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 创建新的空项目
WORKDIR /usr/src/app
RUN cargo new --bin api
RUN cargo new --lib entity-base
RUN cargo new --lib entity-derive
RUN cargo new --lib rbac
RUN cargo new --lib database
RUN cargo new --lib entities
RUN cargo new --lib task
RUN cargo new --lib services
RUN cargo new --lib storage
RUN cargo new --lib libs
RUN cargo new --lib config
RUN cargo new --lib container

# 复制所有的Cargo.toml
COPY Cargo.toml ./
COPY api/Cargo.toml ./api/
COPY entity-base/Cargo.toml ./entity-base/
COPY entity-derive/Cargo.toml ./entity-derive/
COPY rbac/Cargo.toml ./rbac/
COPY database/Cargo.toml ./database/
COPY entities/Cargo.toml ./entities/
COPY task/Cargo.toml ./task/
COPY services/Cargo.toml ./services/
COPY storage/Cargo.toml ./storage/
COPY libs/Cargo.toml ./libs/
COPY config/Cargo.toml ./config/
COPY container/Cargo.toml ./container/

# 构建依赖
RUN cargo build --release
RUN rm src/*.rs && rm */src/*.rs

# 复制源代码
COPY . .

# 构建项目
RUN cargo build --release

# 运行阶段
FROM debian:bullseye-slim

# 修改apt源为阿里云镜像
RUN sed -i 's/deb.debian.org/mirrors.aliyun.com/g' /etc/apt/sources.list && \
    sed -i 's/security.debian.org/mirrors.aliyun.com/g' /etc/apt/sources.list

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    supervisor \
    && rm -rf /var/lib/apt/lists/*

# 创建非root用户
RUN useradd -m -u 1000 -U app

# 创建必要的目录
RUN mkdir -p /uploads && chown app:app /uploads

WORKDIR /app

# 复制编译好的二进制文件
COPY --from=builder --chown=app:app /usr/src/app/target/release/api ./
COPY --from=builder --chown=app:app /usr/src/app/target/release/task ./

# 添加supervisor配置
COPY --chown=app:app supervisord.conf /etc/supervisor/conf.d/supervisord.conf

# 切换到非root用户
USER app

# 设置环境变量
ENV RUST_LOG=info

# 暴露端口
EXPOSE 10001

# 使用supervisor启动服务
CMD ["/usr/bin/supervisord", "-c", "/etc/supervisor/conf.d/supervisord.conf"] 