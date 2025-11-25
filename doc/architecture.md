# Architecture Guide

## Overview

The Intelligent Routing system is designed using a modular, extensible architecture that separates concerns and enables easy customization of load balancing strategies.

## System Design

```
┌─────────────────────────────────────────────────────────────────┐
│                        Client Layer                             │
│                    (Requests with cost/priority)                │
└─────────────────────────┬───────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│                         Router                                  │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │              LoadBalancingStrategy Trait                │    │
│  │  ┌──────────┐  ┌───────────────┐  ┌─────────────────┐   │    │
│  │  │Round     │  │Least          │  │Power of Two     │   │    │
│  │  │Robin     │  │Connections    │  │Choices (P2C)    │   │    │
│  │  └──────────┘  └───────────────┘  └─────────────────┘   │    │
│  └─────────────────────────────────────────────────────────┘    │
└─────────────────────────┬───────────────────────────────────────┘
                          │
                          ▼
┌───────────────────────────────────────────────────────────────┐
│                   Accelerator Pool                            │
│  ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐ ┌─────┐      │
│  │Acc 0│ │Acc 1│ │Acc 2│ │ ... │ │Acc N│ │     │ │     │      │
│  │GPU  │ │GPU  │ │TPU  │ │     │ │GPU  │ │     │ │     │      │
│  └─────┘ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘ └─────┘      │
└───────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Request (`request.rs`)

Represents a computational task to be routed to an accelerator.

```rust
pub struct Request {
    pub id: usize,       // Unique identifier
    pub cost: u32,       // Computational cost (affects load)
    pub priority: u8,    // Request priority (for future use)
}
```

**Key Properties:**
- **id**: Unique identifier for tracking and debugging
- **cost**: The amount of capacity the request will consume on an accelerator
- **priority**: Priority level for future scheduling optimizations

### 2. Accelerator (`accelerator.rs`)

Represents a hardware compute unit (GPU, TPU, or custom accelerator).

```rust
pub struct Accelerator {
    pub id: usize,           // Unique identifier
    pub capacity: u32,       // Maximum load capacity
    pub current_load: u32,   // Current utilization
    pub health_status: bool, // Availability flag
}
```

**Key Methods:**
- `new(id, capacity)`: Creates a new accelerator instance
- `is_available()`: Checks if accelerator can accept requests
- `add_load(load)`: Adds load, returns error if capacity exceeded
- `remove_load(load)`: Reduces current load (request completion)
- `update_load(load)`: Sets load to a specific value

**Capacity Model:**
```
┌───────────────────────────────────┐
│          Accelerator              │
│  ┌──────────────────────────────┐ │
│  │     Capacity: 100            │ │
│  │  ┌────────────────────────┐  │ │
│  │  │  Current Load: 45     █│  │ │
│  │  │  ░░░░░░░░░░░░░░░░░░░░░█│  │ │
│  │  │  ░░░░░░░░░░░░░░░░░░░░░█│  │ │
│  │  └────────────────────────┘  │ │
│  │     Available: 55            │ │
│  └──────────────────────────────┘ │
└───────────────────────────────────┘
```

### 3. Router (`router.rs`)

The central component that manages accelerators and routes requests.

```rust
pub trait LoadBalancingStrategy {
    fn select_accelerator(&self, accelerators: &[Accelerator], request: &Request) -> Option<usize>;
}

pub struct Router {
    pub accelerators: Vec<Accelerator>,
    strategy: Box<dyn LoadBalancingStrategy>,
}
```

**Key Methods:**
- `new(strategy)`: Creates router with specified strategy
- `add_accelerator(acc)`: Registers an accelerator
- `route_request(request)`: Routes request using the strategy

**Routing Flow:**
```
1. Request arrives at Router
2. Strategy.select_accelerator() chooses target
3. Router validates capacity on selected accelerator
4. Load is added to accelerator
5. Accelerator ID returned (or None if failed)
```

### 4. Strategies (`strategies/`)

Pluggable load balancing algorithms implementing the `LoadBalancingStrategy` trait.

## Data Flow

### Request Lifecycle

```
┌──────┐    ┌──────────┐    ┌─────────────┐    ┌─────────────┐
│Client│───▶│  Router  │───▶│  Strategy   │───▶│ Accelerator │
└──────┘    └──────────┘    └─────────────┘    └─────────────┘
    │            │                 │                  │
    │   1. Submit Request          │                  │
    │   ─────────▶                 │                  │
    │            │   2. Select     │                  │
    │            │   ─────────────▶│                  │
    │            │                 │   3. Query Load  │
    │            │                 │   ─────────────▶ │
    │            │                 │   ◀───────────── │
    │            │   4. Return idx │                  │
    │            │   ◀─────────────│                  │
    │            │                      5. Add Load  │
    │            │   ─────────────────────────────▶  │
    │   6. Return acc_id                             │
    │   ◀─────────                                   │
