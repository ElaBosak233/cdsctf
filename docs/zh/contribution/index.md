# 快速上手

## 开发环境

### Rust

请参照 [Rust 官方文档](https://www.rust-lang.org/zh-CN/learn/get-started) 安装最新版本的 Rust。

### Node.js

请参照 [Node.js 官方文档](https://nodejs.org/zh-cn/download/) 安装最新版本的 Node.js。

### Docker

请参照 [Docker 官方文档](https://docs.docker.com/get-docker/) 安装最新版本的 Docker 和 Docker Compose。

## 目录结构

> 此处只展示有必要说明的目录结构

```
├── docs  # 文档
│   └── ...
├── deploys
│   ├── docker-compose.yml  # 生产环境
│   └── docker-compose.dev.yml  # 开发环境
├── src  # 后端
│   └── ...
├── web  # 前端
│   └── ...
├── Cargo.toml  # Rust 项目配置文件
└── Dockerfile  # Docker 镜像构建文件
```

## 编译运行

先克隆仓库，或者可以先复刻再克隆

```bash
git clone https://github.com/ElaBosak233/cloudsdale.git
```

进入仓库

```bash
cd cloudsdale
```

使用 Docker Compose 运行开发时的依赖服务（数据库、消息队列、缓存）

```bash
docker compose -f deploys/docker-compose.dev.yml up
```

### 使用 Cargo 编译/运行后端

```bash
cargo build
```

你可以使用这条命令运行后端（请务必在根目录准备好符合要求的 `application.yml`）

```bash
cargo run --bin cloudsdale
```

::: tip 使用 Windows 进行开发时，遇到无法编译 `aws-lc-sys` 的问题

1. 安装 [CMake](https://cmake.org/download/)
2. 安装 [NASM](https://www.nasm.us/)
3. 安装 [Clang](https://clang.llvm.org/)

并且需要保证 `CMake`、`NASM`、`Clang` 的路径在 `PATH` 环境变量中

:::

### 使用 NPM 编译/运行前端

先进入前端目录

```bash
cd web
```

安装依赖

```bash
npm install
```

你可以使用这条命令运行前端

```bash
npm run dev
```