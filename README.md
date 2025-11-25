# Intelligent Routing

[![PyPI version](https://img.shields.io/pypi/v/intelligent-routing.svg)](https://pypi.org/project/intelligent-routing/)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/intelligent-routing.svg)](https://pypi.org/project/intelligent-routing/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance Rust library implementing intelligent routing algorithms for optimizing request distribution across thousands of accelerators (GPUs, TPUs, or other compute units).

## Overview

This project provides a flexible and efficient load balancing system designed to distribute computational workloads across large clusters of hardware accelerators. It implements multiple routing strategies that can be easily swapped based on workload characteristics and performance requirements.

## Features

- **Multiple Load Balancing Strategies**
  - Round Robin with failover
  - Least Connections
  - Power of Two Choices (P2C)

- **Scalable Architecture**
  - Designed to handle thousands of accelerators
  - Efficient O(1) or O(log n) routing decisions
  - Thread-safe implementations using atomic operations

- **Flexible Configuration**
  - Pluggable strategy pattern for easy algorithm switching
  - Configurable accelerator capacity
  - Request priority support

- **Simulation & Benchmarking**
  - Built-in simulation framework
  - Load distribution statistics
  - Performance metrics (throughput, latency)

## Installation

### Python Package (Recommended)

Install directly from PyPI:

```bash
pip install intelligent-routing
```

### From Source (Development)

```bash
# Clone the repository
git clone https://github.com/yourusername/intelligent_routing.git
cd intelligent_routing

# Create virtual environment (optional but recommended)
python -m venv .venv
source .venv/bin/activate  # On Windows: .venv\Scripts\activate

# Install maturin and build
pip install maturin
maturin develop
```

### Rust Library

Add to your `Cargo.toml`:

```toml
[dependencies]
intelligent_routing = { git = "https://github.com/yourusername/intelligent_routing.git" }
```

Or clone and build locally:

```bash
git clone https://github.com/yourusername/intelligent_routing.git
cd intelligent_routing
cargo build --release
```

## Quick Start

### Python Usage

```python
import intelligent_routing

# Create accelerators
acc1 = intelligent_routing.Accelerator(1, 100)
acc2 = intelligent_routing.Accelerator(2, 100)

# Initialize router with a strategy ("round_robin", "least_connections", "p2c")
router = intelligent_routing.Router("p2c")
router.add_accelerator(acc1)
router.add_accelerator(acc2)

# Route a request
req = intelligent_routing.Request(1, 10, 1)
acc_id = router.route_request(req)

if acc_id is not None:
    print(f"Routed to accelerator {acc_id}")
```

### Rust Usage

```rust
use intelligent_routing::accelerator::Accelerator;
use intelligent_routing::request::Request;
use intelligent_routing::router::Router;
use intelligent_routing::strategies::p2c::PowerOfTwoChoices;

fn main() {
    // Create accelerators
    let mut accelerators = Vec::new();
    for i in 0..1000 {
        accelerators.push(Accelerator::new(i, 100)); // 100 capacity units each
    }

    // Create router with Power of Two Choices strategy
    let strategy = Box::new(PowerOfTwoChoices::new());
    let mut router = Router::new(strategy);
    
    for acc in accelerators {
        router.add_accelerator(acc);
    }

    // Route a request
    let request = Request::new(1, 5, 1); // id=1, cost=5, priority=1
    match router.route_request(&request) {
        Some(acc_id) => println!("Request routed to accelerator {}", acc_id),
        None => println!("No available accelerator"),
    }
}
```

### Run Rust Simulation

```bash
cargo run --release
```

### Choosing a Strategy

```rust
// Round Robin - Simple sequential distribution
let strategy = Box::new(RoundRobin::new());

// Least Connections - Route to least loaded accelerator
let strategy = Box::new(LeastConnections::new());

// Power of Two Choices - Optimal balance of performance and distribution
let strategy = Box::new(PowerOfTwoChoices::new());
```

## Architecture

```
intelligent_routing/
├── src/
│   ├── lib.rs              # Library exports
│   ├── main.rs             # Simulation entry point
│   ├── accelerator.rs      # Accelerator (compute unit) model
│   ├── request.rs          # Request model
│   ├── router.rs           # Core router and strategy trait
│   ├── bindings.rs         # Python bindings (PyO3)
│   └── strategies/
│       ├── mod.rs          # Strategy module exports
│       ├── round_robin.rs  # Round Robin implementation
│       ├── least_connections.rs  # Least Connections implementation
│       └── p2c.rs          # Power of Two Choices implementation
├── tests/
│   ├── test_routing.py     # Python unit tests
│   └── scale_test.py       # Performance benchmarks
├── doc/
│   ├── architecture.md     # System architecture documentation
│   ├── strategies.md       # Load balancing strategies guide
│   └── api.md              # API reference
├── Cargo.toml              # Rust dependencies
└── pyproject.toml          # Python package configuration
```

## Performance

Benchmark results with **10,000 accelerators** and **100,000 requests** (Python Wrapper):

| Strategy | Throughput | Success Rate |
|----------|------------|--------------|
| Power of Two Choices | ~550,000 req/s | 100% |
| Round Robin | ~800,000 req/s | 100% |

*Note: Results may vary based on hardware and configuration. P2C provides better load balancing at the cost of slightly lower throughput compared to Round Robin.*

## Load Balancing Strategies

### Round Robin
Distributes requests sequentially across all accelerators. Simple and fast, but doesn't consider current load.

### Least Connections
Always routes to the accelerator with the lowest current load. Optimal distribution but O(n) complexity per request.

### Power of Two Choices (P2C)
Randomly samples two accelerators and picks the less loaded one. Provides near-optimal load distribution with O(1) complexity. This is the recommended strategy for most use cases.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/intelligent_routing.git
cd intelligent_routing

# Create virtual environment
python -m venv .venv
source .venv/bin/activate  # On Windows: .venv\Scripts\activate

# Install development dependencies
pip install maturin pytest

# Build and install in development mode
maturin develop

# Run tests
pytest tests/ -v

# Run scale benchmarks
python tests/scale_test.py
```

### Building Wheels

```bash
# Build wheel for current platform
maturin build --release

# Build wheels for multiple platforms (requires appropriate setup)
maturin build --release --target x86_64-unknown-linux-gnu
```

### Publishing to PyPI

```bash
# Build release wheel
maturin build --release

# Upload to PyPI (requires PyPI credentials)
maturin publish

# Or upload to TestPyPI first
maturin publish --repository testpypi
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Documentation

For detailed documentation, see the `doc/` folder:

- [Architecture Guide](doc/architecture.md)
- [Strategies Guide](doc/strategies.md)
- [API Reference](doc/api.md)
