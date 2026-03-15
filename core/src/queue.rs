use crate::models::{DownloadTask, DownloadStatus};
use std::collections::VecDeque;

/// ডাউনলোড টাস্কগুলো ম্যানেজ করার জন্য Queue সিস্টেম
pub struct DownloadQueue {
    tasks: VecDeque<DownloadTask>,
    max_concurrent: usize, // একসাথে সর্বোচ্চ কয়টি ডাউনলোড চলবে
    active_count: usize,   // বর্তমানে কয়টি চলছে
}

impl DownloadQueue {
    /// নতুন Queue তৈরি করার ফাংশন
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            tasks: VecDeque::new(),
            max_concurrent,
            active_count: 0,
        }
    }

    /// নতুন ডাউনলোড টাস্ক Queue তে যুক্ত করা
    pub fn add_task(&mut self, task: DownloadTask) {
        self.tasks.push_back(task);
    }

    /// সিরিয়ালে থাকা পরবর্তী Pending টাস্ক বের করে আনা
    pub fn get_next_task(&mut self) -> Option<DownloadTask> {
        // যদি ম্যাক্সিমাম লিমিট পার হয়ে যায়, তবে নতুন টাস্ক শুরু হবে না
        if self.active_count >= self.max_concurrent {
            return None;
        }

        // লিস্টের প্রথম Pending টাস্কটি খুঁজে বের করা
        if let Some(index) = self.tasks.iter().position(|t| t.status == DownloadStatus::Pending) {
            self.active_count += 1;
            self.tasks[index].status = DownloadStatus::Downloading;
            return Some(self.tasks[index].clone());
        }

        None
    }

    /// ডাউনলোড সফল হলে টাস্কটিকে Completed মার্ক করা এবং কিউ থেকে জায়গা কমানো
    pub fn mark_completed(&mut self, task_id: &str) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.status = DownloadStatus::Completed;
            if self.active_count > 0 {
                self.active_count -= 1;
            }
        }
    }
    
    /// ডাউনলোড ফেইল হলে এরর মেসেজ সেভ করা
    pub fn mark_failed(&mut self, task_id: &str, error: String) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.status = DownloadStatus::Failed(error);
            if self.active_count > 0 {
                self.active_count -= 1;
            }
        }
    }
}
