# 📩 Android SMS Logger (Rust + Kotlin)

An Android application that **automatically records all incoming SMS messages into a local `.txt` file**, using a **native Rust library** for fast and safe file handling.

The app works **entirely in the background** thanks to an Android `BroadcastReceiver`.  
Once permissions are granted, **no UI interaction is required**.

---

## ✨ Features

- 📥 Capture all incoming SMS messages
- 🦀 Native Rust backend (via JNI)
- 📝 Append messages to a `sms.txt` file
- ⚙️ Works in background (no need to open the app)
- 🚫 No root required
- 📱 Compatible with Android 8+

---

## 🧠 How It Works

