
use juniper::{
    graphql_object
};

use crate::UserKind;
use crate::Database;

#[derive(Clone, Debug)]
pub struct User {
    pub id: i32,
    pub kind: UserKind,
    pub name: String,
    pub friend_ids: Vec<i32>
}

#[graphql_object(context = Database)]
/// The User in the system
impl User {
    /// the id of the person
    fn id(&self) -> i32 {
        self.id
    }
    /// The kind of person
    fn kind(&self) -> UserKind {
        self.kind
    }
    /// The name of the person
    fn name(&self) -> &str {
        &self.name
    }
    /// The friends for a person
    async fn friends(&self, context: &Database) -> Vec<User> {
        self.friend_ids.iter()
        .map(|id| context.inner.lock().unwrap().id.get(id).unwrap().clone())
        .collect()
    }
}