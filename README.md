# areia
Protect and conceal your application data with areia. This zero-dependency Rust crate manages hidden config and state directories across Windows, Linux, and macOS, handling platform-specific hiding logic (metadata vs. dotfiles) automatically. Like the nymph who hid her son in vines, areia keeps your data safe and unseen.

## Roadmap

- [] Core Functionality
    - [] Environment Variable Resolution
        - [] Unix
            - [] User Space
                - [] Home
                    - [] Prio 2: Fallback: $HOME
                - [] Cache
                    - [] Prio 1: Env Var: XDG_CACHE_HOME
                    - [] Prio 2: Fallback: $HOME/.cache
                - [] Config
                    - [] Prio 1: Env Var: XDG_CONFIG_HOME
                    - [] Prio 2: Fallback: $HOME/.config
                - [] Local
                    - [] Prio2: Fallback: $HOME/.local
                    - [] Data
                        - [] Prio1: Env Var: XDG_DATA_HOME
                        - [] Prio2: Fallback: $HOME/.local/share
                    - [] State
                        - [] Prio1: Env Var: XDG_STATE_HOME
                        - [] Prio2: Fallback: $HOME/.local/state
                    - [] Bin
                        - [] Prio2: Fallback: $HOME/.local/bin
            - [] System Level
                - [] Root
                    - [] Bin
                    - [] SBin (Optional, think about it - root bin only)
                    - [] Etc
                    - [] Lib
                    - [] Dev
                    - [] Var
                    - [] Tmp
                    - [] Home (All Users more specifically)
                    - [] Usr
                    - [] Opt


## Project Design (Can be deleted after 1.0.0)

### 1. The "Must-Haves" (Core Functionality)

These are the foundational pieces required to make the crate functional across Windows, Linux, and macOS.

- Environment Variable Resolution:
    - Implementing a fallback chain for paths (e.g., Check $XDG_CONFIG_HOME, if empty, check $HOME and append /.config).
    - Mechanism: Use std::env::var.
- Platform-Specific "Hide" Implementation:
    - Windows: FFI for SetFileAttributesW (Metadata approach).
    - Unix/macOS: Filename manipulation (Dot-prefix approach).
- Directory "Auto-Creator":
    - A function that not only runs fs::create_dir_all but ensures the root of the new path is hidden if the OS requires it.
- Path "Normalizer":
    - A utility to ensure that if a user provides "My App Name", it is converted to a filesystem-safe format (e.g., my_app_name or my-app-name) to avoid issues with spaces or special characters in CLI environments.

### 2. The "Stable Architecture" Features

These features move the crate from a "script" to a "library" by handling edge cases and providing safety.

- Atomic "Hide-and-Move":
    - A function that handles moving an existing visible directory to a hidden path while ensuring data isn't lost if the process is interrupted.
- The "Double-Search" Discovery:
    - A method that looks for both the visible and hidden versions of a file/dir and returns the one that contains data (preferring the hidden one).
- Ownership & Permissions (Unix-only):
    - When creating a config dir, it should use std::os::unix::fs::PermissionsExt to set the mode to 0o700 (private). This ensures that while the file is hidden from the UI, it is also protected from other users on the system.
- Internal Path Syncing:
    - An API that returns a PathBuf after a "hide" operation, forcing the developer to update their stored state (since the path changes on Unix).

### 3. The "Nice-to-Haves" (Professional Polish)

These features make the crate feel like a high-quality, "native" experience for the end user.

- macOS Hybrid Support:
    - Adding an FFI call for chflags to set UF_HIDDEN. This makes the dotfile even more hidden on macOS (it won't show up in certain GUI search tools).
- Windows "System" Flag:
    - Optionally adding FILE_ATTRIBUTE_SYSTEM (0x4) along with the Hidden flag. This makes the file/folder even harder to see in Windows Explorer (requires unchecking "Hide protected operating system files").
- Redaction/Privacy Helpers:
    - A utility that takes a Path and returns a "Safe for Logs" string (e.g., converting /home/user/.myapp to ~/.myapp) to prevent users from accidentally leaking their username in log files.
- Lockfile Support:
    - A zero-dependency implementation of a hidden .lock file to prevent two instances of the "MyApp" from writing to the same config directory simultaneously.

### List of needed FFI Functions:

To keep it zero-dependency, I need to write these:

- Windows: GetFileAttributesW, SetFileAttributesW.
- Unix (Optional): chmod (if you want to bypass std::fs for more control).
- macOS (Optional): chflags.
    
### Possible Project Suructure

```
hidden_app_dirs/
├── Cargo.toml
└── src/
    ├── lib.rs            // The public API (AppDir, Config locations)
    ├── error.rs          // Custom error types (optional but good practice)
    ├── platform/         // The engine room
    │   ├── mod.rs        // Dispatches to unix.rs or windows.rs based on cfg
    │   ├── unix.rs       // Dotfile logic + File Permissions (0o700)
    │   ├── windows.rs    // FFI calls to kernel32.dll + Attribute logic
    │   └── macos.rs      // (Optional) Specific tweaks if you go beyond unix defaults
    └── utils.rs          // Helpers (Path sanitization, env var fallbacks)
```
