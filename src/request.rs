#[derive(Debug, Clone)]
pub struct Request {
    pub id: usize,
    pub cost: u32,
    pub priority: u8,
}

impl Request {
    pub fn new(id: usize, cost: u32, priority: u8) -> Self {
        Self { id, cost, priority }
    }
}
