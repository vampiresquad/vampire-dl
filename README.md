# 🦇 VampireDL
> **An Advanced, High-Performance, Cross-Platform Download Manager by VAMPIRE_SQUAD.**

VampireDL is built from scratch with a focus on extreme performance, memory safety, and multi-threaded downloading. Powered by a highly concurrent **Rust** core engine, it dynamically segments files to maximize bandwidth utilization and provides flawless resume capabilities.

## ✨ Core Features
- **🚀 Multi-Threaded Engine:** Dynamically splits files into segments for blazing-fast downloads.
- **🛡️ Fault Tolerant:** Exponential backoff and auto-retry for network drops.
- **💾 Data Persistence:** SQLite-based state management for precise pause and resume functionality.
- **🚦 Smart Queue System:** Prioritizes and manages concurrent active downloads.
- **📱 Cross-Platform:** 100% shared core business logic across Android (Kotlin) and Desktop (Tauri).

## 🏗️ System Architecture
The project follows a strict modular monorepo structure:

* **`core/`**: The pure Rust business logic. Contains the Network, Downloader, Storage, and Queue modules.
* **`platform/desktop/`**: The Tauri-based desktop application. Uses HTML/JS/CSS for a lightweight, native-feeling UI.
* **`platform/android/`**: The Android application built with Kotlin and Jetpack Compose. Communicates with the Rust core via JNI (`vampire_ffi`).

## 🛠️ Technology Stack
* **Core Engine:** Rust, Tokio (Async), Reqwest (HTTP), SQLx (SQLite)
* **Desktop UI:** Tauri, HTML5, CSS3, JavaScript
* **Android UI:** Kotlin, Jetpack Compose, JNI (Java Native Interface)

## 🚀 How to Build & Run

### 1. Run Core CLI (Testing)
```bash
cd core
cargo run --release
