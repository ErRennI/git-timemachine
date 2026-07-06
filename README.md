# ⏳ gtm (Git Time Machine)

**gtm**, built with Rust and ratatui, is a lightning-fast, interactive Terminal User Interface (TUI) for exploring Git commit history.

Forget endlessly scrolling through raw `git log -p` outputs. gtm provides a beautifully split, color-coded, and highly optimized environment to travel through your repository's history, perfectly grouping changes file-by-file.

## ✨ Key Features

- 🚀 **Blazing Fast:** Powered by `git2-rs`, it communicates directly with Git's internal C API. No subshells, no overhead.
- 🧠 **Smart Diff Caching:** Diffs are computed only when first viewed and cached in memory. Zero CPU waste when revisiting commits.
- 📂 **File-Aware Diffs:** Automatically parses diffs and groups changes under bold, colored file headers for superior readability.
- 📜 **Zero-Cost Horizontal Scrolling:** Read extremely long lines of code without text wrapping messing up your code structure.
- 🛡️ **Rock Solid:** Safely handles empty repositories, missing commits, and terminal resize events without panicking.

## 🛠️ Installation & Setup

The most reliable way to install gtm on any platform (Linux, macOS, Windows) is to clone the repository and build it directly from the source. This ensures the binary is fully optimized for your specific system architecture.

### Prerequisites

Make sure you have Rust (Cargo) and CMake installed on your system (CMake is required to compile the underlying libgit2 C library).

### 1. Clone and Build

Run the following commands in your terminal to download and compile the project in release mode:

```bash
# Clone the repository
git clone https://github.com/ErRennI/git-timemachine.git
cd git-timemachine

# Build the highly optimized release binary
cargo build --release
```

Once compilation finishes, your executable binary will be created at `target/release/gtm` (or `gtm.exe` on Windows).

### 2. Add to System PATH (Highly Recommended)

To run the `gtm` command globally from inside any Git repository without typing the full executable path, you need to move the binary to a folder that is registered in your system's PATH environment variable.

Choose the instructions for your specific operating system below:

#### 🐧 Linux & 🍏 macOS

Move the compiled binary directly into `/usr/local/bin`, which is already part of your system's global PATH:

```bash
sudo cp target/release/gtm /usr/local/bin/
```

Now you can open a new terminal anywhere and just type `gtm` to run the app!

#### 🪟 Windows (Automated PowerShell Method)

Open PowerShell as Administrator and run the following lines to create a dedicated directory, copy the executable, and permanently append it to your System PATH:

```powershell
$installDir = "$env:ProgramFiles\gtm"
New-Item -ItemType Directory -Force -Path $installDir | Out-Null
Copy-Item "target\release\gtm.exe" "$installDir\"
[Environment]::SetEnvironmentVariable("Path", "$([Environment]::GetEnvironmentVariable('Path', 'Machine'));$installDir", "Machine")
```

⚠️ **Important for Windows Users:** After running the PowerShell commands, you must completely close and restart your terminal (or VS Code) for the new PATH changes to take effect. Once restarted, simply type `gtm` in any directory.

## 🚀 Usage

Navigate to any Git repository and simply run:

```bash
gtm
```

You can also inspect a repository located in another directory without leaving your current path:

```bash
gtm /path/to/your/repo
```

## 🎮 Keybindings & Controls

The interface is divided into two contextual panels: Commit Log (Left) and Details (Right).

### 💡 Quick Navigation Guide

If you want to get started immediately, just keep these 3 basic steps in mind:

1. **Browse:** Use `Up` and `Down` arrow keys on the left side to scroll through your commit history.
2. **Focus:** Press the `Right` arrow key to switch into the Diff Panel, and `Left` arrow key to jump back to the Commit List.
3. **Scroll:** Once inside the Diff Panel, use `Up` / `Down` to scroll vertically, and hold `Ctrl + Left` / `Right` to scroll horizontally through long lines of code.

### Complete Keybindings Table

| Key | Context | Action |
|---|---|---|
| `q` | Global | Safely exits the application and restores the terminal state. |
| `Left Arrow` | Global | If focusing the Right Panel, shifts focus back to the Commit List. |
| `Right Arrow` | Global | If focusing the Left Panel, shifts focus over to the Diff Details. |
| `Up Arrow` | Left Panel | Moves to the previous (newer) commit and loads its diff. |
| `Down Arrow` | Left Panel | Moves to the next (older) commit and loads its diff. |
| `Up Arrow` | Right Panel | Scrolls the active commit's diff content upward. |
| `Down Arrow` | Right Panel | Scrolls the active commit's diff content downward. |
| `Ctrl + Right Arrow` | Right Panel | Scrolls the diff view to the right to read long lines of code. |
| `Ctrl + Left Arrow` | Right Panel | Scrolls the diff view back to the left. |

## 🎨 Interface Colors

The diff viewer uses intuitive coloring to help you review code changes at a glance:

- 👤 **Yellow:** Commit Author & Metadata
- 📁 **Magenta (Bold):** Name of the file currently being inspected.
- 🟢 **Green:** Lines added to the project in that commit (`+`).
- 🔴 **Red:** Lines removed from the project in that commit (`-`).
- 🔵 **Cyan:** Git Hunk context boundaries (`@@ -x,y +z,w @@`).
