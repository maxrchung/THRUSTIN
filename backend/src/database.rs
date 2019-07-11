use mongodb::{Bson, bson, Client, doc, ThreadedClient};
use mongodb::Document;
use mongodb::coll::Collection;
use mongodb::db::ThreadedDatabase;

#[derive(Debug)]
pub struct MongoDB {
    users: Collection
}

impl MongoDB {
    pub fn new() -> MongoDB {
        let client = Client::connect("localhost", 27017).expect("Failed to initialize client");
        let db = client.db("THRUSTIN");
        let users = db.collection("users");
        MongoDB {
            users
        }
    }

    pub fn login(&self, user: &str, pass: &str) -> Option<Document> {
        let doc = doc! {
            "user": user,
            "pass": pass
        };
        let mut cursor = self.users.find(Some(doc.clone()), None).ok().expect("Failed to find login");
        let item = cursor.next();
        // Return doc if found, otherwise None
        match item {
            Some(Ok(doc)) => Some(doc),
            Some(Err(_)) => None,
            None => None,
        }
    }

    pub fn find_user(&self, user: &str) -> Option<Document> {
        let doc = doc! {
            "user": user
        };
        let mut cursor = self.users.find(Some(doc.clone()), None).ok().expect("Failed to find login");
        let item = cursor.next();
        // Return doc if found, otherwise None
        match item {
            Some(Ok(doc)) => Some(doc),
            Some(Err(_)) => None,
            None => None,
        }
    }

    pub fn register(&self, user: &str, pass: &str) -> bool {
        if self.find_user(user).is_some() {
            false;
        }
        let doc = doc! {
            "user": user,
            "pass": pass
        };
        match self.users.insert_one(doc.clone(), None) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub fn add_thrustee(&self, user: &str, thrustee: &str) {

    }

    pub fn add_thruster(&self, user: &str, thruster: &str) {

    }

    pub fn unthrust(&self, user: &str) {

    }
}