```

## Design Patterns

### Strategy Pattern

The system uses the Strategy pattern for load balancing algorithms:

```rust
// Define the strategy interface
pub trait LoadBalancingStrategy {
    fn select_accelerator(&self, accelerators: &[Accelerator], request: &Request) -> Option<usize>;
}

// Implement different strategies
impl LoadBalancingStrategy for RoundRobin { ... }
impl LoadBalancingStrategy for LeastConnections { ... }
impl LoadBalancingStrategy for PowerOfTwoChoices { ... }
```

**Benefits:**
- Easy to add new strategies without modifying core router
- Strategies can be swapped at runtime
- Clean separation of routing logic from infrastructure

### Thread Safety

The Round Robin strategy uses atomic operations for thread safety:

```rust
pub struct RoundRobin {
    current_index: AtomicUsize,
}

impl LoadBalancingStrategy for RoundRobin {
    fn select_accelerator(&self, accelerators: &[Accelerator], _request: &Request) -> Option<usize> {
        let idx = self.current_index.fetch_add(1, Ordering::Relaxed) % accelerators.len();
        // ...
    }
}
```

## Scalability Considerations

### Horizontal Scaling

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  Router 1   │     │  Router 2   │     │  Router N   │
│  (Region A) │     │  (Region B) │     │  (Region N) │
└──────┬──────┘     └──────┬──────┘     └──────┬──────┘
       │                   │                   │
       ▼                   ▼                   ▼
┌─────────────────────────────────────────────────────┐
│             Accelerator Pool (Shared State)         │
│  [Acc 0] [Acc 1] [Acc 2] ... [Acc 999] [Acc 1000]   │
└─────────────────────────────────────────────────────┘
```

### Memory Layout

For optimal cache performance with thousands of accelerators:

```rust
// Current: Array of Structures (AoS)
pub struct Accelerator {
    pub id: usize,
    pub capacity: u32,
    pub current_load: u32,
    pub health_status: bool,
}
accelerators: Vec<Accelerator>

// Alternative: Structure of Arrays (SoA) for better cache locality
pub struct AcceleratorPool {
    ids: Vec<usize>,
    capacities: Vec<u32>,
    current_loads: Vec<u32>,
    health_statuses: Vec<bool>,
}
```

## Extension Points

### Adding a New Strategy

1. Create new file in `src/strategies/`
2. Implement `LoadBalancingStrategy` trait
3. Export in `src/strategies/mod.rs`

```rust
// src/strategies/weighted_random.rs
pub struct WeightedRandom { ... }

impl LoadBalancingStrategy for WeightedRandom {
    fn select_accelerator(&self, accelerators: &[Accelerator], request: &Request) -> Option<usize> {
        // Custom logic here
    }
}
```

### Adding Accelerator Metrics

The `Accelerator` struct can be extended with additional metrics:

```rust
pub struct Accelerator {
    // Existing fields...
    
    // Potential extensions:
    pub latency_ms: f32,
    pub throughput: u32,
    pub error_rate: f32,
    pub temperature: f32,
    pub memory_usage: u32,
}
```

## Performance Characteristics

| Operation | Round Robin | Least Connections | P2C |
|-----------|-------------|-------------------|-----|
| Select | O(1) | O(n) | O(1) |
| Memory | O(1) | O(1) | O(1) |
| Thread-safe | Yes (atomic) | Read-only | Yes |
| Load Balance | Moderate | Optimal | Near-optimal |
