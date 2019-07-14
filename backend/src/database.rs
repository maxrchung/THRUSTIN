use mongodb::coll::Collection;
use mongodb::db::ThreadedDatabase;
use mongodb::Document;
use mongodb::{bson, doc, Client, ThreadedClient};

#[derive(Debug)]
pub struct MongoDB {
    pub users: Collection,
}

impl MongoDB {
    pub fn new(db_name: &str) -> MongoDB {
        let client =
            Client::connect("localhost", 27017).expect("Failed to initialize database client");
        let db = client.db(db_name);
        let users = db.collection("users");
        MongoDB { users }
    }

    fn find_user_doc(&self, user: &str) -> Option<Document> {
        let doc = doc! { "user": user };
        let mut cursor = self
            .users
            .find(Some(doc.clone()), None)
            .ok()
            .expect("Failed to find login");
        // Return doc if found, otherwise None
        match cursor.next() {
            Some(Ok(doc)) => Some(doc),
            Some(Err(_)) => None,
            None => None,
        }
    }

    pub fn does_name_exist(&self, name: &str) -> bool {
        let doc = doc! { "name": name };
        let mut cursor = self
            .users
            .find(Some(doc.clone()), None)
            .ok()
            .expect("Failed to find login");
        // bool for found
        match cursor.next() {
            Some(Ok(_)) => true,
            Some(Err(_)) => false,
            None => false,
        }
    }

    pub fn login(&self, user: &str, pass: &str) -> Option<Document> {
        let doc = doc! {
            "user": user,
            "pass": pass
        };
        let mut cursor = self
            .users
            .find(Some(doc.clone()), None)
            .ok()
            .expect("Failed to find login");
        let item = cursor.next();
        // Return doc if found, otherwise None
        match item {
            Some(Ok(doc)) => Some(doc),
            Some(Err(_)) => None,
            None => None,
        }
    }

    pub fn register(&self, user: &str, pass: &str) -> bool {
        if self.find_user_doc(user).is_some() {
            return false;
        }

        let doc = doc! {
            "user": user,
            "pass": pass,
            "name": user
        };
        match self.users.insert_one(doc.clone(), None) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn add_thrustee(&self, user: &str, thrustee: &str) {}

    pub fn add_thruster(&self, user: &str, thruster: &str) {}

    pub fn unthrust(&self, user: &str) {}
}
