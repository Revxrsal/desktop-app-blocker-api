use crate::spec::BlockerSpec;

#[cfg(target_os = "linux")]
use linux as sys;
#[cfg(target_os = "macos")]
use macos as sys;
#[cfg(target_os = "windows")]
use windows as sys;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "windows")]
pub(crate) type PlatformBlocker = sys::WindowsBlocker;
#[cfg(target_os = "macos")]
pub(crate) type PlatformBlocker = sys::MacosBlocker;
#[cfg(target_os = "linux")]
pub(crate) type PlatformBlocker = sys::LinuxBlocker;

pub fn perform_block(spec: &impl BlockerSpec) {
    PlatformBlocker::perform_block(spec);
}
