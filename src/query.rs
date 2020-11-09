
use juniper::{
    graphql_object, FieldError
};
use crate::{Database, User};


#[derive(Clone, Copy, Debug)]
pub struct Query;

#[graphql_object(context = Database)]
/// Query data from the users db
impl Query {
    /// Retrieve users
    async fn users(context: &Database, name: Option<String>) -> Vec<User> {
        match name {
            None =>  context.inner.lock().unwrap().id.values().map(|v| v.clone()).collect(),
            Some(name) =>  context.inner.lock().unwrap().name.iter().filter_map(|(key, value)| if key.contains(&name){Some(value.clone())}else {None}).collect()

        }
    }
  
    /// Retrieve a single user
    async fn user(context: &Database, id: i32) -> Option<User> {
        context.inner.lock().unwrap().id.get(&id).cloned()
    }

    ///Get user with the supplied name
    async fn named(context: &Database, name: String) -> Option<User> {
        context.inner.lock().unwrap().name.get(&name).cloned()
    }

    /// Fetch a URL and return the response body text.
    async fn request(url: String) -> Result<String, FieldError> {
        Ok(reqwest::get(&url).await?.text().await?)
    }
}
