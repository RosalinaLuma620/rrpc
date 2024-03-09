use std::fs;

fn main() {
    let mut config_path = dirs::home_dir().expect("Failed to find home directory");
    config_path.push(".rrpc.config");

    let config_content = fs::read_to_string(config_path).expect("Failed to read the config file");

    config_content.lines().for_each(|line| {
        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() == 2 {
            println!("\x1b[33m{}=\x1b[0m\x1b[35m{}\x1b[0m", parts[0], parts[1]);
        } else {
            println!(" ");
        }
    });
}
