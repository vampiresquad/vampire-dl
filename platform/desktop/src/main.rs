#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::path::Path;
// আমাদের নিজস্ব কোর ইঞ্জিন ইম্পোর্ট করা হলো
use vampire_core::controller::start_download;

/// এটি একটি Tauri Command। UI (Frontend) থেকে কল করলে এটি রান হবে।
#[tauri::command]
async fn start_new_download(url: String, save_path: String, threads: u64) -> Result<String, String> {
    let path = Path::new(&save_path);
    
    // আমাদের কোর ইঞ্জিনের start_download কল করা হচ্ছে
    match start_download(&url, path, threads).await {
        Ok(_) => Ok("Download completed successfully!".to_string()),
        Err(e) => Err(format!("Download failed: {}", e)),
    }
}

fn main() {
    // Tauri অ্যাপ ইনিশিয়ালাইজ করা এবং UI এর সাথে কোর ইঞ্জিনের কমান্ড যুক্ত করা
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_new_download])
        .run(tauri::generate_context!())
        .expect("Error while running VampireDL Desktop application");
}
