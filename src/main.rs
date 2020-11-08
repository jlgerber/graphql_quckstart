use std::collections::HashMap;

use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, FieldError, GraphQLEnum, RootNode,
};
use warp::{http::Response, Filter};

#[derive(Clone, Debug)]
struct Database {
    users: HashMap<i32,User>,
}

impl juniper::Context for Database {}

impl Database {
    pub fn new() -> Self {
        let mut db = HashMap::new();
        db.insert(1, User{ id: 1, name: "Fred Flinstone".into(), kind: UserKind::Guest, friend_ids: Vec::new() });
        db.insert(2, User{id: 2, name: "Robert Redford".into(), kind: UserKind::Admin, friend_ids: vec![1]});
        db.insert(3, User{id: 3, name: "Lacy Ludlum".into(), kind: UserKind::User, friend_ids: vec![1,2]});
        db.insert(4, User{id: 4, name: "Guy Smiley".into(), kind: UserKind::User, friend_ids: vec![3]});
        db.insert(5, User{id: 5, name: "Ham Sandwich".into(), kind: UserKind::User, friend_ids: vec![1,2]});
        db.insert(6, User{id: 6, name: "Rudolf Rougeshnoz".into(), kind: UserKind::User, friend_ids: vec![1,2,3]});
        db.insert(7, User{id: 7, name: "Donald Dumkoff".into(), kind: UserKind::User, friend_ids: vec![]});

        Database {
            users: db
        }
    }
}

#[derive(Clone, Copy, Debug, GraphQLEnum)]
/// The type of user
enum UserKind {
    Admin,
    User,
    Guest,
}

#[derive(Clone, Debug)]
struct User {
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
        .map(|id| context.users.get(id).expect("couldnt get from id"))
        .collect()
    }
}

#[derive(Clone, Copy, Debug)]
struct Query;

#[graphql_object(context = Database)]
/// Query data from the users db
impl Query {
    /// Retrieve users
    async fn users(context: &Database) -> Vec<&User> {
        context.users.values().collect()
    }
    /// Retrieve a single user
    async fn user(context: &Database, id: i32, name: Option<String>) -> Option<&User> {
        context.users.get(&id)
    }

    /// Fetch a URL and return the response body text.
    async fn request(url: String) -> Result<String, FieldError> {
        Ok(reqwest::get(&url).await?.text().await?)
    }
}

type Schema = RootNode<'static, Query, EmptyMutation<Database>, EmptySubscription<Database>>;

fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Database>::new(),
        EmptySubscription::<Database>::new(),
    )
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "warp_async");
    env_logger::init();

    let log = warp::log("warp_server");

    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(format!(
                "<html><h1>juniper_warp</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>"
            ))
    });

    log::info!("Listening on 127.0.0.1:8080");
   
    let state = warp::any().map(move || Database::new());
    let graphql_filter = juniper_warp::make_graphql_filter(schema(), state.boxed());

    warp::serve(
        warp::get()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql", None))
            .or(homepage)
            .or(warp::path("graphql").and(graphql_filter))
            .with(log),
    )
    .run(([127, 0, 0, 1], 8080))
    .await
}