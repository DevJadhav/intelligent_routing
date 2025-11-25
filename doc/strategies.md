# Load Balancing Strategies Guide

## Overview

This guide provides detailed information about each load balancing strategy implemented in the Intelligent Routing system. Understanding the trade-offs between strategies helps you choose the right one for your workload.

## Strategy Comparison

| Strategy | Complexity | Distribution | Best For |
|----------|------------|--------------|----------|
| Round Robin | O(1) | Fair | Homogeneous loads |
| Least Connections | O(n) | Optimal | Variable load sizes |
| Power of Two Choices | O(1) | Near-optimal | High throughput systems |

## Round Robin

### Description

Round Robin distributes requests sequentially across all available accelerators. Each accelerator receives a request in turn, cycling back to the first after reaching the last.

### Algorithm

```
1. Get current index (atomic counter)
2. Increment counter
3. Select accelerator at (index % num_accelerators)
4. If unavailable, linear probe for next available
5. Return selected accelerator index
```

### Implementation

```rust
pub struct RoundRobin {
    current_index: AtomicUsize,
}

impl LoadBalancingStrategy for RoundRobin {
    fn select_accelerator(&self, accelerators: &[Accelerator], _request: &Request) -> Option<usize> {
        if accelerators.is_empty() {
            return None;
        }
        
        let idx = self.current_index.fetch_add(1, Ordering::Relaxed) % accelerators.len();
        
        if accelerators[idx].is_available() {
            Some(idx)
        } else {
            // Linear probe for next available
            for i in 0..accelerators.len() {
                let next_idx = (idx + i) % accelerators.len();
                if accelerators[next_idx].is_available() {
                    return Some(next_idx);
                }
            }
            None
        }
    }
}
```

### Characteristics

- **Time Complexity**: O(1) average, O(n) worst case (when probing)
- **Space Complexity**: O(1)
- **Thread Safety**: Yes (uses AtomicUsize)

### Use Cases

✅ **Good For:**
- Uniform request costs
- Stateless workloads
- Simple deployment scenarios
- When all accelerators have equal capacity

❌ **Not Ideal For:**
- Variable request costs
- Heterogeneous accelerator capacities
- Latency-sensitive applications

### Visualization

```
Request 1 ──▶ [Acc 0]
Request 2 ──▶ [Acc 1]
Request 3 ──▶ [Acc 2]
Request 4 ──▶ [Acc 0]  (wraps around)
Request 5 ──▶ [Acc 1]
...
```

## Least Connections

### Description

Least Connections routes each request to the accelerator with the lowest current load. This ensures optimal load distribution but requires examining all accelerators.

### Algorithm

```
1. Filter available accelerators
2. Find accelerator with minimum current_load
3. Return its index
```

### Implementation

```rust
pub struct LeastConnections;

impl LoadBalancingStrategy for LeastConnections {
    fn select_accelerator(&self, accelerators: &[Accelerator], _request: &Request) -> Option<usize> {
        accelerators
            .iter()
            .enumerate()
            .filter(|(_, acc)| acc.is_available())
            .min_by_key(|(_, acc)| acc.current_load)
            .map(|(idx, _)| idx)
    }
}
```

### Characteristics

- **Time Complexity**: O(n) always
- **Space Complexity**: O(1)
- **Thread Safety**: Read-only (safe for concurrent reads)

### Use Cases

✅ **Good For:**
- Variable request costs
- Long-running tasks
- Small to medium accelerator pools
- When load distribution is critical

❌ **Not Ideal For:**
- Very large accelerator pools (1000+)
- High request rates (throughput bottleneck)
- Real-time systems with strict latency requirements

### Visualization

```
Accelerators:  [Acc 0: 50] [Acc 1: 30] [Acc 2: 45] [Acc 3: 20]
                                                        ▲
Request ─────────────────────────────────────────────────┘
                                              (lowest load)
```

### Load Distribution

```
Load over time with variable request costs:

Acc 0: ████████████████░░░░░░░░░░░░░░  (60%)
Acc 1: ████████████████░░░░░░░░░░░░░░  (60%)
Acc 2: ███████████████░░░░░░░░░░░░░░░  (58%)
Acc 3: ████████████████░░░░░░░░░░░░░░  (62%)

Very even distribution!
```

## Power of Two Choices (P2C)

### Description

Power of Two Choices is a probabilistic algorithm that achieves near-optimal load distribution with O(1) complexity. It randomly selects two accelerators and routes to the less loaded one.

### Algorithm

```
1. Randomly select two accelerators (idx1, idx2)
2. Compare their current loads
3. Return the index of the less loaded one
4. Handle edge cases (one or both unavailable)
```

### Implementation

