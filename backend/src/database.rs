use mongodb::coll::Collection;
use mongodb::db::ThreadedDatabase;
use mongodb::{Array, bson, Bson, doc, Client, Document, ThreadedClient};

#[derive(Debug)]
pub struct MongoDB {
    pub users: Collection,
}

impl MongoDB {
    fn find_user_doc(&self, user: &str) -> Option<Document> {
        let doc = doc!{ 
            "user": user 
        };
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

    fn strings_to_bson_array(strings: Vec<String>) -> Array {
        let mut array = Vec::new();
        for string in strings {
            array.push(Bson::String(string));
        }
        array
    }

    pub fn bson_array_to_strings(array: Array) -> Vec<String> {
        let mut strings = Vec::new();
        for bson in array {
            if let Bson::String(bson_string) = bson {
                strings.push(bson_string);
            }
        }
        strings
    }

    pub fn does_name_exist(&self, name: &str) -> bool {
        let doc = doc!{ 
            "name": name 
        };
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
        let doc = doc!{
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

    pub fn new(db_name: &str) -> MongoDB {
        let client =
            Client::connect("localhost", 27017).expect("Failed to initialize database client");
        let db = client.db(db_name);
        let users = db.collection("users");
        MongoDB { users }
    }

    pub fn password(&self, name: &str, pass: &str) -> bool {
        let filter = doc!{ 
            "name": name 
        };
        let update = doc!{ 
            "$set": { 
                "pass": pass 
            } 
        };
        match self.users.update_one(filter, update, None) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn register(&self, user: &str, pass: &str) -> bool {
        if self.find_user_doc(user).is_some() {
            return false;
        }
        let doc = doc!{
            "user": user,
            "pass": pass,
            "name": user,
        };
        match self.users.insert_one(doc.clone(), None) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn unthrust(&self, name: &str) -> bool {
        let filter = doc!{ 
            "name": name 
        };
        let array: Array = Vec::new();
        let update = doc!{ 
            "$set": { 
                "thrusters": array.clone(),
                "thrustees": array,
            }
        };
        match self.users.update_one(filter, update, None) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn username(&self, name: &str, user: &str) -> bool {
        let filter = doc!{ 
            "name": name 
        };
        let update = doc!{ 
            "$set": { 
                "user": user 
            } 
        };
        match self.users.update_one(filter, update, None) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn thrustees(&self, name: &str, thrustees: Vec<String>) -> bool {
        let filter = doc!{ 
            "name": name 
        };
        let array = MongoDB::strings_to_bson_array(thrustees);
        let update = doc!{ 
            "$set": { 
                "thrustees": array 
            } 
        };
        match self.users.update_one(filter, update, None) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn thrusters(&self, name: &str, thrusters: Vec<String>) -> bool {
        let filter = doc!{ "name": name };
        let array = MongoDB::strings_to_bson_array(thrusters);
        let update = doc!{ 
            "$set": { 
                "thrusters": array 
            } 
        };
        match self.users.update_one(filter, update, None) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
