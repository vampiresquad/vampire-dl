use std::time::Duration;
use tokio::time::sleep;
use std::future::Future;

/// যেকোনো ফেইল হওয়া টাস্ককে নির্দিষ্ট সময় পরপর পুনরায় চেষ্টা করার ফাংশন
pub async fn retry_with_backoff<F, Fut, T, E>(
    mut action: F,
    max_retries: u32,
    base_delay_ms: u64,
) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut retries = 0;
    
    loop {
        match action().await {
            Ok(result) => return Ok(result), // সফল হলে রেজাল্ট রিটার্ন করবে
            Err(e) => {
                // যদি সর্বোচ্চ চেষ্টার লিমিট পার হয়ে যায়, তবে এরর রিটার্ন করবে
                if retries >= max_retries {
                    return Err(e);
                }
                
                // Exponential Backoff: প্রতিবার ফেইল করলে অপেক্ষার সময় দ্বিগুণ হবে (1s, 2s, 4s...)
                let delay = base_delay_ms * (2_u64.pow(retries));
                eprintln!(
                    "⚠️ Action failed: {}. Retrying in {} ms (Attempt {}/{})", 
                    e, delay, retries + 1, max_retries
                );
                
                // নির্দিষ্ট সময় অপেক্ষা করা
                sleep(Duration::from_millis(delay)).await;
                retries += 1;
            }
        }
    }
}
