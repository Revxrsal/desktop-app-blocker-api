[![Crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/desktop-app-blocker-api.svg

[crates-url]: https://crates.io/crates/desktop-app-blocker-api

# Desktop App Blocker API

**Desktop App Blocker API** is a utility Rust crate for performing cross-platform "app blocking". Useful for
productivity applications or productivity software.

## Features

- Support for Windows and macOS
- Detect/Block Task Manager or Activity Monitor
- Detect/Block terminal applications
- Detect/Block system settings
- Detect/Block installers (Windows)
- Block by window names
- Block executables (Windows)
- Block by bundle IDs (macOS)
- Structs include support for serde, bincode and specta

## Usage

```toml
desktop-app-blocker-api = "0.1.0"
```

#### Features

- `serde`: Includes serde `Serialize` and `Deserialize` derives for types
- `specta`: Includes `Type` derives for types
- `bincode`: Includes `Encode` and `Decode` derives for types