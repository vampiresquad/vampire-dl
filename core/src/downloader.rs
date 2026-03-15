use crate::models::{DownloadStatus, Segment};

/// ফাইলের মোট সাইজকে নির্দিষ্ট সংখ্যক থ্রেড বা সেগমেন্টে ভাগ করার ফাংশন
pub fn calculate_segments(total_size: u64, num_threads: u64) -> Vec<Segment> {
    let mut segments = Vec::new();

    // সাইজ বা থ্রেড সংখ্যা শূন্য হলে খালি লিস্ট রিটার্ন করবে
    if total_size == 0 || num_threads == 0 {
        return segments;
    }

    let segment_size = total_size / num_threads;
    let mut start_byte = 0;

    for i in 0..num_threads {
        let end_byte = if i == num_threads - 1 {
            // শেষ সেগমেন্টের ক্ষেত্রে অবশিষ্ট সব বাইট দিয়ে দেওয়া হবে
            total_size - 1
        } else {
            start_byte + segment_size - 1
        };

        segments.push(Segment {
            id: i,
            start_byte,
            end_byte,
            downloaded_bytes: 0,
            status: DownloadStatus::Pending,
        });

        // পরবর্তী সেগমেন্টের শুরুর পয়েন্ট সেট করা
        start_byte = end_byte + 1;
    }

    segments
}
