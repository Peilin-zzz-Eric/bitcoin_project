# 1. 使用 Rust 官方镜像作为构建阶段的基础镜像
FROM rust:slim-buster as build-stage

# 2. 设置工作目录
WORKDIR /app

# 3. 安装必要的依赖（如 OpenSSL 相关依赖）
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev build-essential ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

# 4. 复制 Cargo.toml 和 Cargo.lock 以安装依赖
COPY Cargo.toml Cargo.lock ./

# 5. 安装依赖项并进行缓存
RUN cargo fetch

# 6. 复制项目的所有文件
COPY . .

# 7. 编译项目为 release 模式
RUN cargo build --release

# 8. 使用 debian:bullseye-slim 镜像作为运行时基础镜像
FROM debian:bullseye-slim

# 9. 安装 OpenSSL 1.1 和其他必要的依赖库
RUN apt-get update && \
    apt-get install -y openssl libssl1.1 ca-certificates curl && \
    rm -rf /var/lib/apt/lists/

# 10. 设置动态链接库路径
ENV LD_LIBRARY_PATH=/usr/lib:/usr/local/lib:/lib

# 11. 复制编译好的二进制文件
COPY --from=build-stage /app/target/release/rust_bitcoin /usr/local/bin/

# 12. 暴露应用使用的端口
EXPOSE 8080

# 13. 定义启动命令
CMD ["rust_bitcoin"]
