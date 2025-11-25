# API Reference

## Overview

This document provides a complete API reference for the Intelligent Routing library. All public types, traits, and functions are documented here.

## Module Structure

```
intelligent_routing
├── accelerator     # Compute unit representation
├── request         # Request model
├── router          # Core routing logic
└── strategies      # Load balancing algorithms
    ├── round_robin
    ├── least_connections
    └── p2c
```

---

## Module: `accelerator`

### Struct: `Accelerator`

Represents a hardware compute accelerator (GPU, TPU, etc.).

```rust
#[derive(Debug, Clone)]
pub struct Accelerator {
    pub id: usize,
    pub capacity: u32,
    pub current_load: u32,
    pub health_status: bool,
}
```

#### Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | `usize` | Unique identifier for the accelerator |
| `capacity` | `u32` | Maximum load capacity |
| `current_load` | `u32` | Current utilization level |
| `health_status` | `bool` | Whether accelerator is healthy/available |

#### Methods

##### `new`

Creates a new accelerator instance.

```rust
pub fn new(id: usize, capacity: u32) -> Self
```

**Parameters:**
- `id`: Unique identifier
- `capacity`: Maximum load capacity

**Returns:** New `Accelerator` with zero load and healthy status

**Example:**
```rust
let acc = Accelerator::new(0, 100);
assert_eq!(acc.id, 0);
assert_eq!(acc.capacity, 100);
assert_eq!(acc.current_load, 0);
assert!(acc.health_status);
```

---

##### `is_available`

Checks if the accelerator can accept new requests.

```rust
pub fn is_available(&self) -> bool
```

**Returns:** `true` if healthy AND has available capacity

**Example:**
```rust
let acc = Accelerator::new(0, 100);
assert!(acc.is_available()); // true - healthy and has capacity
```

---

##### `add_load`

Adds load to the accelerator.

```rust
pub fn add_load(&mut self, load: u32) -> Result<(), String>
```

**Parameters:**
- `load`: Amount of load to add

**Returns:**
- `Ok(())` if load was added successfully
- `Err(String)` if adding load would exceed capacity

**Example:**
```rust
let mut acc = Accelerator::new(0, 100);
assert!(acc.add_load(50).is_ok());
assert_eq!(acc.current_load, 50);
assert!(acc.add_load(60).is_err()); // Would exceed capacity
```

---

##### `remove_load`

Removes load from the accelerator (e.g., when request completes).

```rust
pub fn remove_load(&mut self, load: u32)
```

**Parameters:**
- `load`: Amount of load to remove (clamped to 0)

**Example:**
```rust
let mut acc = Accelerator::new(0, 100);
acc.add_load(50).unwrap();
acc.remove_load(30);
assert_eq!(acc.current_load, 20);
acc.remove_load(100); // Clamps to 0
assert_eq!(acc.current_load, 0);
```

---

##### `update_load`

Sets the current load to a specific value.

```rust
pub fn update_load(&mut self, load: u32)
```

**Parameters:**
- `load`: New load value

**Example:**
```rust
let mut acc = Accelerator::new(0, 100);
acc.update_load(75);
assert_eq!(acc.current_load, 75);
```

---

## Module: `request`

### Struct: `Request`

Represents a computational request to be routed.

```rust
#[derive(Debug, Clone)]
pub struct Request {
    pub id: usize,
    pub cost: u32,
    pub priority: u8,
}
```

#### Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | `usize` | Unique request identifier |
| `cost` | `u32` | Computational cost (load consumed) |
| `priority` | `u8` | Request priority (0-255) |

#### Methods

##### `new`

Creates a new request.

```rust
pub fn new(id: usize, cost: u32, priority: u8) -> Self
```

**Parameters:**
- `id`: Unique identifier
- `cost`: Computational cost
- `priority`: Priority level

**Example:**
```rust
let req = Request::new(1, 10, 1);
assert_eq!(req.id, 1);
assert_eq!(req.cost, 10);
assert_eq!(req.priority, 1);
```

---

## Module: `router`

### Trait: `LoadBalancingStrategy`

Interface for load balancing algorithms.

```rust
pub trait LoadBalancingStrategy {
    fn select_accelerator(&self, accelerators: &[Accelerator], request: &Request) -> Option<usize>;
}
```

#### Methods

##### `select_accelerator`

Selects an accelerator for the given request.

```rust
fn select_accelerator(&self, accelerators: &[Accelerator], request: &Request) -> Option<usize>
```

**Parameters:**
- `accelerators`: Slice of available accelerators
- `request`: The request to route

**Returns:**
- `Some(index)`: Index of selected accelerator
- `None`: No suitable accelerator found

---

### Struct: `Router`

Central routing component that manages accelerators and strategies.

```rust
pub struct Router {
    pub accelerators: Vec<Accelerator>,
    strategy: Box<dyn LoadBalancingStrategy>,
}
```

#### Fields

| Field | Type | Description |
|-------|------|-------------|
| `accelerators` | `Vec<Accelerator>` | Pool of managed accelerators |
| `strategy` | `Box<dyn LoadBalancingStrategy>` | Load balancing strategy |

#### Methods

##### `new`

Creates a new router with the specified strategy.

```rust
pub fn new(strategy: Box<dyn LoadBalancingStrategy>) -> Self
```

**Parameters:**
- `strategy`: Boxed load balancing strategy

