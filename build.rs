//! All the things to do before building the rest of the project

use std::{env, fs::{self}, path::{self, PathBuf}, io};


fn main() {
    // Cargo should re-run this file for any changes made to the resources directory
    println!("cargo:rerun-if-changed=resources");

    if cfg!(debug_assertions) {
        copy_resources_debug_mode();

    }
}

/// Copy the resource files to the usual debug directory
/// 
/// As the name states, this function should only be ran in debug mode.
fn copy_resources_debug_mode() {
    let resources_directory_name = "resources";

    let source_directory = path::absolute(resources_directory_name)
        .expect("failed to set the absolute path to the resources source directory");
    
    let debug_directory = path::absolute(PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("debug")
    )
        .expect("failed to set the absolute path for the debug directory");

    let destination_directory = path::absolute(PathBuf::from(&debug_directory)
        .join(resources_directory_name)
    )
        .expect("failed to set the absolute path for the destination directory");

    // Clean up and make sure no old, unused files are left behind.
    if destination_directory.exists() {
        fs::remove_dir_all(&destination_directory)
            .expect("Failed to delete the existing resources destination directory");
    }

    // Panic if the copy operation fails as this represents an unrecoverable error here.
    copy_directory(source_directory, destination_directory)
        .expect("The resources directory failed to be copied");
}

/// Copy all files and and directories of a (source) directory to another (destination) directory
/// 
/// This function uses depth-first search (DFS), and it ignores any symbolic links (which there 
/// should not be any to begin with) to prevent any cycles to already seen files and directories.
fn copy_directory(initial_source: PathBuf, initial_destination: PathBuf) -> Result<(), io::Error> {
    let mut stack: Vec<(PathBuf, PathBuf)> = Vec::new();
    stack.push((initial_source, initial_destination));

    while let Some((source, destination)) = stack.pop() {
        fs::create_dir_all(&destination)?;

        for entry_result in fs::read_dir(&source)? {
            let entry = entry_result?;
            let entry_type = entry.file_type()?;
            let from = entry.path();
            let to = destination.join(entry.file_name());

            if entry_type.is_dir() {
                stack.push((from, to));
            } 
            else if entry_type.is_file() {
                if let Some(parent) = to.parent() {
                    fs::create_dir_all(parent)?;
                }

                fs::copy(&from, &to)?;
            }
            else {
                // Symlinks (and any other unaccounted for type) should be skipped
                continue;
            }
        }
    }

    Ok(())
}
