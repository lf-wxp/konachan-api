# Konachan API Proxy

[![Rust](https://img.shields.io/badge/Rust-1.88+-dea584?logo=rust)](https://www.rust-lang.org/)
[![Rocket](https://img.shields.io/badge/Rocket-0.5-red)](https://rocket.rs/)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

> [English](README_EN.md) | **中文**

一个用 Rust 编写的 Konachan API 代理服务，基于 Rocket 框架构建。提供 RESTful API 接口，支持图片搜索和代理访问功能。

---

## ✨ 特性

- 🚀 **高性能** - 基于 Rust 和 Rocket 框架，提供卓越的性能表现
- 🔒 **API 密钥认证** - 通过 `x-api-key` 请求头进行身份验证
- 🌐 **CORS 支持** - 可配置跨域资源共享，支持多域名访问
- 📦 **XML 转 JSON** - 自动将 Konachan XML 响应转换为 JSON 格式
- 🖼️ **图片代理** - 支持通过 API 代理访问 Konachan 图片资源
- 🐳 **Docker 支持** - 提供完整的 Docker 镜像和容器化部署方案
- 📝 **类型安全** - 利用 Rust 的类型系统确保代码安全性

---

## 📋 目录

- [特性](#-特性)
- [先决条件](#-先决条件)
- [快速开始](#-快速开始)
- [API 文档](#-api-文档)
- [项目结构](#-项目结构)
- [配置](#-配置)
- [Docker 部署](#-docker-部署)
- [开发](#-开发)
- [测试](#-测试)
- [技术栈](#-技术栈)
- [贡献](#-贡献)
- [许可证](#-许可证)
- [致谢](#-致谢)
- [联系方式](#-联系方式)

---

## 🔧 先决条件

- **Rust 1.88+** (支持 Rust 2024 edition)
- **[cargo-make](https://github.com/sagiegurari/cargo-make)** - Rust 任务运行器

```bash
# 安装 cargo-make
cargo install cargo-make
```

---

## 🚀 快速开始

### 克隆项目

```bash
git clone https://github.com/franciscowu/konachan-api.git
cd konachan-api
```

### 构建和运行

```bash
# 构建 release 版本
cargo make build

# 运行 release 版本
cargo make run

# 或者开发模式（支持热重载）
cargo make run-dev
```

### 验证安装

```bash
# 健康检查（如果实现了的话）
curl http://localhost:8000/

# 测试 API 端点
curl -H "x-api-key: konachan-api" \
  "http://localhost:8000/post?page=1&limit=5&tags=landscape"
```

---

## 📚 API 文档

### 认证

所有 API 请求都需要在请求头中包含 API 密钥：

```
x-api-key: konachan-api
```

### 1. 获取图片列表

**端点:** `GET /post`

**参数:**

| 参数 | 类型 | 必填 | 描述 |
|------|------|------|------|
| `page` | string | 是 | 页码 |
| `limit` | string | 是 | 每页返回结果数量 |
| `tags` | string | 是 | 搜索标签（空格分隔多个标签） |

**请求示例:**

```bash
curl -H "x-api-key: konachan-api" \
  "http://localhost:8000/post?page=1&limit=10&tags=landscape%20nature"
```

**响应格式:**

```json
{
  "data": {
    "count": 100,
    "images": [
      {
        "id": 12345,
        "url": "https://konachan.net/image/test.jpg",
        "width": 1920,
        "height": 1080,
        "preview": "https://konachan.net/preview/test.jpg",
        "preview_width": 150,
        "preview_height": 84,
        "sample": "https://konachan.net/sample/test.jpg",
        "sample_width": 1500,
        "sample_height": 844,
        "tags": "landscape nature sky",
        "security": true,
        "name": "test.jpg"
      }
    ]
  },
  "code": 0,
  "msg": null
}
```

**响应字段说明:**

| 字段 | 类型 | 描述 |
|------|------|------|
| `data.count` | integer | 符合条件的总图片数量 |
| `data.images` | array | 图片对象数组 |
| `images[].id` | integer | 图片 ID |
| `images[].url` | string | 原始图片 URL |
| `images[].width` | integer | 图片宽度 |
| `images[].height` | integer | 图片高度 |
| `images[].preview` | string | 预览图 URL |
| `images[].preview_width` | integer | 预览图宽度 |
| `images[].preview_height` | integer | 预览图高度 |
| `images[].sample` | string | 采样图 URL |
| `images[].sample_width` | integer | 采样图宽度 |
| `images[].sample_height` | integer | 采样图高度 |
| `images[].tags` | string | 图片标签（空格分隔） |
| `images[].security` | boolean | 是否安全内容（`true` 为安全，`false` 为问答/暴露） |
| `images[].name` | string | 解码后的图片文件名 |
| `code` | integer | 状态码（`0` 成功，`1` 错误） |
| `msg` | string | 错误信息（仅在 `code` 为 `1` 时返回） |

### 2. 代理图片访问

**端点:** `GET /image`

**参数:**

| 参数 | 类型 | 必填 | 描述 |
|------|------|------|------|
| `url` | string | 是 | 要代理的图片 URL |

**请求示例:**

```bash
# 代理访问 Konachan 图片
curl "http://localhost:8000/image?url=https://konachan.net/sample/xxx.jpg"
```

**响应:**

- 成功: 返回图片二进制数据，`Content-Type: image/jpeg`
- 失败: 返回 `400 Bad Request` 和错误信息

---

## 📁 项目结构

```
konachan-api/
├── src/
│   ├── main.rs      # 应用入口，定义路由和处理程序
│   ├── conf.rs      # 配置常量（API 端点、API 密钥）
│   ├── fairing.rs   # Rocket fairing（CORS 处理）
│   ├── guard.rs     # 请求守卫（API 密钥验证）
│   └── utils.rs     # 工具函数（XML 解析、HTTP 请求、响应结构体）
├── tests/
│   ├── api_tests.rs           # API 集成测试
│   └── integration_test.rs    # 端到端集成测试
├── Cargo.toml       # Rust 项目配置和依赖
├── Cargo.lock       # 依赖锁定文件
├── Makefile.toml    # cargo-make 任务配置
├── Rocket.toml      # Rocket 框架配置
├── rustfmt.toml     # Rust 代码格式化配置
├── Dockerfile       # Docker 镜像构建文件
├── .dockerignore    # Docker 构建忽略文件
└── README.md        # 项目文档
```

---

## ⚙️ 配置

### 环境变量

| 变量 | 描述 | 默认值 |
|------|------|--------|
| `CORS_ORIGIN` | 允许的 CORS 源，逗号分隔 | 空（禁用 CORS） |
| `ROCKET_PORT` | 服务器端口 | `8000` |
| `ROCKET_ADDRESS` | 服务器地址 | `0.0.0.0` |
| `ROCKET_LOG_LEVEL` | 日志级别 (`off`, `normal`, `debug`, `critical`) | `normal` |
| `API_KEY` | *(未来功能)* API 密钥 | `konachan-api` |

### Rocket 配置

项目使用 `Rocket.toml` 进行配置，也可以通过环境变量覆盖：

```bash
# 自定义端口
ROCKET_PORT=3000 cargo make run

# 自定义日志级别
ROCKET_LOG_LEVEL=debug cargo make run
```

---

## 🐳 Docker 部署

### 构建镜像

```bash
# 使用 cargo-make
cargo make docker-build

# 或直接使用 docker 命令
docker build -t konachan-api .
```

### 运行容器

```bash
# 使用 cargo-make
cargo make docker-run

# 或直接使用 docker 命令
docker run -d \
  --name konachan-api \
  -p 8000:8000 \
  -e CORS_ORIGIN=example.com,api.example.com \
  -e ROCKET_LOG_LEVEL=normal \
  konachan-api
```

### Docker 管理命令

```bash
# 停止容器
cargo make docker-stop

# 移除容器
cargo make docker-rm

# 清理容器和镜像
cargo make docker-clean
```

### Docker Compose (可选)

创建一个 `docker-compose.yml` 文件：

```yaml
version: '3.8'

services:
  konachan-api:
    build: .
    ports:
      - "8000:8000"
    environment:
      - CORS_ORIGIN=example.com
      - ROCKET_LOG_LEVEL=normal
      - ROCKET_PORT=8000
    restart: unless-stopped
```

运行：

```bash
docker-compose up -d
```

---

## 🛠️ 开发

### 可用的 cargo-make 任务

| 任务 | 描述 |
|------|------|
| `cargo make build` | Release 模式构建 |
| `cargo make dev` | Debug 模式构建 |
| `cargo make run` | 运行 release 版本 |
| `cargo make run-dev` | 运行 debug 版本（推荐开发使用） |
| `cargo make test` | 运行测试 |
| `cargo make check` | 检查代码错误 |
| `cargo make clippy` | 运行 clippy 代码检查 |
| `cargo make fmt` | 格式化代码 |
| `cargo make fmt-check` | 检查代码格式 |
| `cargo make lint` | 运行所有代码检查（fmt-check + clippy） |
| `cargo make ci` | 运行 CI 任务（check + test + lint） |
| `cargo make clean` | 清理构建产物 |

### 开发工作流程

```bash
# 1. 启动开发服务器
cargo make run-dev

# 2. 在另一个终端运行测试
cargo make test

# 3. 代码检查和格式化
cargo make lint

# 4. 提交前运行完整 CI 检查
cargo make ci
```

---

## 🧪 测试

```bash
# 运行所有测试
cargo make test

# 运行特定测试
cargo test test_parse_returns_correct_count

# 查看测试覆盖率（需要安装 cargo-tarpaulin）
cargo install cargo-tarpaulin
cargo tarpaulin --out html
```

### 测试用例

- **XML 解析测试** - 验证 XML 响应正确解析为 JSON
- **API 端点测试** - 测试 `/post` 和 `/image` 端点
- **集成测试** - 端到端测试完整请求流程

---

## 📦 技术栈

| 技术 | 版本 | 用途 |
|------|------|------|
| [Rust](https://www.rust-lang.org/) | 1.88+ | 编程语言 |
| [Rocket](https://rocket.rs/) | 0.5 | Web 框架 |
| [Reqwest](https://docs.rs/reqwest/) | 0.13 | HTTP 客户端 |
| [roxmltree](https://docs.rs/roxmltree/) | 0.21 | XML 解析 |
| [Serde](https://serde.rs/) | 1.0 | 序列化/反序列化 |

---

## 🤝 贡献

欢迎贡献！请随时提交 Pull Request 或开启 Issue。

1. Fork 本项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

### 代码规范

- 遵循 Rust 官方代码风格指南
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 确保所有测试通过

---

## 📄 许可证

本项目基于 MIT 许可证开源 - 查看 [LICENSE](LICENSE) 文件了解详情。

---

## 🙏 致谢

- [Konachan](https://konachan.net/) - 提供原始 API 和数据
- [Rocket](https://rocket.rs/) - 优秀的 Rust Web 框架
- [Rust 社区](https://www.rust-lang.org/community) - 提供出色的生态支持

---

## 📧 联系方式

- 作者: franciscowu
- GitHub: [@franciscowu](https://github.com/franciscowu)
- 项目链接: [https://github.com/franciscowu/konachan-api](https://github.com/franciscowu/konachan-api)

---

⭐ 如果这个项目对你有帮助，请给它一个星标！
