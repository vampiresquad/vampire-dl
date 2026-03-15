use reqwest::Client;
use std::error::Error;

/// সার্ভার থেকে পাওয়া ফাইলের প্রাথমিক তথ্য
#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub final_url: String,
    pub content_length: u64,
    pub accept_ranges: bool,
}

/// সার্ভার থেকে ফাইলের মেটাডেটা (সাইজ এবং মাল্টি-থ্রেড সাপোর্ট) নিয়ে আসার ফাংশন
pub async fn fetch_metadata(url: &str) -> Result<FileMetadata, Box<dyn Error>> {
    let client = Client::new();
    
    // সার্ভারে HEAD রিকোয়েস্ট পাঠানো (পুরো ফাইল ডাউনলোড না করে শুধু তথ্য জানার জন্য)
    let response = client.head(url).send().await?;
    
    // রিডাইরেক্ট (Redirect) হওয়ার পর ফাইনাল URL
    let final_url = response.url().to_string();
    
    // ফাইলের সাইজ বের করা
    let content_length = response
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|val| val.to_str().ok())
        .and_then(|val| val.parse::<u64>().ok())
        .unwrap_or(0);
        
    // সার্ভার মাল্টি-থ্রেড (Accept-Ranges) সাপোর্ট করে কিনা চেক করা
    let accept_ranges = response
        .headers()
        .get(reqwest::header::ACCEPT_RANGES)
        .map(|val| val == "bytes")
        .unwrap_or(false);

    Ok(FileMetadata {
        final_url,
        content_length,
        accept_ranges,
    })
}
