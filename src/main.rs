use clap::{App, Arg};
use regex::Regex;
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

    let files_importing_package = filter_files(&files, &package_name);

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

/// This function searches the list of JS/TS files
/// and return those that import the specified package.
fn filter_files<'a>(files: &'a Vec<String>, package_name: &str) -> Vec<&'a String> {
    let import_regex_string = format!("import .+ from (\"|'){}(\"|')", package_name);
    let require_regex_string = format!("require\\((\"|'){}(\"|')\\)", package_name);
    let import_regex = Regex::new(&import_regex_string[..]).unwrap();
    let require_regex = Regex::new(&require_regex_string[..]).unwrap();

    let files_importing_package: Vec<&String> = files
        .iter()
        .filter(|file_name| {
            let file_contents = read_to_string(file_name).expect("Invalid filename");

            let imports_package = import_regex.is_match(&file_contents[..]);
            let requires_package = require_regex.is_match(&file_contents[..]);

            imports_package || requires_package
        })
        .collect();

    files_importing_package
}

#[cfg(test)]
mod tests {
    use super::{filter_files, find_files};
    use regex::Regex;

    #[test]
    fn find_all_files() {
        let files = find_files(".");

        assert_eq!(files.len(), 6);
    }

    #[test]
    fn filters_found_files_correctly() {
        let files = find_files(".");

        let filtered_files = filter_files(&files, "react");

        assert_eq!(filtered_files.len(), 4);
    }

    #[test]
    fn require_regex() {
        let require_regex_string = format!("require\\((\"|'){}(\"|')\\)", "react");
        let require_regex = Regex::new(&require_regex_string[..]).unwrap();

        assert_eq!(
            require_regex.is_match("const react = require(\"react\")"),
            true
        );
    }

    #[test]
    fn import_regex() {
        let import_regex_string = format!("import .+ from (\"|'){}(\"|')", "react");
        let import_regex = Regex::new(&import_regex_string[..]).unwrap();

        assert_eq!(
            import_regex.is_match("import * as React from 'react'"),
            true
        )
    }
}
