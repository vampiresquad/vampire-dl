document.getElementById('download-btn').addEventListener('click', async () => {
    const url = document.getElementById('url-input').value;
    const savePath = document.getElementById('path-input').value;
    const statusBox = document.getElementById('status-box');

    if (!url || !savePath) {
        statusBox.innerHTML = '<p style="color: #ffcc00;">⚠️ Please enter both URL and Save Path.</p>';
        return;
    }

    // ডাউনলোড শুরু হওয়ার মেসেজ
    statusBox.innerHTML = '<p style="color: #00bfff;">⏳ Starting multi-thread download... Please wait.</p>';

    try {
        // Tauri এর মাধ্যমে Rust Core Engine-কে কমান্ড পাঠানো
        const response = await invoke('start_new_download', {
            url: url,
            savePath: savePath,
            threads: 4 // ডিফল্টভাবে ৪টি থ্রেড ব্যবহার হবে
        });
        
        // ডাউনলোড সফল হলে সবুজ মেসেজ
        statusBox.innerHTML = `<p style="color: #00ff00;">✅ ${response}</p>`;
    } catch (error) {
        // ডাউনলোড ফেইল হলে লাল মেসেজ
        statusBox.innerHTML = `<p style="color: #ff3333;">❌ Error: ${error}</p>`;
    }
});
