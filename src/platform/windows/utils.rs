use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::ptr::null_mut;
use windows::Win32::Foundation::{CloseHandle, BOOL, FALSE, HMODULE, HWND, LPARAM, MAX_PATH, TRUE, WPARAM};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::ProcessStatus::GetModuleFileNameExW;
use windows::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_QUERY_INFORMATION, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_TERMINATE};
use windows::Win32::UI::WindowsAndMessaging::{CloseWindow, EnumChildWindows, GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId, IsWindowVisible, SendMessageW, SC_MINIMIZE, WM_SYSCOMMAND};

const STORE_WINDOWS_APP: &'static str = "applicationframehost.exe";

pub fn trim_null_char(value: &str) -> &str {
    value.trim_end_matches(char::from(0))
}

pub fn get_process_id(window: HWND) -> u32 {
    let mut process_id: u32 = 0;
    unsafe {
        GetWindowThreadProcessId(window, Some(&mut process_id as *mut u32));
    }
    process_id
}

pub fn get_window_title(window: HWND) -> String {
    let mut title: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
    let count = unsafe { GetWindowTextW(window, &mut title) };
    if count <= 0 {
        String::new()
    } else {
        String::from_utf16_lossy(&title[0..(count as usize)])
    }
}

pub fn is_visible(window: HWND) -> bool {
    unsafe { IsWindowVisible(window).as_bool() }
}

pub fn minimize(window: HWND) {
    if is_visible(window) {
        unsafe {
            SendMessageW(
                window,
                WM_SYSCOMMAND,
                WPARAM(SC_MINIMIZE as usize),
                LPARAM(0),
            );
        }
    }
}

pub fn terminate_pid(pid: u32) {
    unsafe {
        let process = OpenProcess(PROCESS_TERMINATE, false, pid);
        if let Ok(process) = process {
            TerminateProcess(process, 1).unwrap();
            CloseHandle(process).unwrap();
        }
    }
}

pub fn terminate(window: HWND) {
    let pid = get_process_id(window);
    terminate_pid(pid)
}

pub fn close_window(window: HWND) {
    let _ = unsafe { CloseWindow(window) };
}

pub fn get_active_window() -> HWND {
    unsafe { GetForegroundWindow() }
}

unsafe extern "system" fn get_windows_store_app_name(w: HWND, ptr: LPARAM) -> BOOL {
    let exe_name = &mut *(ptr.0 as *mut String);
    if let Some(name) = get_process_name(w) {
        if name.to_lowercase() != STORE_WINDOWS_APP {
            *exe_name = name;
            return FALSE; // Stop enumeration
        }
    }
    TRUE
}

pub fn get_process_name(window: HWND) -> Option<String> {
    unsafe {
        let id = get_process_id(window);
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if let Ok(snapshot) = snapshot {
            // Initialize the process entry structure
            let mut entry: PROCESSENTRY32W = PROCESSENTRY32W {
                dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
                cntUsage: 0,
                th32ProcessID: 0,
                th32DefaultHeapID: 0,
                th32ModuleID: 0,
                cntThreads: 0,
                th32ParentProcessID: 0,
                pcPriClassBase: 0,
                dwFlags: 0,
                szExeFile: [0; MAX_PATH as usize],
            };

            // Iterate through every process entry in the snapshot
            if Process32FirstW(snapshot, &mut entry).is_ok() {
                loop {
                    let process_id = entry.th32ProcessID;
                    if process_id == id {
                        let mut exe_name = OsString::from_wide(&entry.szExeFile).into_string().unwrap();
                        exe_name = trim_null_char(&exe_name).into();
                        CloseHandle(snapshot).unwrap();

                        if exe_name.eq_ignore_ascii_case(STORE_WINDOWS_APP) {
                            let _ = EnumChildWindows(
                                window,
                                Some(get_windows_store_app_name),
                                LPARAM(&mut exe_name as *mut String as isize),
                            );
                        }
                        return Some(exe_name);
                    }
                    if Process32NextW(snapshot, &mut entry).is_err() {
                        break;
                    }
                }
            }

            // Close the snapshot handle
            CloseHandle(snapshot).unwrap();
        }
        None
    }
}
pub fn get_process_file_path(window: HWND) -> Result<String, String> {
    let pid = get_process_id(window);
    unsafe {
        let mut process = OpenProcess(PROCESS_QUERY_INFORMATION, false, pid)
            .map_err(|e| format!("Failed to open process: {}", e));
        if let Err(_) = process {
            // Retry with PROCESS_QUERY_LIMITED_INFORMATION access
            process = Ok(OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid)
                .map_err(|e| format!("Failed to open process: {}", e))?);
        }
        let process = process?;
        let mut image_path: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
        GetModuleFileNameExW(process, HMODULE(null_mut()), &mut image_path);
        CloseHandle(process).map_err(|e| format!("Failed to close process handle: {}", e))?;
        let process_image_path = OsString::from_wide(&image_path)
            .into_string()
            .unwrap_or_default();
        Ok(process_image_path)
    }
}

