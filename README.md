# Areia

Create valid paths and hidden directories for your application on Windows, Linux, and MacOS.

This zero-dependency Rust crate manages hidden config and state directories across Windows, Linux, and macOS, handling platform-specific hiding logic automatically. Like the Greek nymph who hid her son in vines, Areia keeps your data safe and unseen if needed.

I want to shout out [dirs](https://crates.io/crates/dirs) and [directories](https://crates.io/crates/directories), as they inspired this crate!

All mayor Operating Systems (Windows, Linux, MacOS) are supported.

## Roadmap

- [ ] Core Functionality
    - [ ] Environment Variable Resolution
        - [ ] Linux
            - [x] Home
            - [x] Cache
            - [x] Config
            - [x] Local
                - [x] Data
                - [x] State
                - [x] Bin
            - [ ] Audio
            - [ ] Desktop
            - [ ] Document
            - [ ] Download
            - [ ] Font
            - [ ] Picture
            - [ ] Public
            - [ ] Template
            - [ ] Video
        - [ ] Windows
            - [x] Home
            - [x] Cache
            - [x] Config
            - [x] Local
                - [x] Data
                - [x] State
                - [x] Bin
            - [ ] Audio
            - [ ] Desktop
            - [ ] Document
            - [ ] Download
            - [ ] Font
            - [ ] Picture
            - [ ] Public
            - [ ] Template
            - [ ] Video
        - [ ] MacOS
            - [x] Home
            - [x] Cache
            - [x] Config
            - [x] Local
                - [x] Data
                - [x] State
                - [x] Bin
            - [ ] Audio
            - [ ] Desktop
            - [ ] Document
            - [ ] Download
            - [ ] Font
            - [ ] Picture
            - [ ] Public
            - [ ] Template
            - [ ] Video
    - [ ] Platform-Specific "Hide" Implementation
        - [ ] Windows
        - [ ] Linux
        - [ ] MacOS
- [ ] Stable Features
    - [ ] Atomic "Hide-and-Move"
    - [ ] The "Double-Search" Discovery
    - [ ] Ownership & Permissions (Unix-only)
- [ ] Nice-to-Haves
    - [ ] MacOS Hybrid Support
    - [ ] Windows "System" Flag
    - [ ] Redaction/Privacy Helpers
    - [ ] Directory "Auto-Creator"
    - [ ] Directory "Auto-Deletor"
    - [ ] Path "Normalizer"
- [ ] Support system level directories (maybe - after 1.0.0)
    - [ ] Windows
        - [ ] ProgramFiles
    - [ ] Unix
        - [ ] Root Bin
        - [ ] Root Data
        - [ ] Root Config

## Project Design (Can be deleted after 1.0.0)

<details>
<summary>The tables below are taken from the [directories](https://crates.io/crates/directories) crate. Click for the full license.</summary>

Copyright (c) 2018 directories-rs contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

</details>

### `BaseDirs`

`BaseDirs` query the paths of user-invisible standard directories
that have been defined according to the conventions of the operating system the library is running on.


| Function name      | Value on Linux                                           | Value on Windows            | Value on macOS                      |
|--------------------|----------------------------------------------------------| --------------------------- | ----------------------------------- |
| `home_dir`         | `$HOME`                                                  | `{FOLDERID_Profile}`        | `$HOME`                             |
| `cache_dir`        | `$XDG_CACHE_HOME`        or `$HOME`/.cache               | `{FOLDERID_LocalAppData}`   | `$HOME`/Library/Caches              |
| `config_dir`       | `$XDG_CONFIG_HOME`       or `$HOME`/.config              | `{FOLDERID_RoamingAppData}` | `$HOME`/Library/Application Support |
| `config_local_dir` | `$XDG_CONFIG_HOME`       or `$HOME`/.config              | `{FOLDERID_LocalAppData}`   | `$HOME`/Library/Application Support |
| `data_dir`         | `$XDG_DATA_HOME`         or `$HOME`/.local/share         | `{FOLDERID_RoamingAppData}` | `$HOME`/Library/Application Support |
| `data_local_dir`   | `$XDG_DATA_HOME`         or `$HOME`/.local/share         | `{FOLDERID_LocalAppData}`   | `$HOME`/Library/Application Support |
| `executable_dir`   | `Some($XDG_BIN_HOME)`    or `Some($HOME`/.local/bin`)`   | `None`                      | `None`                              |
| `preference_dir`   | `$XDG_CONFIG_HOME`       or `$HOME`/.config              | `{FOLDERID_RoamingAppData}` | `$HOME`/Library/Preferences         |
| `runtime_dir`      | `Some($XDG_RUNTIME_DIR)` or `None`                       | `None`                      | `None`                              |
| `state_dir`        | `Some($XDG_STATE_HOME)`  or `Some($HOME`/.local/state`)` | `None`                      | `None`                              |

### `UserDirs`

The intended use case for `UserDirs` is to query the paths of user-facing standard directories
that have been defined according to the conventions of the operating system the library is running on.

| Function name    | Value on Linux                                                         | Value on Windows                 | Value on macOS                 |
| ---------------- | ---------------------------------------------------------------------- | -------------------------------- | ------------------------------ |
| `home_dir`       | `$HOME`                                                                | `{FOLDERID_Profile}`             | `$HOME`                        |
| `audio_dir`      | `Some(XDG_MUSIC_DIR)`           or `None`                              | `Some({FOLDERID_Music})`         | `Some($HOME`/Music/`)`         |
| `desktop_dir`    | `Some(XDG_DESKTOP_DIR)`         or `None`                              | `Some({FOLDERID_Desktop})`       | `Some($HOME`/Desktop/`)`       |
| `document_dir`   | `Some(XDG_DOCUMENTS_DIR)`       or `None`                              | `Some({FOLDERID_Documents})`     | `Some($HOME`/Documents/`)`     |
| `download_dir`   | `Some(XDG_DOWNLOAD_DIR)`        or `None`                              | `Some({FOLDERID_Downloads})`     | `Some($HOME`/Downloads/`)`     |
| `font_dir`       | `Some($XDG_DATA_HOME`/fonts/`)` or `Some($HOME`/.local/share/fonts/`)` | `None`                           | `Some($HOME`/Library/Fonts/`)` |
| `picture_dir`    | `Some(XDG_PICTURES_DIR)`        or `None`                              | `Some({FOLDERID_Pictures})`      | `Some($HOME`/Pictures/`)`      |
| `public_dir`     | `Some(XDG_PUBLICSHARE_DIR)`     or `None`                              | `Some({FOLDERID_Public})`        | `Some($HOME`/Public/`)`        |
| `template_dir`   | `Some(XDG_TEMPLATES_DIR)`       or `None`                              | `Some({FOLDERID_Templates})`     | `None`                         | 
| `video_dir`      | `Some(XDG_VIDEOS_DIR)`          or `None`                              | `Some({FOLDERID_Videos})`        | `Some($HOME`/Movies/`)`        |

### 1. The "Must-Haves" (Core Functionality)

These are the foundational pieces required to make the crate functional across Windows, Linux, and macOS.

- Internal Path Syncing:
    - An API that returns a PathBuf after a "hide" operation, forcing the developer to update their stored state (since the path changes on Unix).
- Environment Variable Resolution:
    - Implementing a fallback chain for paths (e.g., Check $XDG_CONFIG_HOME, if empty, check $HOME and append /.config).
    - Mechanism: Use std::env::var.
- Platform-Specific "Hide" Implementation:
    - Windows: FFI for SetFileAttributesW (Metadata approach).
    - Unix/macOS: Filename manipulation (Dot-prefix approach).
    - Ensure that only the root of the path is hidden.

### 2. The "Stable Architecture" Features

These features move the crate from a "script" to a "library" by handling edge cases and providing safety.

- Atomic "Hide-and-Move":
    - A function that handles moving an existing visible directory to a hidden path while ensuring data isn't lost if the process is interrupted.
- The "Double-Search" Discovery:
    - A method that looks for both the visible and hidden versions of a file/dir and returns the one that contains data (preferring the hidden one).
- Ownership & Permissions (Unix-only):
    - When creating a config dir, it should use std::os::unix::fs::PermissionsExt to set the mode to 0o700 (private). This ensures that while the file is hidden from the UI, it is also protected from other users on the system.

### 3. The "Nice-to-Haves" (Professional Polish)

These features make the crate feel like a high-quality, "native" experience for the end user.

- macOS Hybrid Support:
    - Adding an FFI call for chflags to set UF_HIDDEN. This makes the dotfile even more hidden on macOS (it won't show up in certain GUI search tools).
- Windows "System" Flag:
    - Optionally adding FILE_ATTRIBUTE_SYSTEM (0x4) along with the Hidden flag. This makes the file/folder even harder to see in Windows Explorer (requires unchecking "Hide protected operating system files").
- Redaction/Privacy Helpers:
    - A utility that takes a Path and returns a "Safe for Logs" string (e.g., converting /home/user/.myapp to ~/.myapp) to prevent users from accidentally leaking their username in log files.
- Directory "Auto-Creator":
    - A function that runs fs::create_dir_all on a given path.
    - Also inverse "Auto-Deleter"
    - A convenience function - just pass in whatever path areia constructs.
- Path "Normalizer":
    - A utility to ensure that if a user provides "My App Name", it is converted to a filesystem-safe format (e.g., my_app_name or my-app-name) to avoid issues with spaces or special characters in CLI environments.
    - This needs to be bundled into any `.new()` functions and can be publicly exposed for convenience.

### List of needed FFI Functions:

To keep it zero-dependency, I need to write these:

- Windows: GetFileAttributesW, SetFileAttributesW.
- Unix (Optional): chmod (if you want to bypass std::fs for more control).
- macOS (Optional): chflags.
    
### Possible Project Suructure

```
areia/
├── Cargo.toml
└── src/
    ├── lib.rs            // The public API (AppDir, Config locations)
    ├── error.rs          // Custom error types (optional but good practice)
    ├── platform/         // The engine room
    │   ├── mod.rs        // Dispatches to unix.rs or windows.rs based on cfg
    │   ├── unix.rs       // Dotfile logic + File Permissions (0o700)
    │   ├── macos.rs      // (Optional) Specific tweaks if you go beyond unix defaults
    │   └── windows.rs    // FFI calls to kernel32.dll + Attribute logic
    └── utils/            // Helpers (Path sanitization, env var fallbacks)
        ├── mod.rs
        ├── unix.rs       // Unix-specific helpers and fallbacks
        ├── macos.rs      // macOS-specific helpers and fallbacks
        └── windows.rs    // Windows-specific helpers and fallbacks
```

