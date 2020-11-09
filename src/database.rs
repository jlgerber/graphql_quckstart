use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use crate::UserKind;
use crate::UserInput;
use crate::User;

#[derive(Clone, Debug)]
pub struct DbInner {
    pub id:HashMap<i32,User>,
    pub name: HashMap<String, User>,
    pub kind: HashMap<UserKind, Vec<User>>,
    pub index: usize
}


#[derive(Clone, Debug)]
pub struct Database {
   pub inner: Arc<Mutex<DbInner>>
}

impl Default for Database {
    fn default() -> Self {
        let mut kinddb = HashMap::new();
        kinddb.insert(UserKind::Admin, Vec::new());
        kinddb.insert(UserKind::Guest, Vec::new());
        kinddb.insert(UserKind::User, Vec::new());
        let inner = Arc::new(Mutex::new( DbInner{
            id: HashMap::new(),
            name: HashMap::new(),
            kind: kinddb,
            index: 0
        }));
        Self{inner}
    }
}
impl juniper::Context for Database {}

impl Database {
    /// insert an entry into the database and return the id for 
    /// the entry
    pub fn insert(&self, user: UserInput) -> i32 {
        // increment the ud and hold the previous id
        let mut db_ = self.inner.lock().unwrap();
        let id = db_.index as i32;
        db_.index +=1;
        let UserInput{kind, name, friend_ids} = user;
        // create instance of User from 
        let user = User {
            id, kind, name: name.clone(), friend_ids
        };
        // populate the "database"
       
        db_.id.insert(id, user.clone());
       
        db_.name.insert(name, user.clone());
       
        match db_.kind.get_mut(&kind) {
            Some(vals) => vals.push(user),
            None => panic!("should never reach here")
        };
        // return the id
        id
    }

    /// new up a database with example data
    pub fn new() -> Self {
        let  db = Database::default();
        db.insert(UserInput{ name: "Fred Flinstone".into(), kind: UserKind::Guest, friend_ids: vec![8,9,10] });
        db.insert(UserInput{ name: "Robert Redford".into(), kind: UserKind::Admin, friend_ids: vec![1]});
        db.insert(UserInput{ name: "Lacy Ludlum".into(), kind: UserKind::User, friend_ids: vec![1,2]});
        db.insert(UserInput{ name: "Guy Smiley".into(), kind: UserKind::User, friend_ids: vec![3]});
        db.insert(UserInput{ name: "Ham Sandwich".into(), kind: UserKind::User, friend_ids: vec![1,2]});
        db.insert(UserInput{ name: "Rudolf Rougeshnoz".into(), kind: UserKind::User, friend_ids: vec![1,2,3]});
        db.insert(UserInput{ name: "Donald Dumkoff".into(), kind: UserKind::User, friend_ids: vec![]});
        db.insert(UserInput{ name: "Wilma Flinstone".into(), kind: UserKind::Guest, friend_ids: vec![9,1] });
        db.insert(UserInput{ name: "Barney Rubble".into(), kind: UserKind::User, friend_ids: vec![1,8,10] });
        db.insert(UserInput{ name: "Betty Rubble".into(), kind: UserKind::User, friend_ids: vec![1,8,9] });

        db
    }
}


