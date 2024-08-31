use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::ArgMatches;

pub mod config;

fn main() -> std::io::Result<()> {
    let flags = config::read_flags();

    build_file_tree(flags)
}

#[derive(PartialEq)]
enum FlType {
    Dir,
    File,
    Other,
}

struct Node {
    name: String,
    path: PathBuf,
    f_type: FlType,
    level: u8,
}

impl Node {
    fn new(name: impl Into<String>, path: PathBuf, f_type: FlType, level: u8) -> Node {
        Node {
            name: name.into(),
            path,
            f_type,
            level,
        }
    }
}

fn build_file_tree(flags: ArgMatches) -> std::io::Result<()> {
    let mut stack: Vec<Node> = Vec::new();

    stack.push(Node::new(".", Path::new(".").to_path_buf(), FlType::Dir, 0));

    while !stack.is_empty() {
        if let Some(node) = stack.pop() {
            let cur_level = node.level;

            if cur_level <= *flags.get_one("Level").expect("Level is required") {
                if node.f_type == FlType::Dir {
                    let cur_dir_entries = fs::read_dir(node.path.clone())?;

                    for entry in cur_dir_entries {
                        let entry = entry?;

                        let file_name = entry.file_name().to_string_lossy().into_owned();

                        if !flags.contains_id("All") {
                            if let Some(first_char) = file_name.chars().next() {
                                if first_char == '.' {
                                    continue;
                                }
                            }
                        }

                        let f_type;

                        match entry.file_type()?.is_dir() {
                            true => f_type = FlType::Dir,
                            false => match entry.file_type()?.is_file() {
                                true => f_type = FlType::File,
                                false => f_type = FlType::Other,
                            },
                        }

                        stack.push(Node::new(file_name, entry.path(), f_type, cur_level + 1));
                    }
                }

                render_node(&node);
            }
        }
    }

    Ok(())
}

fn render_node(node: &Node) {
    let indent: String = format!(
        "{}{}",
        "  ".repeat(node.level.saturating_sub(1).into()),
        "|-- "
    );

    let name_to_render = match &node.f_type {
        FlType::Dir => format!("{}/", node.name),
        FlType::File => node.name.clone(),
        FlType::Other => format!("{}*", node.name),
    };

    println!("{indent}{name_to_render}")
}
