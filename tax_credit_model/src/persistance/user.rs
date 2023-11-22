use std::collections::HashMap;

use crate::{
    concurrency::mutex::Mutex,
    schema::{
        errors::{Error, Result},
        user::{User, UserId},
    },
};

pub trait UserClient: Sync + Send {
    fn get_user_by_id(&self, user_id: &UserId) -> Result<User>;
    fn update_user(&self, user: &User) -> Result<User>;
    fn create_user(&self, user: &User) -> Result<User>;
}

pub struct InMemoryUserClient {
    id: Mutex<UserId>,
    user_by_id: Mutex<HashMap<usize, User>>,
}

impl InMemoryUserClient {
    pub fn new() -> Self {
        Self {
            id: Mutex::new(0),
            user_by_id: Mutex::new(HashMap::new()),
        }
    }
}

impl UserClient for InMemoryUserClient {
    fn get_user_by_id(&self, user_id: &UserId) -> Result<User> {
        self.user_by_id
            .lock()?
            .get(user_id)
            .map(|user| user.clone())
            .ok_or_else(|| Error::not_found("User not found"))
    }

    fn update_user(&self, user: &User) -> Result<User> {
        *self
            .user_by_id
            .lock()?
            .get_mut(user.id())
            .ok_or_else(|| Error::not_found("User not found"))? = user.clone();

        Ok(user.clone())
    }

    fn create_user(&self, user: &User) -> Result<User> {
        let mut user = user.clone();
        let mut id = self.id.lock()?;
        user.set_id(&id);
        *id += 1;

        self.user_by_id
            .lock()?
            .insert(user.id().clone(), user.clone());

        Ok(user)
    }
}
