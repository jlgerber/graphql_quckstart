use crate::UserKind;

#[derive(Clone, Debug)]
pub struct UserInput {
    pub kind: UserKind,
    pub name: String,
    pub friend_ids: Vec<i32>
}
