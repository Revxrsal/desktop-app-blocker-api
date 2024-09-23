use std::process::Command;

pub fn close_system_settings() -> bool {
    close_running("System Settings")
}

pub fn close_terminal() -> bool {
    close_running("Terminal")
}

pub fn close_activity_monitor() -> bool {
    close_running("Activity Monitor")
}

pub fn close_running(name: &str) -> bool {
    if is_process_running(name) {
        close_process(name);
        return true;
    }
    false
}

pub fn is_process_running(process_name: &str) -> bool {
    let output = Command::new("pgrep")
        .arg(process_name)
        .output()
        .expect("Failed to execute pgrep");

    !output.stdout.is_empty()
}

pub fn close_process(process_name: &str) {
    Command::new("pkill")
        .arg(process_name)
        .output()
        .expect("Failed to execute pkill");
}
