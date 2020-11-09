use juniper::{
   GraphQLEnum, 
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, GraphQLEnum)]
/// The type of user
pub enum UserKind {
    Admin,
    User,
    Guest,
}