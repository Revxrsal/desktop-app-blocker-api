# Desktop App Blocker API
DABA is a utility Rust crate for performing cross-platform "app blocking". Useful for productivity applications or productivity software.

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