use std::{env, fs, path::Path};

fn main() -> std::io::Result<()> {
    // command line flags parsing
    let args: Vec<String> = env::args().collect();

    let mut is_include_hidden: bool = false;

    for arg in args {
        if arg == "-a" {
            is_include_hidden = true;
        }
    }

    // programm logic
    let cur_path = Path::new(".");

    let cur_dir_entries = fs::read_dir(cur_path)?;

    for entry in cur_dir_entries {
        let entry = entry?;

        let file_name = entry.file_name();

        if !is_include_hidden {
            if let Some(first_char) = file_name.to_string_lossy().chars().next() {
                if first_char == '.' {
                    continue;
                }
            }
        }

        let file_type = entry.file_type()?;

        match file_type.is_dir() {
            true => println!("{}/", file_name.to_string_lossy()),
            false => match file_type.is_file() {
                true => println!("{}", file_name.to_string_lossy()),
                false => println!("{}*", file_name.to_string_lossy()),
            },
        }
    }
    Ok(())
}
