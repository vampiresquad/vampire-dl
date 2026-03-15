use std::path::PathBuf;

/// ডাউনলোডের বর্তমান অবস্থা
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Paused,
    Merging,
    Completed,
    Failed(String),
}

/// প্রতিটি সেগমেন্ট বা চাঙ্কের (chunk) তথ্য
#[derive(Debug, Clone)]
pub struct Segment {
    pub id: u64,
    pub start_byte: u64,
    pub end_byte: u64,
    pub downloaded_bytes: u64,
    pub status: DownloadStatus,
}

/// মূল ডাউনলোড টাস্কের সম্পূর্ণ তথ্য
#[derive(Debug, Clone)]
pub struct DownloadTask {
    pub id: String,
    pub url: String,
    pub file_name: String,
    pub save_path: PathBuf,
    pub total_size: u64,
    pub downloaded_size: u64,
    pub segments: Vec<Segment>,
    pub status: DownloadStatus,
    pub created_at: u64,
}

impl DownloadTask {
    /// নতুন টাস্ক তৈরি করার কনস্ট্রাক্টর
    pub fn new(id: String, url: String, file_name: String, save_path: PathBuf) -> Self {
        Self {
            id,
            url,
            file_name,
            save_path,
            total_size: 0,
            downloaded_size: 0,
            segments: Vec::new(),
            status: DownloadStatus::Pending,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
}
