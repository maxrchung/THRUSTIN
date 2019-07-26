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
    fn find_name_doc(&self, name: &str) -> Option<Document> {
        let doc = doc! {
            "name": name
        };
        let mut cursor = self
            .users
            .find(Some(doc.clone()), None)
            .ok()
            .expect("Failed to find name");
        // Return doc if found, otherwise None
        match cursor.next() {
            Some(Ok(doc)) => Some(doc),
            Some(Err(_)) => None,
            None => None,
        }
    }

    fn find_user_doc(&self, user: &str) -> Option<Document> {
        let doc = doc! {
            "user": user
        };
        let mut cursor = self
            .users
            .find(Some(doc.clone()), None)
            .ok()
            .expect("Failed to find user");
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

    pub fn account(&self, name: &str) -> Vec<String> {
        let mut messages = Vec::new();
        match self.find_name_doc(&name) {
            Some(doc) => {
                messages.push(String::from("A display of your account information and statistical information. Please enjoy THRUSTIN!"));
                if let Some(Bson::String(user)) = doc.get("user") {
                    messages.push(format!("Username - {}", user));
                }
                messages.push(format!("Name - {}", name));
                messages.push(String::from("Password - [ENCRYPTED_CONTENT__UNVIEWABLE]"));
                if let Some(points) = doc.get("points_gained") {
                    messages.push(format!("Pointed Earned So Far - {}", points));
                } else {
                    messages.push(String::from("Pointed Earned - 0"));
                }
                if let Some(games) = doc.get("games_played") {
                    messages.push(format!("Games Played So Far - {}", games));
                } else {
                    messages.push(String::from("Games Played So Far - 0"));
                }
                if let Some(games) = doc.get("games_won") {
                    messages.push(format!("Games Won So Far - {}", games));
                } else {
                    messages.push(String::from("Games Won So Far - 0"));
                }
            }
            None => {
                messages.push(String::from("Yo there's a bit of an epic problem. We couldn't find your account data lmao. What is going on."));
            }
        }
        messages
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

    // Shows a list of chieftains
    pub fn chieftains(&self) -> Vec<String> {
        let mut messages = Vec::new();
        let doc = doc! {
            "is_chieftain": true
        };
        let cursor = self
            .users
            .find(Some(doc.clone()), None)
            .ok()
            .expect("Failed to find chieftains");

        messages.push(String::from("A LIST OF CHIEFTAINS RESPONSIBLE FOR MANAGEMENT OF THIS THRUSTIN SERVER IS AS FOLLOWS."));
        let mut chieftains = Vec::new();
        for doc in cursor {
            if let Ok(doc) = doc {
                if let Some(&Bson::String(ref name)) = doc.get("name") {
                    chieftains.push(name.clone());
                }
            }
        }
        chieftains.sort_unstable_by(|a, b| a.cmp(&b));
        messages.append(&mut chieftains);
        messages
    }

    // lol, this appoints a chieftain
    pub fn chieftain(&self, name: &str) -> bool {
        // Only set chieftain if it's an actual database bro
        if let Some(_) = self.find_name_doc(&name) {
            let filter = doc! {
                "name": name
            };
            let update = doc! {
                "$set": {
                    "is_chieftain": true
                }
            };
            self.users.update_one(filter, update, None).expect("Failed to update chieftain");
            true
        } else {
            false
        }
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

    pub fn is_chieftain(&self, name: &str) -> bool {
        if let Some(doc) = self.find_name_doc(&name) {
            if let Some(&Bson::Boolean(is_chieftain)) = doc.get("is_chieftain") {
                is_chieftain
            } else {
                false
            }
        } else {
            false
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

    // Whenever user joins or starts game
    pub fn up_games_played(&self, name: &str) {
        if let Some(doc) = self.find_name_doc(&name) {
            if let Some(&Bson::I32(games)) = doc.get("games_played") {
                let filter = doc! {
                    "name": name
                };
                let update = doc! {
                    "$set": {
                        "games_played": games + 1
                    }
                };
                self.users.update_one(filter, update, None).expect("Failed to update games played");
            }
        }
    }

    // Whenever user wins game
    pub fn up_games_won(&self, name: &str) {
        if let Some(doc) = self.find_name_doc(&name) {
            if let Some(&Bson::I32(games)) = doc.get("games_won") {
                let filter = doc! {
                    "name": name
                };
                let update = doc! {
                    "$set": {
                        "games_won": games + 1
                    }
                };
                self.users.update_one(filter, update, None).expect("Failed to update games won");
            }
        }
    }

    // When user gets a point
    pub fn up_points_gained(&self, name: &str) {
        if let Some(doc) = self.find_name_doc(&name) {
            if let Some(&Bson::I32(points)) = doc.get("points_gained") {
                let filter = doc! {
                    "name": name
                };
                let update = doc! {
                    "$set": {
                        "points_gained": points + 1
                    }
                };
                self.users.update_one(filter, update, None).expect("Failed to update points gained");
            }
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
            "points_gained": 0,
            "games_played": 0,
            "games_won": 0,
            "is_chieftain": false
        };
        match self.users.insert_one(doc.clone(), None) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    // For testing purposes
    pub fn register_chieftain(&self) -> bool {
        let hash = self.hash_password("chieftain");
        let doc = doc! {
            "user": "chieftain",
            "pass": &hash,
            "name": "chieftain",
            "points_gained": 0,
            "games_played": 0,
            "games_won": 0,
            "is_chieftain": true
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
