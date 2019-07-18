use argon2;
use mongodb::coll::Collection;
use mongodb::db::ThreadedDatabase;
use mongodb::{bson, doc, Array, Bson, Client, Document, ThreadedClient};
use rand::Rng;

#[derive(Debug)]
pub struct MongoDB {
    pub users: Collection,
    config: argon2::Config<'static>,
}

impl MongoDB {
    fn find_user_doc(&self, user: &str) -> Option<Document> {
        let doc = doc! {
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

    fn hash_password(&self, pass: &str) -> String {
        let mut rng = rand::thread_rng();
        // Damn this was hard
        let salt: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        let hash = argon2::hash_encoded(pass.as_bytes(), &salt, &self.config)
            .expect("Failed to hash password");
        String::from(hash)
    }

    fn verify_password(&self, hash: &str, pass: &str) -> bool {
        let matches =
            argon2::verify_encoded(hash, pass.as_bytes()).expect("Failed to verify password");
        matches
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
        let doc = doc! {
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
        let doc = doc! {
            "user": user,
        };
        let mut cursor = self
            .users
            .find(Some(doc.clone()), None)
            .ok()
            .expect("Failed to find user");
        let item = cursor.next();
        // Return doc if found, otherwise None
        match item {
            Some(Ok(doc)) => match doc.get("pass") {
                Some(&Bson::String(ref hash)) => {
                    if self.verify_password(hash, pass) {
                        Some(doc)
                    } else {
                        None
                    }
                }
                _ => None,
            },
            Some(Err(_)) => None,
            None => None,
        }
    }

    pub fn new(db_name: &str) -> MongoDB {
        let client =
            Client::connect("localhost", 27017).expect("Failed to initialize database client");
        let db = client.db(db_name);
        let users = db.collection("users");
        let config = argon2::Config::default();
        MongoDB { users, config }
    }

    pub fn password(&self, name: &str, pass: &str) -> bool {
        let filter = doc! {
            "name": name
        };
        let hash = self.hash_password(pass);
        let update = doc! {
            "$set": {
                "pass": &hash
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
        let hash = self.hash_password(pass);
        let doc = doc! {
            "user": user,
            "pass": &hash,
            "name": user,
        };
        match self.users.insert_one(doc.clone(), None) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn unthrust(&self, name: &str) -> bool {
        let filter = doc! {
            "name": name
        };
        let array: Array = Vec::new();
        let update = doc! {
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
        let filter = doc! {
            "name": name
        };
        let update = doc! {
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
        let filter = doc! {
            "name": name
        };
        let array = MongoDB::strings_to_bson_array(thrustees);
        let update = doc! {
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
        let filter = doc! { "name": name };
        let array = MongoDB::strings_to_bson_array(thrusters);
        let update = doc! {
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
