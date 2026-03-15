package com.vampiresquad.vampiredl

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

class MainActivity : ComponentActivity() {
    
    // Rust-এর তৈরি করা C লাইব্রেরি (vampire_ffi) মেমোরিতে লোড করা হচ্ছে
    init {
        System.loadLibrary("vampire_ffi")
    }

    // এটি সেই ফাংশন যা সরাসরি Rust-এর Core Engine কে কল করবে!
    external fun startDownload(url: String, savePath: String, threads: Int): String

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            MaterialTheme(colorScheme = darkColorScheme()) {
                DownloadScreen()
            }
        }
    }
}

@Composable
fun DownloadScreen() {
    var url by remember { mutableStateOf("") }
    var status by remember { mutableStateOf("Ready for action...") }

    Column(modifier = Modifier.padding(16.dp).fillMaxSize()) {
        Text(text = "🦇 VampireDL Mobile", style = MaterialTheme.typography.headlineMedium, color = MaterialTheme.colorScheme.error)
        Spacer(modifier = Modifier.height(8.dp))
        Text(text = "Advanced High-Performance Downloader", style = MaterialTheme.typography.bodySmall)
        
        Spacer(modifier = Modifier.height(24.dp))
        
        OutlinedTextField(
            value = url,
            onValueChange = { url = it },
            label = { Text("Download URL") },
            modifier = Modifier.fillMaxWidth()
        )
        
        Spacer(modifier = Modifier.height(16.dp))
        
        Button(
            onClick = { 
                status = "⏳ Downloading via Core Engine..." 
                // পরবর্তীতে এখানে startDownload(...) কল করা হবে
            },
            modifier = Modifier.fillMaxWidth(),
            colors = ButtonDefaults.buttonColors(containerColor = MaterialTheme.colorScheme.error)
        ) {
            Text("Start Download")
        }
        
        Spacer(modifier = Modifier.height(24.dp))
        
        Surface(color = MaterialTheme.colorScheme.surfaceVariant, modifier = Modifier.fillMaxWidth().padding(8.dp)) {
            Text(text = status, modifier = Modifier.padding(16.dp))
        }
    }
}
