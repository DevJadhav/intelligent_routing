#[derive(Debug, Clone)]
pub struct Accelerator {
    pub id: usize,
    pub capacity: u32,
    pub current_load: u32,
    pub health_status: bool,
}

impl Accelerator {
    pub fn new(id: usize, capacity: u32) -> Self {
        Self {
            id,
            capacity,
            current_load: 0,
            health_status: true,
        }
    }

    pub fn update_load(&mut self, load: u32) {
        self.current_load = load;
    }

    pub fn is_available(&self) -> bool {
        self.health_status && self.current_load < self.capacity
    }
    
    pub fn add_load(&mut self, load: u32) -> Result<(), String> {
        if self.current_load + load > self.capacity {
            return Err("Capacity exceeded".to_string());
        }
        self.current_load += load;
        Ok(())
    }
    
    pub fn remove_load(&mut self, load: u32) {
        if load > self.current_load {
            self.current_load = 0;
        } else {
            self.current_load -= load;
        }
    }
}
