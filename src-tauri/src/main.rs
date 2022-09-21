#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};
use Tauri::State;

#[derive(Default)]
struct Counter(Arc<Mutex<i32>>);
fn main() {
  tauri::Builder::default()
    .manage(Counter(Default::default()))
    .invoke_handler(tauri::generate_handler![hello_world, add_count])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn hello_world() -> String {
  "Hello World!".to_string()
}

#[tauri::command]
fn add_count(num: i32, counter: State<'_, Counter>) -> String {
  let mut val = counter.0.lock().unwrap();
  *val += num;

  format!("{val}")
}