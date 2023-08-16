use std::sync::{Arc, RwLock};

use bevy::prelude::Component;

#[derive(Clone, Component)]
pub struct TaskWrapper<T> {
    pub result: Arc<RwLock<Option<T>>>,
}

impl<T> TaskWrapper<T> {
    pub fn new() -> Self {
        Self {
            result: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn register(&mut self, t: impl std::future::Future<Output = T>) {
        let ret = t.await;
        let mut lock = self.result.write().unwrap();
        *lock = Some(ret);
    }
}
