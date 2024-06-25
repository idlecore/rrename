// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use regex::Regex;

#[tauri::command]
fn get_cwd() -> String {
    std::env::current_dir().unwrap()
        .to_str().unwrap()
        .to_string()
}

#[tauri::command]
fn test_file_regex(re: String) -> bool {
    matches!(Regex::new(&re), Ok(_))
}

#[tauri::command]
fn get_files(dir: String, re: String) -> Result<Vec<String>, &'static str> {
    let creg = Regex::new(&re).unwrap();
    if let Ok(files) = std::fs::read_dir(dir) {
        Ok(files.into_iter()
            .map(|x| x.unwrap().path())
            .filter(|x| x.is_file())
            .map(|x| x.file_name().unwrap().to_str().unwrap().to_string())
            .filter(|x| creg.is_match(x))
            .collect::<Vec<_>>())
    } else {
        Err("Invalid dir")
    }
}

#[tauri::command]
fn process_replacements(files: Vec<String>, re: String, out: String) -> Result<Vec<String>, &'static str> {
    if let Ok(creg) = Regex::new(&re) {
        Ok(files.into_iter()
            .map(|x| creg.replace(&x, &out).to_string())
            .collect::<Vec<_>>())
    } else {
        Err("Invalid regex")
    }
}

#[tauri::command]
fn rename(dir: String, original: Vec<String>, renamed: Vec<String>) -> Vec<String> {
    let mut report: Vec<String> = Vec::new();
    let path = std::path::Path::new(&dir);
    for file in std::iter::zip(original, renamed) {
        let result = std::fs::rename(path.join(file.0.clone()), path.join(file.1.clone()));
        match result {
            Ok(_) => { report.push(format!("{} -> {}", file.0.clone(), file.1.clone())); },
            Err(_) => { report.push(format!("Failed to rename {}", file.0.clone())); },
        }
    }
    return report;
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
                        get_cwd,
                        get_files,
                        test_file_regex,
                        process_replacements,
                        rename
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
