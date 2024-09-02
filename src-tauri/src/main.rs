// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{collections::HashMap,};
use std::fs::File;
use std::io::Write;
use serde_json;
use chrono::Local;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn rate(ratings: HashMap<i8, i8>, prefix: &str) -> Result<(), String> {

    let json = serde_json::to_string_pretty(&ratings).map_err(|e| e.to_string())?;
    

    let date = Local::now().format("%Y-%m-%d").to_string();
    let filename = format!("{}-{}.json", prefix, date);
    let mut file = File::create(&filename).map_err(|e| e.to_string())?;
    file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;


    for (key, value) in &ratings {
        println!("{}: {}", key, value);
    }    

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![rate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
