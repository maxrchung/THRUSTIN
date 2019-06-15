use std::fs;
use std::path::Path;
use std::thread;
use thrustin;
use thrustin::communication::ChannelCommunication;

pub fn setup() -> ChannelCommunication {
    let mut server = ChannelCommunication::new();
    let mut client = ChannelCommunication::new();
    ChannelCommunication::bind(&mut server, &mut client);
    thread::spawn(move || {
        thrustin::run_channel(server);
    });
    client
}

pub fn run_test_server(id: &str) {
    let lifetime = String::from(id);
    thread::spawn(move || {
        thrustin::run_fs_server(&lifetime);
    });
}

pub struct FileSystemClient {
    server_path: String,
    id: String,
}

impl FileSystemClient {
    pub fn new(server_path: &str, id: &str) -> FileSystemClient {
        FileSystemClient {
            server_path: String::from(server_path),
            id: String::from(id),
        }
    }

    fn block_if(&self, path: &str) {
        let path = Path::new(path);
        while path.exists() {}
    }

    fn block_for(&self, path: &str) {
        let path = Path::new(path);
        while !path.exists() {}
    }

    pub fn stop(&self) {
        self.block_for(&self.server_path);

        let end_path = format!("{}/end", &self.server_path);
        fs::write(end_path, "").expect("Unable to write end message");

        // Block and make sure server cleans up
        self.block_if(&self.server_path);
    }

    pub fn read(&self) -> String {
        self.block_for(&self.server_path);

        // Keep on looking until what we want is found
        loop {
            let dir;
            loop {
                match fs::read_dir(&self.server_path) {
                    Ok(read) => {
                        dir = read;
                        break;
                    }
                    _ => (),
                }
            }

            for entry in dir {
                let entry = entry.expect("Failed to make server entry");
                let path = entry.path();
                let os_file_name = path
                    .file_name()
                    .expect("Failed to get file name for client message");
                let file_name = os_file_name
                    .to_os_string()
                    .into_string()
                    .expect("Failed to convert OS String file name to String");

                let split: Vec<&str> = file_name.split("_____").collect();
                if split.len() == 2 && split[0] == "server" && split[1] == self.id {
                    let mut msg = String::new();
                    // Message may not be done reading yet, so keep on checking
                    while msg.is_empty() {
                        match fs::read_to_string(&path) {
                            Ok(contents) => msg = contents,
                            _ => (),
                        }
                    }

                    fs::remove_file(path).expect("Failed to remove client file");
                    return msg;
                }
            }
        }
    }

    pub fn send(&self, msg: &str) {
        self.block_for(&self.server_path);

        // Block if message hasn't been read yet
        let formatted = format!("{}/{}_____server", &self.server_path, &self.id);
        self.block_if(&formatted);

        fs::write(&formatted, msg).expect("Unable to write client message");
    }

    pub fn send_and_read(&self, msg: &str) -> String {
        self.send(msg);
        self.read()
    }

    pub fn name(&self) -> String {
        let command = format!(".n {}", self.id);
        self.send(&command);
        self.read()
    }
}
