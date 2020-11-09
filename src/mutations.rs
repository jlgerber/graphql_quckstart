
use juniper::{
    graphql_object, FieldResult
};
use crate::{Database, UserKind, UserInput, User};


pub struct Mutations;

#[graphql_object(
    Context = Database
)]
impl Mutations {
    /// add a user
    async fn addUser(context: &Database, name: String, kind: UserKind, friend_ids: Option<Vec<i32>>) -> FieldResult<Option<User>> {
        let friend_ids = friend_ids.unwrap_or_else(|| Vec::new());
        // prove that the context is working within he scope of the call...
        //let id = context.insert(UserInput{kind, name: name.clone(), friend_ids: friend_ids.clone()});
        let id = context.insert(UserInput{kind, name, friend_ids});
        let result = context.id.lock().unwrap().get(&id).cloned();
        println!("Result {:?}", result);
        Ok(result)
    }
}
