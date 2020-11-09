
use juniper::{
     EmptySubscription, RootNode
};

pub mod counter;
pub mod database;
pub mod mutations;
pub mod query;
pub mod user;
pub mod userinput;
pub mod userkind;

pub use counter::Counter;
pub use database::Database;
pub use mutations::Mutations;
pub use query::Query;
pub use user::User;
pub use userinput::UserInput;
pub use userkind::UserKind;


pub type Schema = RootNode<'static, Query, Mutations, EmptySubscription<Database>>;

pub fn schema() -> Schema {
    Schema::new(
        Query,
        Mutations,
        EmptySubscription::<Database>::new(),
    )
}
