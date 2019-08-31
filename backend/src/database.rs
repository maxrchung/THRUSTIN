use crate::lobby::Lobby;
use argon2;
use chrono::{DateTime, Duration, TimeZone, Utc};
use mongodb::coll::Collection;
use mongodb::db::ThreadedDatabase;
use mongodb::{bson, doc, Array, Bson, Client, Document, ThreadedClient};
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Database {
    pub bans: Collection,
    pub users: Collection,
    // Cache bans so we don't have to constantly load from database for each call
    bans_cache: HashMap<String, (i64, DateTime<Utc>)>,
    config: argon2::Config<'static>,
}

impl Database {
    fn find_ban_doc(&self, ip_addr: &str) -> Option<Document> {
        let doc = doc! {
            "ip_addr": ip_addr
        };
        let mut cursor = self
            .bans
            .find(Some(doc.clone()), None)
            .ok()
            .expect("Failed to find ban");
        // Return doc if found, otherwise None
        match cursor.next() {
            Some(Ok(doc)) => Some(doc),
            Some(Err(_)) => None,
            None => None,
        }
    }

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

    fn load_bans(&mut self) {
        let cursor = self
            .bans
            .find(Some(doc! {}), None)
            .ok()
            .expect("Failed to load bans");

        self.bans_cache = HashMap::new();
        for doc in cursor {
            if let Ok(doc) = doc {
                let ip_addr = if let Ok(ip_addr) = doc.get_str("ip_addr") {
                    ip_addr.clone()
                } else {
                    ""
                };

                let duration = if let Ok(duration) = doc.get_i64("duration") {
                    duration.clone()
                } else {
                    0
                };

                let end = if let Ok(end) = doc.get_utc_datetime("end") {
                    end.clone()
                } else {
                    Utc.timestamp(0, 0)
                };

                self.bans_cache
                    .insert(String::from(ip_addr), (duration, end));
            }
        }
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
                    messages.push(format!("Points Earned So Far - {}", points));
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
				if let Some(&Bson::I32(level)) = doc.get("level") {
					if let Some(exp) = doc.get("exp") {
						messages.push(format!("Level - {}", level));
						let exponent: f32 = 2.15;
						let exp_needed = (level as f32).powf(exponent).round() as i32;
						messages.push(format!("Experience - {} / {}", exp, exp_needed));
					} else {
						messages.push(String::from("Level - 1"));
						messages.push(String::from("Experience - 0 / 1"));
					}
				} else {
					messages.push(String::from("Level - 1"));
					messages.push(String::from("Experience - 0 / 1"));
				}
            }
            None => {
                messages.push(String::from("Yo there's a bit of an epic problem. We couldn't find your account data lmao. What is going on."));
            }
        }
        messages
    }

    pub fn ban(&mut self, ip_addr: &str) -> bool {
        let doc = self.find_ban_doc(&ip_addr);
        match doc {
            // If ban exists, update time
            Some(doc) => {
                let duration = if let Ok(duration) = doc.get_i64("ip_addr") {
                    duration.clone()
                } else {
                    3600
                };
                let end = Utc::now() + Duration::seconds(duration);
                let update = doc! {
                    "$set": {
                        "duration": duration,
                        "end": end,
                    }
                };
                match self.bans.update_one(doc, update, None) {
                    Ok(_) => {
                        self.load_bans();
                        true
                    }
                    _ => false,
                }
            }
            // If ban doesn't exist, add new ban
            None => {
                // in seconds, starting at 1 hour
                let duration = 3600;
                let end = Utc::now() + Duration::seconds(duration);
                let doc = doc! {
                    "ip_addr": ip_addr,
                    "duration": duration,
                    "end": end,
                };
                match self.bans.insert_one(doc.clone(), None) {
                    Ok(_) => {
                        self.load_bans();
                        true
                    }
                    _ => false,
                }
            }
        }
    }

    pub fn bans(&self) -> Vec<String> {
        let mut messages = vec![String::from("Banned fellows from this server. Kill'em.")];
        let mut bans = Vec::new();
        for (ip_addr, (duration, end)) in self.bans_cache.clone() {
            bans.push(format!("{} {} {}", ip_addr, duration, end));
        }
        bans.sort_unstable_by(|a, b| a.cmp(&b));
        messages.append(&mut bans);
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
            self.users
                .update_one(filter, update, None)
                .expect("Failed to update chieftain");
            true
        } else {
            false
        }
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
                if let Ok(name) = doc.get_str("name") {
                    chieftains.push(String::from(name));
                }
            }
        }
        chieftains.sort_unstable_by(|a, b| a.cmp(&b));
        messages.append(&mut chieftains);
        messages
    }

    pub fn color(&self, name: &str, bg: &str, fg: &str) -> bool {
        let filter = doc! {
            "name": name
        };
        let update = doc! {
            "$set": {
                "bg": bg,
                "fg": fg,
            }
        };
        match self.users.update_one(filter, update, None) {
            Ok(_) => true,
            Err(_) => false,
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

    pub fn is_banned(&self, ip_addr: &str) -> Option<&(i64, DateTime<Utc>)> {
        if self.bans_cache.contains_key(&String::from(ip_addr)) {
            self.bans_cache.get(&String::from(ip_addr))
        } else {
            None
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
            Some(Ok(doc)) => match doc.get_str("pass") {
                Ok(hash) => {
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

    pub fn name(&self, old_name: &str, new_name: &str) -> bool {
        let filter = doc! {
            "name": old_name
        };
        let update = doc! {
            "$set": {
                "name": new_name
            }
        };
        match self.users.update_one(filter, update, None) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn new(db_name: &str) -> Database {
        let client =
            Client::connect("localhost", 27017).expect("Failed to initialize database client");
        let db = client.db(db_name);
        let bans = db.collection("bans");
        let users = db.collection("users");
        let config = argon2::Config::default();

        let mut db = Database {
            bans,
            users,
            config,
            bans_cache: HashMap::new(),
        };
        db.load_bans();
        db
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
                self.users
                    .update_one(filter, update, None)
                    .expect("Failed to update games played");
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
                self.users
                    .update_one(filter, update, None)
                    .expect("Failed to update games won");
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
                self.users
                    .update_one(filter, update, None)
                    .expect("Failed to update points gained");
            }
        }
    }

	// Whenever a player is in a lobby that has ended, winner or not
	pub fn up_exp(&self, name: &str, exp_gained: i32) {
		if let Some(doc) = self.find_name_doc(&name) {
            if let Some(&Bson::I32(exp)) = doc.get("exp") {
                let filter = doc! {
                    "name": name
                };
                let update = doc! {
                    "$set": {
                        "exp": exp + exp_gained
                    }
                };
                self.users
                    .update_one(filter, update, None)
                    .expect("Failed to update exp");
            }
        }
	}

	// When a player gains enough EXP to level up, level is increased and EXP decreases by amount required to level up
	pub fn up_level(&self, name: &str, exp_to_level: i32) {
        if let Some(doc) = self.find_name_doc(&name) {
			if let Some(&Bson::I32(exp)) = doc.get("exp") {
				if let Some(&Bson::I32(level)) = doc.get("level") {
					let filter = doc! {
						"name": name
					};
					let update = doc! {
						"$set": {
							"level": level + 1,
							"exp": exp - exp_to_level
						}
					};
					self.users
						.update_one(filter, update, None)
						.expect("Failed to update level");
				}
			}
        }
    }

    pub fn register(&self, user: &str, pass: &str) -> bool {
        if self.find_user_doc(user).is_some() {
            return false;
        }
        let hash = self.hash_password(pass);
        let doc = doc! {
            "bg": "b7410e",
            "fg": "000",
            "user": user,
            "pass": &hash,
            "name": user,
            "points_gained": 0,
            "games_played": 0,
            "games_won": 0,
            "is_chieftain": false,
			"exp": 0,
			"level": 1,
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
			"level": 1,
			"exp": 0,
            "is_chieftain": true
        };
        match self.users.insert_one(doc.clone(), None) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn unban(&mut self, ip_addr: &str) -> bool {
        let doc = self.find_ban_doc(&ip_addr);
        match doc {
            // Set ban time to now if found
            Some(doc) => {
                let end = Utc::now();
                let update = doc! {
                    "$set": {
                        "end": end,
                    }
                };
                match self.bans.update_one(doc, update, None) {
                    Ok(_) => {
                        self.load_bans();
                        true
                    }
                    _ => false,
                }
            }
            None => false,
        }
    }

    pub fn unchieftain(&self, name: &str) -> bool {
        // Return false if user doesn't exist
        if self.find_name_doc(name).is_none() {
            return false;
        }
        let filter = doc! {
            "name": name
        };
        let update = doc! {
            "$set": {
                "is_chieftain": false
            }
        };
        match self.users.update_one(filter, update, None) {
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
        let array = Database::strings_to_bson_array(thrustees);
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
        let array = Database::strings_to_bson_array(thrusters);
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
