use regex::Regex;
use std::fs;
use std::path::Path;

fn is_unwanted_line(line: &str) -> bool {
    let patterns = [
        r"^\[Musique\]$",
        r"</c>",
        r"<\d{2}:\d{2}:\d{2},\d{3}><c>", // remove timespams
        r"^\s*$",
        r"^\d+$",
    ];

    patterns.iter().any(|&pattern| Regex::new(pattern).unwrap().is_match(line))
}

fn group_lines_by_timestamp(text: &str) -> String {
    let mut grouped_lines = String::new();
    let mut current_group = String::new();

    for line in text.lines() {
        if Regex::new(r"\d{2}:\d{2}:\d{2},\d{3}").unwrap().is_match(line) {
            if !current_group.is_empty() {
                grouped_lines.push_str(&current_group.trim());
                grouped_lines.push_str("\r\n\r\n");
                current_group.clear();
            }
        } else if !is_unwanted_line(line) {
            current_group.push_str(line);
            current_group.push(' ');
        }
    }

    if !current_group.is_empty() {
        grouped_lines.push_str(&current_group.trim());
    }

    grouped_lines
}

fn process_file(file_path: &str) -> Result<(), String> {
    if !Path::new(file_path).exists() {
        return Err(format!("{} does not exist", file_path));
    }

    let content = fs::read_to_string(file_path)
        .map_err(|_| format!("Failed to read file {}", file_path))?;

    let grouped_content = group_lines_by_timestamp(&content);

    let output_path = format!("{}.txt", file_path);
    fs::write(&output_path, &grouped_content)
        .map_err(|_| format!("Failed to write file {}", output_path))?;

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} your_file.srt", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    if let Err(e) = process_file(file_path) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
