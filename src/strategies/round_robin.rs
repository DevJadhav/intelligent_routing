use crate::accelerator::Accelerator;
use crate::request::Request;
use crate::router::LoadBalancingStrategy;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct RoundRobin {
    current_index: AtomicUsize,
}

impl RoundRobin {
    pub fn new() -> Self {
        Self {
            current_index: AtomicUsize::new(0),
        }
    }
}

impl LoadBalancingStrategy for RoundRobin {
    fn select_accelerator(&self, accelerators: &[Accelerator], _request: &Request) -> Option<usize> {
        if accelerators.is_empty() {
            return None;
        }
        
        // Simple round robin
        let idx = self.current_index.fetch_add(1, Ordering::Relaxed) % accelerators.len();
        
        // Check availability, if not available, linear probe (simplified for now)
        // In a real system we might want a more robust failover
        if accelerators[idx].is_available() {
            Some(idx)
        } else {
            // Try to find next available
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
