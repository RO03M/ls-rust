use std::error::Error;
use std::fs::{self, DirEntry, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

fn main() {
    run(Path::new("."), Some(false)).expect("Couldn't execute command");
}

fn run(dir: &Path, recursive: Option<bool>) -> Result<(), Box<dyn Error>> {
    let recursive = recursive.unwrap_or(true);

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let metadata = entry.metadata().expect("Couldn't get metadata");

            if metadata.is_dir() && recursive {
                run(entry.path().as_path(), Some(false)).expect("Couldn't execute command");
            }

            show_dir_entry(entry).expect("Couldn't get entry details");
        }
    }

    return Ok(());
}

fn show_dir_entry(entry: DirEntry) -> Result<(), Box<dyn Error>> {
    let file_size = entry.metadata()?.len();

    let permissions = entry.metadata()?.permissions();
    
    let permissions_string = extract_unix_mode_from_permissions(permissions);

    let file_name = entry
        .file_name()
        .into_string()
        .or_else(|f| Err(format!("invalid entry: {:?}", f)))?;

    println!("{} {} {}", permissions_string, file_size, file_name);

    return Ok(());
}

fn extract_unix_mode_from_permissions(permissions: Permissions) -> String {
    let permissions_mode = permissions.mode();
    let permissions_octal = format!("{:o}", permissions_mode);

    let permissions_octal_chars: Vec<char> = permissions_octal.chars().collect();

    if permissions_octal_chars.len() > 3 {
        let last_three = &permissions_octal_chars[permissions_octal_chars.len() - 3..];

        let mut unix_permission_string = String::from("");

        for char in last_three {
            let unix_permission_string_chunk = match char {
                '0' => "---",
                '1' => "--x",
                '2' => "-w-",
                '3' => "-wx",
                '4' => "r--",
                '5' => "r-x",
                '6' => "rw-",
                '7' => "rwx",
                _ => "Invalid number", // Handle other cases
            };

            unix_permission_string = format!("{}{}", unix_permission_string, unix_permission_string_chunk);
        }

        return unix_permission_string;
    } else {
        return String::from("---");
    }
}