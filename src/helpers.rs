use std::{fs, path::Path};

pub fn get_excluded_files() -> Vec<String> {
    let excluded_files_string =
        fs::read_to_string(".xccignore").expect("Unable to read .xccignore file");
    let excluded_files: Vec<String> = excluded_files_string
        .split("\n")
        .filter(|x| x.starts_with("-"))
        .map(|x| {
            x.replace("- ", "")
                .split("#")
                .next()
                .map(str::trim)
                .unwrap_or("")
                .to_string()
        })
        .collect();

    excluded_files
}

pub fn remove_filename(path: &str) -> String {
    let path = Path::new(path);
    match path.parent() {
        Some(parent) => parent.to_str().unwrap_or("").to_string(),
        None => path.to_str().unwrap_or("").to_string(),
    }
}
