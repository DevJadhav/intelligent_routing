use crate::accelerator::Accelerator;
use crate::request::Request;

pub trait LoadBalancingStrategy: Send + Sync {
    fn select_accelerator(&self, accelerators: &[Accelerator], request: &Request) -> Option<usize>;
}

pub struct Router {
    pub accelerators: Vec<Accelerator>,
    strategy: Box<dyn LoadBalancingStrategy>,
}

impl Router {
    pub fn new(strategy: Box<dyn LoadBalancingStrategy>) -> Self {
        Self {
            accelerators: Vec::new(),
            strategy,
        }
    }

    pub fn add_accelerator(&mut self, accelerator: Accelerator) {
        self.accelerators.push(accelerator);
    }

    pub fn route_request(&mut self, request: &Request) -> Option<usize> {
        let idx = self.strategy.select_accelerator(&self.accelerators, request)?;
        // Ideally we would update load here or return the index for the caller to handle
        // For simulation purposes, let's assume the router updates the load immediately if successful
        if let Some(acc) = self.accelerators.get_mut(idx) {
             if acc.add_load(request.cost).is_ok() {
                 return Some(acc.id);
             }
        }
        None
    }
}
