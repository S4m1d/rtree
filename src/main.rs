use std::{fs, path::Path};

fn main() -> std::io::Result<()> {
    let cur_path = Path::new(".");

    let cur_dir_entries = fs::read_dir(cur_path)?;

    for entry in cur_dir_entries {
        let entry = entry?;
        let file_name = entry.file_name();
        println!("{}", file_name.to_string_lossy())
    }
    Ok(())
}
