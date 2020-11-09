use std::collections::HashMap;

use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, FieldError, GraphQLEnum, RootNode,
};


#[derive(Clone, Debug)]
pub struct Database {
    id: HashMap<i32,User>,
    name: HashMap<String, User>,
    kind: HashMap<UserKind, Vec<User>>
}

impl Default for Database {
    fn default() -> Self {
        let mut kinddb = HashMap::new();
        kinddb.insert(UserKind::Admin, Vec::new());
        kinddb.insert(UserKind::Guest, Vec::new());
        kinddb.insert(UserKind::User, Vec::new());
        Self {
            id: HashMap::new(),
            name: HashMap::new(),
            kind: kinddb
        }
    }
}
impl juniper::Context for Database {}

impl Database {
    pub fn insert(&mut self, user: User) {
        let id = user.id;
        let name = user.name.clone();
        let kind = user.kind.clone();
        self.id.insert(id, user.clone());
        self.name.insert(name, user.clone());
        match self.kind.get_mut(&kind) {
            Some(vals) => vals.push(user),
            None => panic!("should never reach here")
        };
    }

    pub fn new() -> Self {
        let mut db = Database::default();
        db.insert(User{ id: 1, name: "Fred Flinstone".into(), kind: UserKind::Guest, friend_ids: vec![8,9,10] });
        db.insert(User{id: 2, name: "Robert Redford".into(), kind: UserKind::Admin, friend_ids: vec![1]});
        db.insert(User{id: 3, name: "Lacy Ludlum".into(), kind: UserKind::User, friend_ids: vec![1,2]});
        db.insert(User{id: 4, name: "Guy Smiley".into(), kind: UserKind::User, friend_ids: vec![3]});
        db.insert(User{id: 5, name: "Ham Sandwich".into(), kind: UserKind::User, friend_ids: vec![1,2]});
        db.insert(User{id: 6, name: "Rudolf Rougeshnoz".into(), kind: UserKind::User, friend_ids: vec![1,2,3]});
        db.insert(User{id: 7, name: "Donald Dumkoff".into(), kind: UserKind::User, friend_ids: vec![]});
        db.insert(User{ id: 8, name: "Wilma Flinstone".into(), kind: UserKind::Guest, friend_ids: vec![9,1] });
        db.insert(User{ id: 9, name: "Barney Rubble".into(), kind: UserKind::User, friend_ids: vec![1,8,10] });
        db.insert(User{ id: 10, name: "Betty Rubble".into(), kind: UserKind::User, friend_ids: vec![1,8,9] });

        db
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, GraphQLEnum)]
/// The type of user
pub enum UserKind {
    Admin,
    User,
    Guest,
}

#[derive(Clone, Debug)]
pub struct User {
    id: i32,
    kind: UserKind,
    name: String,
    friend_ids: Vec<i32>
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
    async fn friends(&self, context: &Database) -> Vec<&User> {
        self.friend_ids.iter()
        .map(|id| context.id.get(id).expect("couldnt get from id"))
        .collect()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Query;

#[graphql_object(context = Database)]
/// Query data from the users db
impl Query {
    /// Retrieve users
    async fn users(context: &Database, name: Option<String>) -> Vec<&User> {
        match name {
            None =>  context.id.values().collect(),
            Some(name) =>  context.name.iter().filter_map(|(key, value)| if key.contains(&name){Some(value)}else {None}).collect()

        }
    }
  
    /// Retrieve a single user
    async fn user(context: &Database, id: i32) -> Option<&User> {
        context.id.get(&id)
    }

    ///Get user with the supplied name
    async fn named(context: &Database, name: String) -> Option<&User> {
        context.name.get(&name)
    }

    /// Fetch a URL and return the response body text.
    async fn request(url: String) -> Result<String, FieldError> {
        Ok(reqwest::get(&url).await?.text().await?)
    }
}

pub type Schema = RootNode<'static, Query, EmptyMutation<Database>, EmptySubscription<Database>>;

pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Database>::new(),
        EmptySubscription::<Database>::new(),
    )
}
