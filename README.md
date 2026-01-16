# Areia

Create valid paths and hidden directories for your application on Windows, Linux, and MacOS.

This zero-dependency Rust crate manages hidden config and state directories across Windows, Linux, and macOS, handling platform-specific hiding logic automatically. Like the Greek nymph who hid her son in vines, Areia keeps your data safe and unseen if needed.

I want to shout out [dirs](https://crates.io/crates/dirs) and [directories](https://crates.io/crates/directories), as they inspired this crate!

All mayor Operating Systems (Windows, Linux, MacOS) are supported.

## Roadmap

- [x] Core Functionality
    - [x] Environment Variable Resolution
        - [x] Linux
            - [x] Home
            - [x] Cache
            - [x] Config
            - [x] Local
                - [x] Data
                - [x] State
                - [x] Bin
            - [x] Audio
            - [x] Desktop
            - [x] Document
            - [x] Download
            - [x] Font
            - [x] Picture
            - [x] Public
            - [x] Template
            - [x] Video
        - [x] Windows
            - [x] Home
            - [x] Cache
            - [x] Config
            - [x] Local
                - [x] Data
                - [x] State
                - [x] Bin
            - [x] Audio
            - [x] Desktop
            - [x] Document
            - [x] Download
            - [x] Font
            - [x] Picture
            - [x] Public
            - [x] Template
            - [x] Video
        - [x] MacOS
            - [x] Home
            - [x] Cache
            - [x] Config
            - [x] Local
                - [x] Data
                - [x] State
                - [x] Bin
            - [x] Audio
            - [x] Desktop
            - [x] Document
            - [x] Download
            - [x] Font
            - [x] Picture
            - [x] Public
            - [x] Template
            - [x] Video
    - [x] Platform-Specific "Hide" Implementation
        - [x] Windows
        - [x] Linux
        - [x] MacOS
- [ ] Stable Features
    - [x] Basic Hiding
    - [x] Atomic "Hide-and-Move"
    - [ ] Super Hiding
        - [x] MacOS Hybrid Support
        - [x] Windows "System" Flag
- [ ] Nice-to-Haves
    - [ ] Directory "Auto-Creator"
    - [ ] Directory "Auto-Deletor"
- [ ] after 1.0.0
    - [ ] `dotfile` path maker (No automatic file creation)
    - [ ] Support system level directories (maybe)
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

### 2. The "Stable Architecture" Features

- Atomic "Hide-and-Move":
    - A function that handles moving an existing visible directory to a hidden path while ensuring data isn't lost if the process is interrupted.
- Super Hiding:
    - macOS Hybrid Support:
        - Adding an FFI call for chflags to set UF_HIDDEN. This makes the dotfile even more hidden on macOS (it won't show up in certain GUI search tools).
    - Windows "System" Flag:
        - Optionally adding FILE_ATTRIBUTE_SYSTEM (0x4) along with the Hidden flag. This makes the file/folder even harder to see in Windows Explorer (requires unchecking "Hide protected operating system files").

### 3. The "Nice-to-Haves" (Professional Polish)

- Directory "Auto-Creator":
    - A function that runs fs::create_dir_all on a given path.
    - Also inverse "Auto-Deleter"
    - A convenience function - just pass in whatever path areia constructs.
    - Also creates the file if it doesn't exist.
- Directory "Auto-Deletor":
    - A function that runs fs::remove_dir_all on a given path.
    - Deletes all files inside the directories if any exists.

