use crate::accelerator::Accelerator;
use crate::request::Request;
use crate::router::LoadBalancingStrategy;
use rand::Rng;

pub struct PowerOfTwoChoices;

impl PowerOfTwoChoices {
    pub fn new() -> Self {
        Self
    }
}

impl LoadBalancingStrategy for PowerOfTwoChoices {
    fn select_accelerator(&self, accelerators: &[Accelerator], _request: &Request) -> Option<usize> {
        if accelerators.is_empty() {
            return None;
        }
        
        let mut rng = rand::thread_rng();
        let len = accelerators.len();
        
        // Pick two random indices
        let idx1 = rng.random_range(0..len);
        let idx2 = rng.random_range(0..len);
        
        let acc1 = &accelerators[idx1];
        let acc2 = &accelerators[idx2];
        
        // If one is unavailable, pick the other if available
        if !acc1.is_available() {
            return if acc2.is_available() { Some(idx2) } else { None };
        }
        if !acc2.is_available() {
            return Some(idx1);
        }
        
        // Both available, pick the one with less load
        if acc1.current_load <= acc2.current_load {
            Some(idx1)
        } else {
            Some(idx2)
        }
    }
}
