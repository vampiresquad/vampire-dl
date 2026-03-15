use std::path::Path;
use std::time::Instant;

// আমাদের তৈরি করা লাইব্রেরি (vampire_core) থেকে ফাংশন ইম্পোর্ট করা
use vampire_core::controller::start_download;

#[tokio::main]
async fn main() {
    // টেস্ট করার জন্য একটি ডেমো ফাইলের URL (Hetzner-এর 100MB টেস্ট ফাইল)
    let file_url = "https://speed.hetzner.de/100MB.bin";
    let save_path = Path::new("vampire_test_download.bin");
    
    // ফাইলটি কয়টি অংশে ভাগ করে ডাউনলোড করতে চান (যেমন: 4 থ্রেড)
    let num_threads = 4;

    println!("🦇 Starting VampireDL Engine...");
    println!("Target URL: {}", file_url);
    println!("Concurrent Threads: {}", num_threads);

    // সময় গণনা শুরু
    let start_time = Instant::now();

    // ডাউনলোড প্রসেস কল করা
    match start_download(file_url, save_path, num_threads).await {
        Ok(_) => {
            let duration = start_time.elapsed();
            println!(
                "✅ Download finished successfully in {:.2} seconds!",
                duration.as_secs_f64()
            );
        }
        Err(e) => {
            eprintln!("❌ Download failed: {}", e);
        }
    }
}
