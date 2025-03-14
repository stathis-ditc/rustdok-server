# RustDok Server

RustDok is an S3-compatible object storage server built with Rust. It provides a RESTful API for managing buckets and objects in an S3-compatible storage backend.

## Disclaimer

**NO WARRANTY**: RustDok is provided "as is" without warranty of any kind, express or implied. The authors and contributors make no representations or warranties of any kind concerning the software, express or implied, including, without limitation, warranties of merchantability, fitness for a particular purpose, or non-infringement. In no event shall the authors or copyright holders be liable for any claim, damages, or other liability, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the software or the use or other dealings in the software.

Use at your own risk.

## Features

- **S3 Compatibility**: Works with S3-compatible storage backends (primarily tested with Rook Ceph)
- **Bucket Management**: Create, list, and delete buckets
- **Object Operations**: Upload, download, view, list, and delete objects
- **Folder Support**: Create and navigate folder-like structures
- **Modern API**: RESTful API with JSON responses
- **CORS Support**: Built-in CORS configuration for web applications
- **Health Checks**: Built-in health check endpoints for container orchestration
- **Docker Support**: Optimized Docker image with multi-architecture support (AMD64/ARM64)

## Prerequisites

- Rust (latest stable version)
- An S3-compatible storage service (primarily tested with Rook Ceph)
- Access and secret keys for your S3 service

> **Note**: While RustDok is designed to work with any S3-compatible storage service, it has been primarily tested with Rook Ceph. Compatibility with other S3 services (AWS S3, MinIO, etc.) is not guaranteed and may require additional configuration or code changes.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/RustDok.git
   cd RustDok/server
   ```

2. Create a `.env` file in the server directory with the following variables:
   ```
   S3_ENDPOINT_URL=https://your-s3-endpoint.com
   S3_REGION=eu-central-1  # Optional for some services like Ceph
   S3_ACCESS_KEY=your-access-key
   S3_SECRET_KEY=your-secret-key
   RUSTDOK_WEBUI_URL=http://rustdok-webui-url:port-number  # Optional if rustdok webui is used. URL for CORS configuration
   RUST_LOG=info  # Optional, sets the logging level (trace, debug, info, warn, error)
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

## Running the Server

### Using Cargo

Start the server with:

```bash
cargo run --release
```

The server will start on `0.0.0.0:8080` by default.

### Using Docker

Build the Docker image:

```bash
docker build -t rustdok-server:latest .
```

Run the container:

```bash
docker run -p 8080:8080 \
  -e S3_ENDPOINT_URL=https://your-s3-endpoint.com \
  -e S3_REGION=eu-central-1 \
  -e S3_ACCESS_KEY=your-access-key \
  -e S3_SECRET_KEY=your-secret-key \
  -e RUSTDOK_WEBUI_URL=http://localhost:3000 \
  rustdok-server:latest
```

Or use an environment file:

```bash
docker run -p 8080:8080 --env-file .env rustdok-server:latest
```

## API Endpoints

### Health Checks

- **Liveness Check**
  - `GET /healthz`
  - Returns a 200 OK response if the server is running
  - Used by container orchestration systems to determine if the application is alive

- **Readiness Check**
  - `GET /readyz`
  - Returns a 200 OK response if the server is ready to accept requests
  - Used by container orchestration systems to determine if traffic should be routed to the container

### Bucket Operations

- **List Buckets**
  - `GET /api/v1/buckets`
  - Returns a list of all buckets

- **Create Bucket**
  - `POST /api/v1/buckets`
  - Request body: `{ "name": "bucket-name" }`
  - Creates a new bucket

- **Delete Bucket**
  - `DELETE /api/v1/bucket/{name}`
  - Deletes a bucket and all its contents

### Object Operations

- **List Objects in Bucket**
  - `GET /api/v1/bucket/{bucket}/objects?prefix=optional/prefix`
  - Lists objects in a bucket, optionally filtered by prefix

- **Upload Object**
  - `POST /api/v1/bucket/{bucket}/object?prefix=optional/prefix&replace=false`
  - Multipart form data with file
  - Uploads a file to the specified bucket

- **Download Object**
  - `GET /api/v1/bucket/{bucket}/download/{key}`
  - Downloads an object from the bucket

- **View Object**
  - `GET /api/v1/bucket/{bucket}/view/{key}`
  - Views an object in the browser with appropriate content type

- **Delete Object**
  - `DELETE /api/v1/bucket/{bucket}/object/{key}`
  - Deletes an object from the bucket

- **Create Folder**
  - `POST /api/v1/bucket/{bucket}/folders`
  - Request body: `{ "name": "folder/path/" }`
  - Creates a new folder in the bucket

- **Check if Object Exists**
  - `GET /api/v1/bucket/{bucket}/exists?filename=object-key`
  - Checks if an object exists in the bucket

## Development

### Project Structure

- `src/main.rs` - Entry point and server configuration
- `src/api/` - API endpoints and route configuration
  - `src/api/config.rs` - API configuration
  - `src/api/health.rs` - Health check endpoints
  - `src/api/v1/` - API v1 endpoints
    - `src/api/v1/buckets.rs` - Bucket operations
    - `src/api/v1/objects.rs` - Object operations
- `src/rdlib/` - Core library functionality
  - `src/rdlib/s3/` - S3 service implementation
    - `service.rs` - S3 client configuration
    - `bucket/` - Bucket operations
    - `object/` - Object operations
    - `error.rs` - Error handling
    - `types.rs` - Data structures
- `src/models/` - Data models for requests and responses

### Logging

RustDok uses `env_logger` for logging. You can control the log level by setting the `RUST_LOG` environment variable:

- `RUST_LOG=trace` - Most verbose, shows all logs including trace-level details
- `RUST_LOG=debug` - Shows debug information useful for development
- `RUST_LOG=info` - Default level, shows informational messages about normal operation
- `RUST_LOG=warn` - Shows only warnings and errors
- `RUST_LOG=error` - Shows only errors

You can set the log level in your `.env` file or when running the server:

```bash
RUST_LOG=debug cargo run
```

You can also set different log levels for different modules:

```bash
RUST_LOG=rustdok_server=debug,actix_web=info cargo run
```

### Documentation
 
API Documentation has been already generated and available under `docs`. To visualise in ui format
open `docs/rustdok_server/index.html` in local browser

### OpenAPI Documentation

The project includes OpenAPI v3 documentation that can be viewed using ReDoc:

1. Open the `docs/openapiv3/redoc.html` file in your browser to view the API documentation
2. Alternatively, you can use any OpenAPI viewer with the `docs/openapiv3/openapi.yaml` file

To generate and view the API documentation for local development run:

```bash
cargo doc --no-deps --open
```

## Testing

Run the test suite with:

```bash
cargo test
```

For testing with the mock S3 service:

```bash
cargo test --features testing
```

## Docker

### Multi-Architecture Support

The project includes GitHub Actions workflows for building multi-architecture Docker images (AMD64 and ARM64). The workflow automatically builds and pushes images to GitHub Container Registry.

To use the pre-built images:

```bash
docker pull ghcr.io/devs-in-the-cloud/rustdok-server:latest
```

## Dependencies

RustDok is built with the following major dependencies:

- **Actix Web**: Fast, pragmatic, and flexible web framework
- **AWS SDK for Rust**: Official AWS SDK for interacting with S3-compatible services
- **Tokio**: Asynchronous runtime for Rust
- **Serde**: Serialization/deserialization framework

## License

[MIT License](LICENSE)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
