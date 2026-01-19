# Areia

Create valid paths and hidden directories for your application on Windows, Linux, and macOS.

This zero-dependency Rust crate manages hidden config and state directories across Windows, Linux, and macOS, handling platform-specific hiding logic automatically. Like the Greek nymph who hid her son in vines, Areia keeps your data safe and unseen if needed.

I want to shout out [dirs](https://crates.io/crates/dirs) and [directories](https://crates.io/crates/directories), as they inspired this crate!

## Important notes

All major Operating Systems (Windows, Linux, macOS) are supported.

The crate is tested on Linux. It builds on Windows and macOS (on GitHub Actions) but I lack the capability to test them.

## Features

- Get paths to standard directories (e.g. `home`, `cache`, etc.)
- Create or unhide hidden paths (Unix only)
- Create or unhide hidden files / directories
- Create or unhide super hidden files / directories
- Zero Dependencies
- Auto-Creator and Auto-Deletor for nested directories

## Motivation

The main reason for writing this, was as always, the zero external Dependencies goal. Along with that, I also really wanted to play around with supporting different operating system architectures again.\
During designing and scoping the project, I also set my sights on implementing some FFI and creating my own trait for the first time.

## Usage

### Add to Cargo.toml

```toml
[dependencies]
areia = { git = "https://github.com/xqhare/areia" }
```

### BaseDirs

```rust
use areia::BaseDirs;

if let Ok(base_dirs) = BaseDirs::new() {
    assert!(base_dirs.home_dir().is_dir());
    assert!(base_dirs.cache_dir().is_dir());
    assert!(base_dirs.config_dir().is_dir());
    assert!(base_dirs.config_local_dir().is_dir());
    assert!(base_dirs.data_dir().is_dir());
    assert!(base_dirs.data_local_dir().is_dir());
    if let Some(executable_dir) = base_dirs.executable_dir() {
        assert!(executable_dir.is_dir());
    }
    assert!(base_dirs.preference_dir().is_dir());
    if let Some(runtime_dir) = base_dirs.runtime_dir() {
        assert!(runtime_dir.is_dir());
    }
    if let Some(state_dir) = base_dirs.state_dir() {
        assert!(state_dir.is_dir());
    }
    
} else {
    println!("Failed to get base dirs");
}
```

### UserDirs

```rust
use areia::UserDirs;

if let Ok(user_dirs) = UserDirs::new() {
    assert!(user_dirs.home_dir().is_dir());
    if let Some(audio_dir) = user_dirs.audio_dir() {
        assert!(audio_dir.is_dir());
    }
    if let Some(desktop_dir) = user_dirs.desktop_dir() {
        assert!(desktop_dir.is_dir());
    }
    if let Some(document_dir) = user_dirs.document_dir() {
        assert!(document_dir.is_dir());
    }
    if let Some(download_dir) = user_dirs.download_dir() {
        assert!(download_dir.is_dir());
    }
    if let Some(font_dir) = user_dirs.font_dir() {
        assert!(font_dir.is_dir());
    }
    if let Some(picture_dir) = user_dirs.picture_dir() {
        assert!(picture_dir.is_dir());
    }
    if let Some(public_dir) = user_dirs.public_dir() {
        assert!(public_dir.is_dir());
    }
    if let Some(template_dir) = user_dirs.template_dir() {
        assert!(template_dir.is_dir());
    }
    if let Some(video_dir) = user_dirs.video_dir() {
        assert!(video_dir.is_dir());
    }
}
```

### Hidden

If the path doesn't exist:

```rust
use areia::Hidden;
use std::path::PathBuf;

let mut path = PathBuf::from("to_hide/some.file");
let hidden_path = path.hide();
if cfg!(target_os = "windows") {
    assert!(hidden_path.is_err());
} else {
    assert!(hidden_path.is_ok());
    assert!(hidden_path.as_ref().unwrap().exists());
    assert!(hidden_path.as_ref().unwrap().is_file());
    assert!(hidden_path.as_ref().unwrap().is_hidden().unwrap());

    let unhidden_path = hidden_path.as_ref().unwrap().clone().unhide();
    assert!(unhidden_path.is_ok());
    assert!(unhidden_path.as_ref().unwrap().exists() && !hidden_path.as_ref().unwrap().exists());
    assert!(unhidden_path.as_ref().unwrap().is_file());
    assert!(!unhidden_path.as_ref().unwrap().is_hidden().unwrap());
    assert_eq!(unhidden_path.as_ref().unwrap(), &path);
    // Cleanup created files
    assert!(std::fs::remove_file(unhidden_path.as_ref().unwrap()).is_ok());
    assert!(std::fs::remove_dir_all(unhidden_path.as_ref().unwrap().parent().unwrap()).is_ok());
}
```

Or if the path already exists:

```rust
use areia::Hidden;
use std::path::PathBuf;

let mut path = PathBuf::from("real_dir/existing.file");
assert!(std::fs::create_dir_all(&path.parent().unwrap()).is_ok());
assert!(std::fs::File::create(&path).is_ok());

let mut hidden_path = path.hide().unwrap();
assert!(hidden_path.exists());
assert!(hidden_path.is_file());
assert!(hidden_path.is_hidden().unwrap());

let unhidden_path = hidden_path.unhide().unwrap();
assert!(unhidden_path.exists() && !hidden_path.exists());
assert!(unhidden_path.is_file());
assert!(!unhidden_path.is_hidden().unwrap());
assert_eq!(unhidden_path, path);

// cleanup created files
assert!(std::fs::remove_file(&unhidden_path).is_ok());
assert!(std::fs::remove_dir_all(&unhidden_path.parent().unwrap()).is_ok());
```

Or if the path is inside a hidden system directory:

```rust
use areia::{BaseDirs, Hidden};
use std::path::PathBuf;

let base_dirs = BaseDirs::new().unwrap();
let cache_dir = base_dirs.cache_dir();

let mut path = cache_dir.join("hidden_dir/hidden.file");
let hidden_path = path.hide();
assert!(hidden_path.is_ok());
assert_eq!(hidden_path.as_ref().unwrap(), &path);
assert!(hidden_path.as_ref().unwrap().exists());
assert!(hidden_path.as_ref().unwrap().is_file());
assert!(hidden_path.as_ref().unwrap().is_hidden().unwrap());

let unhidden_path = hidden_path.as_ref().unwrap().clone().unhide();
// Can't unhide a file inside a hidden system directory
assert!(unhidden_path.is_err());

// Cleanup created files
assert!(std::fs::remove_file(hidden_path.as_ref().unwrap()).is_ok());
assert!(std::fs::remove_dir(hidden_path.as_ref().unwrap().parent().unwrap()).is_ok());
```

#### `try_into`

> This concept does not apply on Windows.

##### `try_into_hidden_path`

A convenience function provided by the `Hidden` trait. \
It has the same behavior as `hide` but does not create any files or directories. \
It creates a new path with the last component hidden and returns it.

```rust
if cfg!(not(target_os = "windows")) {
    use areia::Hidden;
    use std::path::PathBuf;

    let mut path = PathBuf::from("non_existing/some.file");
    let hidden_path = path.try_into_hidden_path();
    assert!(hidden_path.is_ok());
    assert_eq!(hidden_path.as_ref().unwrap(), &PathBuf::from("non_existing/.some.file"));
    assert!(!hidden_path.as_ref().unwrap().exists());
    assert!(hidden_path.as_ref().unwrap().is_hidden().unwrap());

    let mut path2 = PathBuf::from("a_dir/");
    let hidden_path2 = path2.try_into_hidden_path();
    assert!(hidden_path2.is_ok());
    assert_eq!(hidden_path2.as_ref().unwrap(), &PathBuf::from(".a_dir/"));
    assert!(!hidden_path2.as_ref().unwrap().exists());
    assert!(hidden_path2.as_ref().unwrap().is_hidden().unwrap());
}
```

##### `try_into_unhidden_path`

A convenience function provided by the `Hidden` trait. \
It has the same behavior as `unhide` but does not create any files or directories. \
It creates a new path with the last component unhidden and returns it.

```rust
if cfg!(not(target_os = "windows")) {
    use areia::Hidden;
    use std::path::PathBuf;

    let path = PathBuf::from("non_existing/.some.file");
    let unhidden_path = path.try_into_unhidden_path();
    assert!(unhidden_path.is_ok());
    assert_eq!(unhidden_path.as_ref().unwrap(), &PathBuf::from("non_existing/some.file"));
    assert!(!unhidden_path.as_ref().unwrap().exists());
    assert!(!unhidden_path.as_ref().unwrap().is_hidden().unwrap());

    let path2 = PathBuf::from(".a_dir/");
    let unhidden_path2 = path2.try_into_unhidden_path();
    assert!(unhidden_path2.is_ok());
    assert_eq!(unhidden_path2.as_ref().unwrap(), &PathBuf::from("a_dir/"));
    assert!(!unhidden_path2.as_ref().unwrap().exists());
    assert!(!unhidden_path2.as_ref().unwrap().is_hidden().unwrap());
}
```

### Super Hidden

> Super Hidden is supported on macOS and Windows - The concept doesn't apply on Linux.\
> See the `SuperHidden` documentation for more details.

If the path doesn't exist:

```rust
if cfg!(not(target_os = "linux")) {
    use areia::SuperHidden;
    use std::path::PathBuf;

    let mut path = PathBuf::from("super_hide/some.file");
    let super_hidden_path = path.super_hide();
    assert!(super_hidden_path.is_ok());
    assert!(super_hidden_path.as_ref().unwrap().exists());
    assert!(super_hidden_path.as_ref().unwrap().is_file());
    assert!(super_hidden_path.as_ref().unwrap().is_super_hidden().unwrap());

    let unhidden_path = super_hidden_path.as_ref().unwrap().clone().super_unhide();
    assert!(unhidden_path.is_ok());
    assert!(unhidden_path.as_ref().unwrap().exists() && !super_hidden_path.as_ref().unwrap().exists());
    assert!(unhidden_path.as_ref().unwrap().is_file());
    assert!(!unhidden_path.as_ref().unwrap().is_super_hidden().unwrap());
    assert_eq!(unhidden_path.as_ref().unwrap(), &path);
    // Cleanup created files
    assert!(std::fs::remove_file(unhidden_path.as_ref().unwrap()).is_ok());
    assert!(std::fs::remove_dir_all(unhidden_path.as_ref().unwrap().parent().unwrap()).is_ok());
}
```

Or if the path already exists:

```rust
if cfg!(not(target_os = "linux")) {
    use areia::SuperHidden;
    use std::path::PathBuf;

    let mut path = PathBuf::from("any_dir/existing.file");
    assert!(std::fs::create_dir_all(&path.parent().unwrap()).is_ok());
    assert!(std::fs::File::create(&path).is_ok());

    let mut super_hidden_path = path.super_hide().unwrap();
    assert!(super_hidden_path.exists());
    assert!(super_hidden_path.is_file());
    assert!(super_hidden_path.is_super_hidden().unwrap());

    let unhidden_path = super_hidden_path.super_unhide().unwrap();
    assert!(unhidden_path.exists() && !super_hidden_path.exists());
    assert!(unhidden_path.is_file());
    assert!(!unhidden_path.is_super_hidden().unwrap());
    assert_eq!(unhidden_path, path);

    // cleanup created files
    assert!(std::fs::remove_file(&unhidden_path).is_ok());
    assert!(std::fs::remove_dir_all(&unhidden_path.parent().unwrap()).is_ok());
}
```

Or if the path is inside a hidden system directory:

```rust
if cfg!(not(target_os = "linux")) {
    use areia::{BaseDirs, SuperHidden};
    use std::path::PathBuf;

    let base_dirs = BaseDirs::new().unwrap();
    let cache_dir = base_dirs.cache_dir();

    let mut path = cache_dir.join("hidden_dir/hidden.file");
    let super_hidden_path = path.super_hide();
    assert!(super_hidden_path.is_ok());
    assert_eq!(super_hidden_path.as_ref().unwrap(), &path);
    assert!(super_hidden_path.as_ref().unwrap().exists());
    assert!(super_hidden_path.as_ref().unwrap().is_file());
    assert!(super_hidden_path.as_ref().unwrap().is_super_hidden().unwrap());

    let unhidden_path = super_hidden_path.as_ref().unwrap().clone().super_unhide();
    // Can't unhide a file inside a hidden system directory
    assert!(unhidden_path.is_err());

    // Cleanup created files
    assert!(std::fs::remove_file(super_hidden_path.as_ref().unwrap()).is_ok());
    assert!(std::fs::remove_dir(super_hidden_path.as_ref().unwrap().parent().unwrap()).is_ok());
}
```

### Auto-Creator and Auto-Deletor

The functions `auto_creator` and `auto_deleter` are provided for convenience.

> Especially the `auto_deletor` is very powerful and must be used with care.\
> Please read the provided documentation directly on the function itself.

```rust
use areia::{auto_creator, auto_deletor};
use std::path::PathBuf;
let path = PathBuf::from("test_dir/test_file.txt");
assert!(auto_creator(&path).is_ok());
assert!(&path.exists());
assert!(auto_deletor(&path).is_ok());
assert!(!&path.exists());
```

They accept anything that implements `Into<PathBuf>`:

```rust
use areia::{auto_creator, auto_deletor};

let path = "str_test_dir/test_file.txt";
assert!(auto_creator(path).is_ok());
assert!(std::path::Path::new(path).exists());
assert!(auto_deletor(path).is_ok());
assert!(!std::path::Path::new(path).exists());
```

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
        - [x] macOS
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
        - [x] macOS
- [x] Stable Features
    - [x] Remove dev `unwraps`
    - [x] Documentation
        - [x] Examples in all function documentation
        - [x] README
            - [x] Full usage examples
    - [x] Tests
    - [x] Basic Hiding
    - [x] Atomic "Hide-and-Move"
    - [x] Super Hiding
        - [x] macOS Hybrid Support
        - [x] Windows "System" Flag
- [x] Nice-to-Haves
    - [x] Directory "Auto-Creator"
    - [x] Directory "Auto-Deletor"
- [ ] after 1.0.0
    - [x] `dotfile` path maker (No automatic file creation)
    - [ ] Support system level directories (maybe)
        - [ ] Windows
            - [ ] ProgramFiles
        - [ ] Unix
            - [ ] Root Bin
            - [ ] Root Data
            - [ ] Root Config

## `BaseDirs` and `UserDirs` expected output

<details>
<summary>The tables below are taken from the directories crate. Click for the full license.</summary>

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