```rust
pub struct PowerOfTwoChoices;

impl LoadBalancingStrategy for PowerOfTwoChoices {
    fn select_accelerator(&self, accelerators: &[Accelerator], _request: &Request) -> Option<usize> {
        if accelerators.is_empty() {
            return None;
        }
        
        let mut rng = rand::thread_rng();
        let len = accelerators.len();
        
        let idx1 = rng.gen_range(0..len);
        let idx2 = rng.gen_range(0..len);
        
        let acc1 = &accelerators[idx1];
        let acc2 = &accelerators[idx2];
        
        // Handle availability
        if !acc1.is_available() {
            return if acc2.is_available() { Some(idx2) } else { None };
        }
        if !acc2.is_available() {
            return Some(idx1);
        }
        
        // Pick less loaded
        if acc1.current_load <= acc2.current_load {
            Some(idx1)
        } else {
            Some(idx2)
        }
    }
}
```

### Characteristics

- **Time Complexity**: O(1)
- **Space Complexity**: O(1)
- **Thread Safety**: Yes (random selection is independent)

### Mathematical Foundation

The power of two choices achieves an exponential improvement in maximum load:

| Selection Method | Max Load (with high probability) |
|------------------|----------------------------------|
| Random (1 choice) | O(log n / log log n) |
| **2 Choices** | **O(log log n)** |
| d Choices | O(log log n / log d) |

With 1000 accelerators:
- Random: ~5-6x average load maximum
- P2C: ~2-3x average load maximum

### Use Cases

✅ **Good For:**
- Large accelerator pools (100+)
- High request rates
- Real-time systems
- Most production deployments

❌ **Not Ideal For:**
- Tiny pools (< 10 accelerators, use Round Robin)
- When absolutely optimal distribution is required

### Visualization

```
Step 1: Random Selection
Accelerators: [Acc 0] [Acc 1] [Acc 2] [Acc 3] [Acc 4] [Acc 5]
                 ▲                       ▲
              idx1=0                   idx2=3
              load=45                  load=30

Step 2: Compare and Choose
              load=45      >          load=30
                                        ▲
                                      Winner!

Request ──────────────────────────────▶ [Acc 3]
```

### Load Distribution Comparison

```
After 10,000 requests across 1,000 accelerators:

Random (1 choice):
Max: ████████████████████ (45)
Avg: █████████ (10)
Std: 6.2

Power of Two Choices:
Max: ███████████ (15)
Avg: █████████ (10)
Std: 1.7

Least Connections:
Max: █████████ (11)
Avg: █████████ (10)
Std: 0.5
```

## Choosing a Strategy

### Decision Tree

```
                    ┌─────────────────────────┐
                    │  How many accelerators? │
                    └───────────┬─────────────┘
                                │
            ┌───────────────────┼───────────────────┐
            │                   │                   │
          <10               10-100               >100
            │                   │                   │
            ▼                   ▼                   ▼
      ┌───────────┐      ┌───────────┐      ┌───────────┐
      │Round Robin│      │   P2C or  │      │    P2C    │
      └───────────┘      │   Least   │      └───────────┘
                         │  Conns    │
                         └───────────┘
```

### Performance vs Distribution Trade-off

```
        Perfect Distribution
               ▲
               │         ★ Least Connections
               │              
               │       ★ P2C
               │
               │
               │  ★ Round Robin
               │
               └──────────────────────────▶
                    Low Latency/High Throughput
```

### Workload Recommendations

| Workload Type | Recommended Strategy | Reason |
|---------------|---------------------|--------|
| Web requests (uniform) | Round Robin | Simple, fast |
| ML inference (variable) | P2C | Balances load efficiently |
| Batch processing | Least Connections | Optimal utilization |
| Real-time gaming | P2C | Low latency |
| Scientific computing | Least Connections | Maximize utilization |

## Advanced: Implementing Custom Strategies

### Template

```rust
use crate::accelerator::Accelerator;
use crate::request::Request;
use crate::router::LoadBalancingStrategy;

pub struct CustomStrategy {
    // Your state here
}

impl CustomStrategy {
    pub fn new() -> Self {
        Self {
            // Initialize state
        }
    }
}

impl LoadBalancingStrategy for CustomStrategy {
    fn select_accelerator(&self, accelerators: &[Accelerator], request: &Request) -> Option<usize> {
        // Your logic here
        // Return Some(index) for success, None for failure
    }
}
```

### Ideas for Custom Strategies

1. **Weighted Round Robin**: Consider accelerator capacities
2. **Locality-Aware**: Route to accelerators close to data
3. **Priority-Based**: Use request priority for scheduling
4. **Adaptive**: Switch strategies based on load patterns
5. **Consistent Hashing**: For stateful workloads
