

use std::fs;

#[tauri::command]
fn list_files(dir: String) -> Result<Vec<String>, String> {
  let entries = fs::read_dir(dir)
    .map_err(|e| e.to_string())?
    .filter_map(|entry| entry.ok())
    .map(|entry| entry.file_name().to_string_lossy().into_owned())
    .collect();

  Ok(entries)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![list_files])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
