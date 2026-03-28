# Description

## konachan-api
Konachan data API in Rust with Rocket framework

# Prerequisites

- Rust 1.85+ (with Rust 2024 edition support)
- [cargo-make](https://github.com/sagiegurari/cargo-make) - Rust task runner

```bash
cargo install cargo-make
```

# Installation

## Using cargo-make

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
| `cargo make lint` | Run all linting checks |
| `cargo make ci` | Run CI tasks (check, test, lint) |
| `cargo make clean` | Clean build artifacts |
| `cargo make docker-build` | Build Docker image |
| `cargo make docker-run` | Run Docker container |
| `cargo make docker-stop` | Stop Docker container |
| `cargo make docker-clean` | Clean Docker resources |

## Using Docker

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

### Using Docker Compose
```bash
docker-compose up -d
```

# Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `CORS_ORIGIN` | Comma-separated list of allowed CORS origins | `` |
| `ROCKET_PORT` | Server port | `8000` |
| `ROCKET_ADDRESS` | Server address | `0.0.0.0` |

# CLI
```bash
CORS_ORIGIN=domain.com,domain2.com konachan-api
```

# API Endpoints

## GET /post
Get posts from Konachan.

**Parameters:**
- `page` - Page number
- `limit` - Number of results per page
- `tags` - Search tags

**Headers:**
- `x-api-key` - API key (required)

## GET /image
Proxy image from Konachan.

**Parameters:**
- `url` - Image URL to proxy
