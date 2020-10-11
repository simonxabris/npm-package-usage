use clap::{App, Arg};
use std::env::current_dir;
use std::fs;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

static IGNORABLE_DIRS: [&str; 2] = ["node_modules", "build"];

fn main() {
    let args = App::new("NPM package Usage")
        .arg(Arg::with_name("package-name").required(true))
        .get_matches();

    let package_name = args
        .value_of("package-name")
        .expect("Please supply a package name");

    let mut default_current_dir = PathBuf::new();
    default_current_dir.push(".");

    let current_dir = current_dir().unwrap_or(default_current_dir);

    let files = find_files(current_dir);

    let files_importing_package: Vec<String> = files
        .into_iter()
        .filter(|file_name| {
            let file_contents = read_to_string(file_name).expect("Invalid filename");

            file_contents.contains(package_name)
        })
        .collect();

    for file in &files_importing_package {
        println!("{}", file);
    }

    println!(
        "The package {} is used in {} file(s).",
        package_name,
        files_importing_package.len()
    );
}

fn find_files<T: AsRef<Path>>(dir: T) -> Vec<String> {
    let entries = fs::read_dir(&dir).unwrap();
    let mut files = Vec::new();

    for entry in entries {
        let entry = entry.unwrap();

        let path = entry.path();

        let is_dir = path.is_dir();

        let name = path.file_name().unwrap().to_str().unwrap();

        let should_ignore = IGNORABLE_DIRS.contains(&name);

        if is_dir && !should_ignore {
            let f = find_files(&path);
            files.extend(f);
        } else {
            let is_ts = name
                .split(".")
                .last()
                .filter(|ext| *ext == "ts" || *ext == "js")
                .is_some();

            if is_ts {
                let path = dir.as_ref().to_str().unwrap();

                let stripped_path: String = path.chars().filter(|char| *char != '"').collect();

                let full_path = format!("{}/{}", stripped_path, name);

                files.push(full_path);
            }
        }
    }

    files
}