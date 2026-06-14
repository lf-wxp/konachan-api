# Konachan API Proxy

[![Rust](https://img.shields.io/badge/Rust-1.88+-dea584?logo=rust)](https://www.rust-lang.org/)
[![Rocket](https://img.shields.io/badge/Rocket-0.5-red)](https://rocket.rs/)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

> **English** | [中文](README.md)

A high-performance Konachan API proxy service built with Rust and Rocket framework. Provides RESTful API endpoints for image search and proxy access.

## ✨ Features

- 🚀 **High Performance** - Built with Rust and Rocket framework for exceptional performance
- 🔒 **API Key Authentication** - Identity verification via `x-api-key` request header
- 🌐 **CORS Support** - Configurable Cross-Origin Resource Sharing with multi-domain support
- 📦 **XML to JSON** - Automatically converts Konachan XML responses to JSON format
- 🖼️ **Image Proxy** - Supports proxy access to Konachan image resources via API
- 🐳 **Docker Support** - Complete Docker image and containerization deployment solution
- 📝 **Type Safety** - Leverages Rust's type system to ensure code safety

## 📋 Table of Contents

- [Konachan API Proxy](#konachan-api-proxy)
  - [✨ Features](#-features)
  - [📋 Table of Contents](#-table-of-contents)
  - [🔧 Prerequisites](#-prerequisites)
  - [🚀 Quick Start](#-quick-start)
    - [Clone the Project](#clone-the-project)
    - [Build and Run](#build-and-run)
    - [Verify Installation](#verify-installation)
  - [📚 API Documentation](#-api-documentation)
    - [Authentication](#authentication)
    - [1. Get Image List](#1-get-image-list)
    - [2. Proxy Image Access](#2-proxy-image-access)
  - [📁 Project Structure](#-project-structure)
  - [⚙️ Configuration](#️-configuration)
    - [Environment Variables](#environment-variables)
    - [Rocket Configuration](#rocket-configuration)
  - [🐳 Docker Deployment](#-docker-deployment)
    - [Build Image](#build-image)
    - [Run Container](#run-container)
    - [Docker Management Commands](#docker-management-commands)
    - [Docker Compose (Optional)](#docker-compose-optional)
  - [🛠️ Development](#️-development)
    - [Available cargo-make Tasks](#available-cargo-make-tasks)
    - [Development Workflow](#development-workflow)
  - [🧪 Testing](#-testing)
    - [Test Cases](#test-cases)
  - [📦 Tech Stack](#-tech-stack)
  - [🤝 Contributing](#-contributing)
    - [Code Standards](#code-standards)
  - [📄 License](#-license)
  - [🙏 Acknowledgments](#-acknowledgments)
  - [📧 Contact](#-contact)

## 🔧 Prerequisites

- **Rust 1.88+** (supports Rust 2024 edition)
- **[cargo-make](https://github.com/sagiegurari/cargo-make)** - Rust task runner

```bash
# Install cargo-make
cargo install cargo-make
```

## 🚀 Quick Start

### Clone the Project

```bash
git clone https://github.com/franciscowu/konachan-api.git
cd konachan-api
```

### Build and Run

```bash
# Build release version
cargo make build

# Run release version
cargo make run

# Or development mode (with hot reload)
cargo make run-dev
```

### Verify Installation

```bash
# Health check (if implemented)
curl http://localhost:8000/

# Test API endpoint
curl -H "x-api-key: konachan-api" \
  "http://localhost:8000/post?page=1&limit=5&tags=landscape"
```

## 📚 API Documentation

### Authentication

All API requests require an API key in the request header:

```
x-api-key: konachan-api
```

### 1. Get Image List

**Endpoint:** `GET /post`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `page` | string | Yes | Page number |
| `limit` | string | Yes | Number of results per page |
| `tags` | string | Yes | Search tags (space-separated for multiple tags) |

**Request Example:**

```bash
curl -H "x-api-key: konachan-api" \
  "http://localhost:8000/post?page=1&limit=10&tags=landscape%20nature"
```

**Response Format:**

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

**Response Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `data.count` | integer | Total number of images matching the criteria |
| `data.images` | array | Array of image objects |
| `images[].id` | integer | Image ID |
| `images[].url` | string | Original image URL |
| `images[].width` | integer | Image width |
| `images[].height` | integer | Image height |
| `images[].preview` | string | Preview image URL |
| `images[].preview_width` | integer | Preview image width |
| `images[].preview_height` | integer | Preview image height |
| `images[].sample` | string | Sample image URL |
| `images[].sample_width` | integer | Sample image width |
| `images[].sample_height` | integer | Sample image height |
| `images[].tags` | string | Image tags (space-separated) |
| `images[].security` | boolean | Whether content is safe (`true` for safe, `false` for questionable/explicit) |
| `images[].name` | string | Decoded image filename |
| `code` | integer | Status code (`0` success, `1` error) |
| `msg` | string | Error message (only returned when `code` is `1`) |

### 2. Proxy Image Access

**Endpoint:** `GET /image`

**Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `url` | string | Yes | Image URL to proxy |

**Request Example:**

```bash
# Proxy access to Konachan image
curl "http://localhost:8000/image?url=https://konachan.net/sample/xxx.jpg"
```

**Response:**

- Success: Returns image binary data, `Content-Type: image/jpeg`
- Failure: Returns `400 Bad Request` with error message

## 📁 Project Structure

```
konachan-api/
├── src/
│   ├── main.rs      # Application entry point, route definitions and handlers
│   ├── conf.rs      # Configuration constants (API endpoint, API key)
│   ├── fairing.rs   # Rocket fairing (CORS handling)
│   ├── guard.rs     # Request guard (API key validation)
│   └── utils.rs     # Utility functions (XML parsing, HTTP requests, response structs)
├── tests/
│   ├── api_tests.rs           # API integration tests
│   └── integration_test.rs    # End-to-end integration tests
├── Cargo.toml       # Rust project configuration and dependencies
├── Cargo.lock       # Dependency lock file
├── Makefile.toml    # cargo-make task configuration
├── Rocket.toml      # Rocket framework configuration
├── rustfmt.toml     # Rust code formatting configuration
├── Dockerfile       # Docker image build file
├── .dockerignore    # Docker build ignore file
└── README.md        # Project documentation
```

## ⚙️ Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `CORS_ORIGIN` | Allowed CORS origins, comma-separated | Empty (CORS disabled) |
| `ROCKET_PORT` | Server port | `8000` |
| `ROCKET_ADDRESS` | Server address | `0.0.0.0` |
| `ROCKET_LOG_LEVEL` | Log level (`off`, `normal`, `debug`, `critical`) | `normal` |
| `API_KEY` | *(Future feature)* API key | `konachan-api` |

### Rocket Configuration

The project uses `Rocket.toml` for configuration, which can also be overridden via environment variables:

```bash
# Custom port
ROCKET_PORT=3000 cargo make run

# Custom log level
ROCKET_LOG_LEVEL=debug cargo make run
```

## 🐳 Docker Deployment

### Build Image

```bash
# Using cargo-make
cargo make docker-build

# Or directly with docker command
docker build -t konachan-api .
```

### Run Container

```bash
# Using cargo-make
cargo make docker-run

# Or directly with docker command
docker run -d \
  --name konachan-api \
  -p 8000:8000 \
  -e CORS_ORIGIN=example.com,api.example.com \
  -e ROCKET_LOG_LEVEL=normal \
  konachan-api
```

### Docker Management Commands

```bash
# Stop container
cargo make docker-stop

# Remove container
cargo make docker-rm

# Clean up container and image
cargo make docker-clean
```

### Docker Compose (Optional)

Create a `docker-compose.yml` file:

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

Run:

```bash
docker-compose up -d
```

## 🛠️ Development

### Available cargo-make Tasks

| Task | Description |
|------|-------------|
| `cargo make build` | Build in release mode |
| `cargo make dev` | Build in debug mode |
| `cargo make run` | Run release version |
| `cargo make run-dev` | Run debug version (recommended for development) |
| `cargo make test` | Run tests |
| `cargo make check` | Check code for errors |
| `cargo make clippy` | Run clippy code checks |
| `cargo make fmt` | Format code |
| `cargo make fmt-check` | Check code formatting |
| `cargo make lint` | Run all code checks (fmt-check + clippy) |
| `cargo make ci` | Run CI tasks (check + test + lint) |
| `cargo make clean` | Clean build artifacts |

### Development Workflow

```bash
# 1. Start development server
cargo make run-dev

# 2. Run tests in another terminal
cargo make test

# 3. Code checking and formatting
cargo make lint

# 4. Run full CI checks before committing
cargo make ci
```

## 🧪 Testing

```bash
# Run all tests
cargo make test

# Run specific test
cargo test test_parse_returns_correct_count

# View test coverage (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out html
```

### Test Cases

- **XML Parsing Tests** - Verify XML responses are correctly parsed to JSON
- **API Endpoint Tests** - Test `/post` and `/image` endpoints
- **Integration Tests** - End-to-end testing of complete request flow

## 📦 Tech Stack

| Technology | Version | Purpose |
|------------|---------|---------|
| [Rust](https://www.rust-lang.org/) | 1.88+ | Programming Language |
| [Rocket](https://rocket.rs/) | 0.5 | Web Framework |
| [Reqwest](https://docs.rs/reqwest/) | 0.13 | HTTP Client |
| [roxmltree](https://docs.rs/roxmltree/) | 0.21 | XML Parser |
| [Serde](https://serde.rs/) | 1.0 | Serialization/Deserialization |

## 🤝 Contributing

Contributions are welcome! Feel free to submit Pull Requests or open Issues.

1. Fork this project
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Code Standards

- Follow Rust official code style guidelines
- Use `cargo fmt` to format code
- Use `cargo clippy` to check code quality
- Ensure all tests pass

## 📄 License

This project is open sourced under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Konachan](https://konachan.net/) - Provides the original API and data
- [Rocket](https://rocket.rs/) - Excellent Rust web framework
- [Rust Community](https://www.rust-lang.org/community) - Provides outstanding ecosystem support

## 📧 Contact

- Author: franciscowu
- GitHub: [@franciscowu](https://github.com/franciscowu)
- Project Link: [https://github.com/franciscowu/konachan-api](https://github.com/franciscowu/konachan-api)

---

⭐ If this project helps you, please give it a star!
