use std::path::Path;
use std::io::SeekFrom;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncSeekExt, AsyncWriteExt};
use std::error::Error;

/// ডাউনলোডের জন্য নতুন ফাইল তৈরি এবং জায়গা নির্ধারণ (Pre-allocate) করার ফাংশন
pub async fn create_file(path: &Path, total_size: u64) -> Result<File, Box<dyn Error>> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .await?;

    // ফাইলের সাইজ আগে থেকেই ঠিক করে নেওয়া (যেন স্টোরেজ ফুল হয়ে ক্র্যাশ না করে)
    if total_size > 0 {
        file.set_len(total_size).await?;
    }

    Ok(file)
}

/// ফাইলের নির্দিষ্ট জায়গায় (offset) ডাউনলোড হওয়া ডেটা সেভ করার ফাংশন
pub async fn write_chunk(
    file: &mut File,
    offset: u64,
    data: &[u8],
) -> Result<(), Box<dyn Error>> {
    // ফাইলের ঠিক যেই জায়গায় ডেটা বসবে, সেখানে পয়েন্টার নিয়ে যাওয়া
    file.seek(SeekFrom::Start(offset)).await?;
    
    // ডেটা রাইট করা (সেভ করা)
    file.write_all(data).await?;
    
    Ok(())
}
