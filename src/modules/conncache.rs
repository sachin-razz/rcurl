#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct ConnCache {
    pub active_connections: usize,
}

#[allow(dead_code)]
impl ConnCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn acquire_connection(&mut self) -> usize {
        self.active_connections += 1;
        self.active_connections
    }

    pub fn release_connection(&mut self) {
        if self.active_connections > 0 {
            self.active_connections -= 1;
        }
    }
}
