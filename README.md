# Reverse Proxy with Pingora

This project is a **Reverse Proxy** built in Rust 🦀 using the [Pingora](https://crates.io/crates/pingora) library. The proxy implements several features to handle modern HTTP workloads efficiently.

## Features

- **🔄 Load Balancing:** Round Robin algorithm for upstream selection.
- **⚡ Rate Limiting:** Enforces limits per app ID.
- **📡 Upstream Health Checks:** Periodic TCP health checks.
- **🔌 Custom Request Filters:** Filters and modifies incoming and outgoing requests.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- Cargo (comes with Rust)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/dexter-xD/reverse-proxy.git
   cd reverse-proxy
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Run the server:

   ```bash
   cargo run --release
   ```

### Testing the Proxy

Use the following `curl` command to test the proxy:

```bash
curl 127.0.0.1:6188 -svo /dev/null
```

## How It Works

### Main Features Explained

1. **Load Balancer**
   - Implements the Round Robin algorithm to distribute traffic across upstreams.
   - Configures periodic health checks to ensure upstream availability.

2. **Rate Limiter**
   - Limits requests based on the `appid` header.
   - Denies requests exceeding the rate limit with a `429 Too Many Requests` response.

3. **Custom Filters**
   - Adds/Modifies headers for requests sent to upstreams.
   - Validates incoming requests using custom logic.

### Key Components

- **`main.rs`**: Entry point of the application.
- **`load_balancer.rs`**: Handles upstream selection and request forwarding.
- **`rate_limiter.rs`**: Implements rate limiting.

### Configuration

- **Upstream Servers**:
  Configure upstreams in the `main.rs` file:

  ```rust
  let mut upstreams = LoadBalancer::try_from_iter(["1.1.1.1:443", "1.0.0.1:443", "127.0.0.1:343"]).unwrap();
  ```

- **Rate Limit**:
  Adjust the maximum requests per second in `rate_limiter.rs`:

  ```rust
  pub static MAX_REQ_PER_SEC: isize = 1;
  ```

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## Acknowledgments

- Built using [Pingora](https://crates.io/crates/pingora) for its lightweight and efficient proxying capabilities.
- Inspired by modern HTTP proxy requirements.

---
