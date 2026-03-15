use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::fs::File;
use tokio::io::{AsyncSeekExt, AsyncWriteExt};
use std::io::SeekFrom;
use std::error::Error;
use std::path::Path;

use crate::models::Segment;
use crate::network::fetch_metadata;
use crate::downloader::calculate_segments;
use crate::storage::create_file;

/// একটি নির্দিষ্ট সেগমেন্ট ডাউনলোড এবং ফাইলে সেভ করার অ্যাসিঙ্ক্রোনাস ফাংশন
async fn download_segment(
    client: Client,
    url: String,
    segment: Segment,
    file_mutex: Arc<Mutex<File>>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // সার্ভারকে বলা যে ফাইলের ঠিক কোন বাইট থেকে কোন বাইট পর্যন্ত আমরা চাই
    let range_header = format!("bytes={}-{}", segment.start_byte, segment.end_byte);
    let mut response = client.get(&url).header(reqwest::header::RANGE, range_header).send().await?;
    
    let mut current_offset = segment.start_byte;

    // সার্ভার থেকে ডেটা আসা শুরু হলে তা সরাসরি ফাইলে লেখা
    while let Some(chunk) = response.chunk().await? {
        // Mutex লক ব্যবহার করা যেন একাধিক থ্রেড একই সাথে ফাইলে লিখতে গিয়ে ক্র্যাশ না করে
        let mut file = file_mutex.lock().await;
        file.seek(SeekFrom::Start(current_offset)).await?;
        file.write_all(&chunk).await?;
        
        current_offset += chunk.len() as u64;
    }

    Ok(())
}

/// সম্পূর্ণ ডাউনলোড প্রসেস কন্ট্রোল করার মেইন ফাংশন
pub async fn start_download(url: &str, save_path: &Path, num_threads: u64) -> Result<(), Box<dyn Error>> {
    // ১. ফাইলের মেটাডেটা (সাইজ) বের করা
    let metadata = fetch_metadata(url).await?;
    let total_size = metadata.content_length;
    
    // ২. হার্ডডিস্কে জায়গা ফাঁকা (Pre-allocate) করা
    let file = create_file(save_path, total_size).await?;
    // ফাইলটিকে Arc ও Mutex দিয়ে মোড়ানো যেন সব থ্রেড নিরাপদে এটি ব্যবহার করতে পারে
    let shared_file = Arc::new(Mutex::new(file));
    
    // ৩. সার্ভার সাপোর্ট করলে থ্রেড অনুযায়ী সেগমেন্ট ভাগ করা, না করলে ১টি থ্রেড ব্যবহার করা
    let actual_threads = if metadata.accept_ranges { num_threads } else { 1 };
    let segments = calculate_segments(total_size, actual_threads);
    
    let client = Client::new();
    let mut tasks = Vec::new();

    // ৪. প্রতিটি সেগমেন্টের জন্য আলাদা টাস্ক (Thread) চালু করা
    for segment in segments {
        let client_clone = client.clone();
        let url_clone = url.to_string();
        let file_clone = Arc::clone(&shared_file);

        let task = tokio::spawn(async move {
            match download_segment(client_clone, url_clone, segment.clone(), file_clone).await {
                Ok(_) => println!("Segment {} downloaded successfully.", segment.id),
                Err(e) => eprintln!("Error in segment {}: {}", segment.id, e),
            }
        });
        tasks.push(task);
    }

    // ৫. সব থ্রেডের কাজ শেষ হওয়া পর্যন্ত অপেক্ষা করা
    for task in tasks {
        task.await?;
    }

    println!("Download Completed Successfully!");
    Ok(())
}
