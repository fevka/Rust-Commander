Rust Commander (HoN Assistant)
Rust Commander is a high-performance external game assistant for Heroes of Newerth (HoN), built with Rust and Tauri. It utilizes the native Windows OCR engine to read screen data in real-time and provides audio-visual alerts for critical game timings (Stacks, Runes, Bosses) and execution thresholds.

🚀 Features
This tool acts as a "Commander," automating the tracking of essential game mechanics:

⚔️ Execute Alert (Smart OCR)
Real-time HP Monitoring: Uses Windows Native OCR to read the health bar of the selected unit.

Threshold Warning: If the target's health drops below 350 HP (Kill threshold for abilities like Pyromancer/Witch Slayer/Legionnaire), it plays a distinct sound (execute.wav).

⏱️ Stack Timer
Jungle Stacking: Warns you at :50 seconds of every minute (e.g., 1:50, 2:50).

Purpose: Reminds you to pull neutral creeps at x:55 to stack the camp.

💎 Rune Control
Rune Spawns: Alerts you every 2 minutes, exactly 15 seconds before the spawn time (e.g., 1:45, 3:45, 5:45).

Advantage: Gives you enough time to move towards the river before the rune spawns.

👹 Boss Timers (Kongor & Phoenix)
Automatic Tracking: Calculates respawn times based on when the boss dies.

Kongor: 10 minutes (600s) respawn time.

Phoenix: 15 minutes (900s) respawn time.

Pre-Spawn Alert: Plays a warning sound 30 seconds before the boss respawns to help your team prepare.

🛠️ Tech Stack
Core: Rust (Logic, Screen Capture, Threading)

Frontend: Tauri (UI, Overlay)

OCR: windows-rs (Windows.Media.Ocr) - Uses the built-in Windows 10/11 OCR engine for zero-dependency text recognition.

Audio: rodio - For low-latency sound playback.

⚙️ Prerequisites
Since this project relies on Windows Native APIs, it requires:

OS: Windows 10 or Windows 11.

Language Pack: Ensure the English (or relevant) language pack is installed in Windows settings for the OCR engine to function correctly.

Development Tools:

Rust (rustup)

Node.js & npm (for Tauri frontend)

Build Tools for Visual Studio (C++ workload)

📦 Installation & Setup
Clone the repository:

Bash
git clone https://github.com/yourusername/rust-commander.git
cd rust-commander
Install Frontend Dependencies:

Bash
npm install
Run in Development Mode:

Bash
cargo tauri dev
Build for Production:

Bash
cargo tauri build
🎮 How to Use
Launch the Application: Open the Rust Commander executable.

Start the Game: Wait for the match to begin.

Sync the Timer:

Click the "Start Bot" button exactly when the in-game timer hits 00:00 (creep spawn).

Alternative: Press the bound hotkey (if configured) to sync.

Boss Events:

When Kongor dies, click the "Kongor Dead" button in the UI. The bot will automatically calculate the next spawn time (Current Time + 10 mins).

When Phoenix dies, click the "Phoenix Dead" button (Current Time + 15 mins).

Execute Feature:

The bot continuously scans the specific screen region defined in config.rs (default is top-left target frame). Ensure your game resolution matches or update the coordinates in the config.

# Project Name

This software is developed strictly for **educational and testing purposes**.

## ⚠️ Disclaimer / Warning

Please read this section carefully before using the software:

* **Use at Your Own Risk:** Anyone who chooses to use this software takes full responsibility for their actions.
* **No Liability:** The authors/developers are **not responsible** for any damage, bans, or legal consequences caused by the use of this tool.
* **Modifications:** We accept no liability for actions taken using either the original source code or any modified versions of it.

By using this software, you acknowledge that you are solely responsible for compliance with any relevant laws and terms of service.