**Example:**
```rust
let strategy = Box::new(PowerOfTwoChoices::new());
let router = Router::new(strategy);
```

---

##### `add_accelerator`

Adds an accelerator to the pool.

```rust
pub fn add_accelerator(&mut self, accelerator: Accelerator)
```

**Parameters:**
- `accelerator`: Accelerator to add

**Example:**
```rust
let mut router = Router::new(Box::new(PowerOfTwoChoices::new()));
router.add_accelerator(Accelerator::new(0, 100));
router.add_accelerator(Accelerator::new(1, 100));
```

---

##### `route_request`

Routes a request to an appropriate accelerator.

```rust
pub fn route_request(&mut self, request: &Request) -> Option<usize>
```

**Parameters:**
- `request`: Request to route

**Returns:**
- `Some(acc_id)`: ID of accelerator that received the request
- `None`: No accelerator available

**Example:**
```rust
let mut router = Router::new(Box::new(PowerOfTwoChoices::new()));
router.add_accelerator(Accelerator::new(0, 100));

let req = Request::new(1, 10, 1);
match router.route_request(&req) {
    Some(id) => println!("Routed to accelerator {}", id),
    None => println!("No available accelerator"),
}
```

---

## Module: `strategies`

### Struct: `RoundRobin`

Sequential distribution strategy.

```rust
pub struct RoundRobin {
    current_index: AtomicUsize,
}
```

#### Methods

##### `new`

Creates a new Round Robin strategy.

```rust
pub fn new() -> Self
```

**Example:**
```rust
let strategy = Box::new(RoundRobin::new());
let router = Router::new(strategy);
```

---

### Struct: `LeastConnections`

Minimum load distribution strategy.

```rust
pub struct LeastConnections;
```

#### Methods

##### `new`

Creates a new Least Connections strategy.

```rust
pub fn new() -> Self
```

**Example:**
```rust
let strategy = Box::new(LeastConnections::new());
let router = Router::new(strategy);
```

---

### Struct: `PowerOfTwoChoices`

Probabilistic load balancing strategy.

```rust
pub struct PowerOfTwoChoices;
```

#### Methods

##### `new`

Creates a new Power of Two Choices strategy.

```rust
pub fn new() -> Self
```

**Example:**
```rust
let strategy = Box::new(PowerOfTwoChoices::new());
let router = Router::new(strategy);
```

---

## Complete Usage Example

```rust
use intelligent_routing::accelerator::Accelerator;
use intelligent_routing::request::Request;
use intelligent_routing::router::Router;
use intelligent_routing::strategies::p2c::PowerOfTwoChoices;

fn main() {
    // Create router with P2C strategy
    let mut router = Router::new(Box::new(PowerOfTwoChoices::new()));
    
    // Add 1000 accelerators with capacity 100 each
    for i in 0..1000 {
        router.add_accelerator(Accelerator::new(i, 100));
    }
    
    // Route 10000 requests
    let mut success = 0;
    let mut failed = 0;
    
    for i in 0..10000 {
        let request = Request::new(i, 5, 1); // cost=5, priority=1
        
        match router.route_request(&request) {
            Some(_) => success += 1,
            None => failed += 1,
        }
        
        // Simulate load decay every 100 requests
        if i % 100 == 0 {
            for acc in &mut router.accelerators {
                acc.remove_load(3);
            }
        }
    }
    
    println!("Success: {}, Failed: {}", success, failed);
    
    // Calculate statistics
    let loads: Vec<u32> = router.accelerators.iter()
        .map(|a| a.current_load)
        .collect();
    
    let avg: f64 = loads.iter().sum::<u32>() as f64 / loads.len() as f64;
    println!("Average load: {:.2}", avg);
}
```

---

## Error Handling

### Capacity Error

When adding load exceeds capacity:

```rust
let mut acc = Accelerator::new(0, 10);
acc.add_load(5).unwrap();

match acc.add_load(10) {
    Ok(_) => println!("Load added"),
    Err(e) => println!("Error: {}", e), // "Capacity exceeded"
}
```

### No Available Accelerator

When all accelerators are full or unhealthy:

```rust
let req = Request::new(1, 10, 1);
match router.route_request(&req) {
    Some(id) => println!("Routed to {}", id),
    None => {
        // Handle failure: retry, queue, or reject
        eprintln!("No accelerator available");
    }
}
```

---

## Thread Safety

| Component | Thread Safe | Notes |
|-----------|-------------|-------|
| `Accelerator` | No | Use `Mutex<Accelerator>` for concurrent access |
| `Request` | Yes | Immutable after creation |
| `Router` | No | Use `Mutex<Router>` or implement sharding |
| `RoundRobin` | Yes | Uses `AtomicUsize` |
| `LeastConnections` | Yes* | Read-only access to accelerators |
| `PowerOfTwoChoices` | Yes | Thread-local RNG |

*Note: The strategy itself is thread-safe, but the router's mutable operations are not.

---

## Performance Characteristics

| Operation | Time Complexity | Space Complexity |
|-----------|-----------------|------------------|
| `Accelerator::new` | O(1) | O(1) |
| `Accelerator::add_load` | O(1) | O(1) |
| `Router::add_accelerator` | O(1) amortized | O(n) |
| `RoundRobin::select` | O(1) avg, O(n) worst | O(1) |
| `LeastConnections::select` | O(n) | O(1) |
| `PowerOfTwoChoices::select` | O(1) | O(1) |
