use colorize::AnsiColor;
use regex::Regex;
use std::process::Command;

use crate::{
    entities::{File, Xccov},
    helpers::{get_excluded_files, remove_filename},
};

pub fn generate_coverage(xcresult_path: &str) {
    println!(".:: X-CODE COVERAGE ::.");

    println!("\n⏳ Loading...\n");

    // Read and parse the JSON file
    let output = Command::new("xcrun")
        .args(["xccov", "view", "--report", "--json", xcresult_path])
        .output()
        .expect("Failed to execute command");
    let xccov_string = String::from_utf8_lossy(&output.stdout);

    let json: Xccov = serde_json::from_str(xccov_string.trim()).expect("Invalid JSON format");

    let mut covered_lines = 0;
    let mut executable_lines = 0;

    let excluded_files = get_excluded_files();
    let files = &json.targets.first().unwrap().files;

    let mut included_files: Vec<&File> = vec![];

    'outer: for file in files {
        for pattern in &excluded_files {
            let result = Regex::new(pattern).expect("Invalid regex pattern");

            if result.is_match(&file.path) {
                continue 'outer;
            }
        }

        covered_lines += file.covered_lines;
        executable_lines += file.executable_lines;
        included_files.push(file);
    }

    // Print the included files
    included_files.sort_by(|a, b| a.coverage().total_cmp(&b.coverage()));

    for file in included_files {
        let coverage = file.coverage();
        let emoji = if coverage >= 0.7 {
            "🟢"
        } else if (0.5..0.7).contains(&coverage) {
            "🟡"
        } else {
            "🔴"
        };

        let path = remove_filename(&file.path);

        println!(
            "{} {:.2}% [{}/{}] {}{}{}",
            emoji,
            coverage * 100.0,
            file.covered_lines,
            file.executable_lines,
            path.b_black(),
            "/".b_black(),
            file.name
        );
    }

    // Calculate coverage
    let coverage = if executable_lines > 0 {
        covered_lines as f64 / executable_lines as f64
    } else {
        0.0
    };
    let percentage = coverage * 100.0;

    println!("\n> Total Covered Lines: {}", covered_lines);
    println!("> Total Executable Lines: {}", executable_lines);

    println!("👉 CODE COVERAGE: {:.2}% 👈\n", percentage);
}
