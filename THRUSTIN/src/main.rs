mod lobby;


fn main() {
    let name: std::string::String = "name".to_string();
    let pw: std::string::String = "pw".to_string();
    let max: u32 = 12;
    let id: u32 = 1;

    let lobby = crate::lobby::lobby::new(name, pw, max, id);

    println!("{}", lobby.name);
}
