use std::sync::MutexGuard;

use crate::schema::errors::{Error, Result};

pub struct Mutex<T> {
    lock: std::sync::Mutex<T>,
}

impl<T> Mutex<T> {
    pub fn new(value: T) -> Self {
        Self {
            lock: std::sync::Mutex::new(value),
        }
    }

    pub fn lock(&self) -> Result<MutexGuard<'_, T>> {
        self.lock
            .lock()
            .map_err(|err| Error::invalid_argument(&err.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::Mutex;

    #[test]
    pub fn acquire_lock() {
        let mutex = Mutex::new(0);

        let lock = Mutex::lock(&mutex).unwrap();

        assert_eq!(*lock, 0);
    }
}
