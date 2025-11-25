use crate::accelerator::Accelerator;
use crate::request::Request;
use crate::router::LoadBalancingStrategy;

pub struct LeastConnections;

impl LeastConnections {
    pub fn new() -> Self {
        Self
    }
}

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
