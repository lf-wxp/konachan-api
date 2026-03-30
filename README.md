# konachan-api

Konachan data API in Rust with Rocket framework.

## Prerequisites

- Rust 1.88+ (with Rust 2024 edition support)
- [cargo-make](https://github.com/sagiegurari/cargo-make) - Rust task runner

```bash
cargo install cargo-make
```

## Installation

### Build

```bash
cargo make build
```

### Run

```bash
cargo make run
```

### Development

```bash
cargo make run-dev
```

### Available Tasks

| Task | Description |
|------|-------------|
| `cargo make build` | Build in release mode |
| `cargo make dev` | Build in debug mode |
| `cargo make run` | Run in release mode |
| `cargo make run-dev` | Run in debug mode |
| `cargo make test` | Run tests |
| `cargo make check` | Check for errors |
| `cargo make clippy` | Run clippy linter |
| `cargo make fmt` | Format code |
| `cargo make fmt-check` | Check code formatting |
| `cargo make lint` | Run all linting checks (fmt-check + clippy) |
| `cargo make ci` | Run CI tasks (check, test, lint) |
| `cargo make clean` | Clean build artifacts |
| `cargo make docker-build` | Build Docker image |
| `cargo make docker-run` | Run Docker container |
| `cargo make docker-stop` | Stop and remove Docker container |
| `cargo make docker-rm` | Remove Docker container |
| `cargo make docker-clean` | Clean Docker container and image |

## Docker

### Build Image

```bash
cargo make docker-build
# or
docker build -t konachan-api .
```

### Run Container

```bash
cargo make docker-run
# or
docker run -d \
  --name konachan-api \
  -p 8000:8000 \
  -e CORS_ORIGIN=domain.com,domain2.com \
  konachan-api
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `CORS_ORIGIN` | Comma-separated list of allowed CORS origins | `` |
| `ROCKET_PORT` | Server port | `8000` |
| `ROCKET_ADDRESS` | Server address | `0.0.0.0` |
| `ROCKET_LOG_LEVEL` | Log level (`off`, `normal`, `debug`, `critical`) | `normal` |

## CLI

```bash
CORS_ORIGIN=domain.com,domain2.com cargo make run
```

## API Endpoints

### GET /post

Get posts from Konachan.

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `page` | string | Page number |
| `limit` | string | Number of results per page |
| `tags` | string | Search tags |

**Headers:**

| Header | Required | Description |
|--------|----------|-------------|
| `x-api-key` | Yes | API key for authentication |

**Example:**

```bash
curl -H "x-api-key: konachan-api" "http://localhost:8000/post?page=1&limit=10&tags=landscape"
```

### GET /image

Proxy image from Konachan.

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `url` | string | Image URL to proxy |

**Example:**

```bash
curl "http://localhost:8000/image?url=https://konachan.net/sample/xxx.jpg"
```
