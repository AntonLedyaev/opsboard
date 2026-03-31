use std::sync::{Arc, Mutex};
use crate::queue::Queue;

#[derive(Clone)]
pub struct AppState {
    pub queue: Arc<Mutex<Queue>>,
}

impl AppState {
    pub fn new(queue: Queue) -> Self {
        Self{ queue: Arc::new(Mutex::new(queue)) }
    }
}
