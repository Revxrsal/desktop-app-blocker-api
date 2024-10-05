use self::utils::{
    get_active_window, get_process_name, get_window_title,
    is_visible, minimize,
};
use crate::blocker::app::AppBlockAction;
use crate::spec::BlockerSpec;
use crate::platform::windows::utils::{close_window, terminate};
use windows::Win32::Foundation::HWND;
use crate::platform::sys::utils::get_process_file_path;

pub mod utils;

/// Task manager's name
const TASK_MANAGER: &str = "taskmgr.exe";

/// A generic installer/uninstaller
const INSTALLER: &str = "msiexec.exe";

#[derive(Debug)]
pub struct WindowsBlocker;

impl WindowsBlocker {
    
    pub fn perform_block(spec: &impl BlockerSpec) {
        let app_block_action = spec.app_block_action();
        let escape_block_action = spec.escape_block_action();
        let current_window = get_active_window();
        let process_name = get_process_name(current_window);
        let title = get_window_title(current_window);
        let directory = get_process_file_path(current_window).ok();
        if let Some(process_name) = process_name {
            if spec.should_block_sign_out_buttons() && is_illegal_context_menu(current_window, &process_name, &title) {
                close_window(current_window);
                return;
            }
            if is_whitelisted_system_app(&process_name) {
                return;
            }
            if spec.should_block_task_manager() && is_task_manager(&process_name) {
                execute_action(&escape_block_action, current_window);
                return;
            }
            if spec.should_block_terminal() && is_terminal_app(&process_name) {
                execute_action(&escape_block_action, current_window);
                return;
            }
            if spec.should_block_system_settings() && is_system_settings_app(&process_name) {
                execute_action(&escape_block_action, current_window);
                return;
            }
            if spec.should_block_installers() && is_installer(&process_name) {
                execute_action(&escape_block_action, current_window);
                return;
            }
            if spec.should_block_window(&process_name, &title, directory.as_ref()) {
                execute_action(&app_block_action, current_window);
                return;
            }
        }
    }
}

fn execute_action(action: &AppBlockAction, window: HWND) {
    match action {
        AppBlockAction::Close => terminate(window),
        AppBlockAction::MinimizeWindow => minimize_if_visible(window)
    }
}

fn is_illegal_context_menu(_window: HWND, process_name: &str, title: &str) -> bool {
    if process_name.eq("explorer.exe") && title.is_empty() {
        return true;
    }
    false
}

fn minimize_if_visible(window: HWND) {
    if is_visible(window) {
        minimize(window);
    }
}

fn is_task_manager(process_name: &str) -> bool {
    TASK_MANAGER.eq_ignore_ascii_case(process_name)
}

fn is_terminal_app(process_name: &str) -> bool {
    match process_name {
        "windowsterminal.exe" | "powershell.exe" | "cmd.exe" | "conhost.exe" => true,
        _ => false,
    }
}

fn is_system_settings_app(process_name: &str) -> bool {
    match process_name {
        "systemsettings.exe" | "regedit.exe" | "control.exe" | "mmc.exe"
        | "startmenuexperiencehost.exe" | "csrss.exe" => true,
        _ => false,
    }
}

fn is_whitelisted_system_app(process_name: &str) -> bool {
    match process_name {
        "lockapp.exe"
        | "shellhostexperience.exe"
        | "mmtoastnotifier.exe"
        | "explorer.exe"
        | "searchhost.exe"
        | "windowsinternal.composableshell.experiences.textinput.inputapp.exe" => true,
        _ => false,
    }
}

fn is_installer(process_name: &str) -> bool {
    INSTALLER.eq_ignore_ascii_case(process_name)
}
