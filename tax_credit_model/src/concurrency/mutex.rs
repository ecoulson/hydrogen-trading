use std::sync::MutexGuard;

use crate::schema::errors::{Error, Result};

pub struct Mutex<T> {
    lock: std::sync::Mutex<T>,
}

impl<T> Mutex<T> {
    pub fn new(value: T) -> Mutex<T> {
        Mutex {
            lock: std::sync::Mutex::new(value),
        }
    }

    pub fn lock(mutex: &Mutex<T>) -> Result<MutexGuard<'_, T>> {
        mutex
            .lock
            .lock()
            .map_err(|err| Error::create_invalid_argument_error(&err.to_string()))
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
