use crate::blocker::app::AppBlockAction;

/// The `BlockerSpec` trait defines the interface for a blocking policy,
/// providing methods to specify what actions should be taken for blocking
/// different system features such as applications, processes, system settings,
/// etc. This trait contains both platform-independent and platform-specific methods.
pub trait BlockerSpec {
    /// Retrieves the action to be performed when blocking an application.
    ///
    /// This defines what happens when an app needs to be blocked, such as
    /// terminating, hiding, or restricting its access.
    ///
    /// # Returns
    /// `AppBlockAction` - the specified action to take when blocking an app.
    fn app_block_action(&self) -> AppBlockAction;

    /// Retrieves the action to be performed when the user attempts to bypass
    /// or escape the block.
    ///
    /// This defines the action to be taken when a user tries to circumvent
    /// the blocking policy.
    ///
    /// # Returns
    /// `AppBlockAction` - the specified action to take when an escape attempt occurs.
    fn escape_block_action(&self) -> AppBlockAction;

    /// Indicates whether the Task Manager (or Activity Monitor) should
    /// be blocked.
    ///
    /// # Returns
    /// `true` if Task Manager/Activity Monitor should be blocked,
    /// `false` otherwise.
    fn should_block_task_manager(&self) -> bool;

    /// Determines whether a window with the given title should be blocked.
    ///
    /// This is applicable to all platforms except macOS.
    ///
    /// # Parameters
    /// - `title`: The title of the window to check for blocking.
    ///
    /// # Returns
    /// `true` if the window should be blocked, `false` otherwise.
    #[cfg(not(target_os = "macos"))]
    fn should_block_window(&self, title: &str) -> bool;

    /// Indicates whether terminal applications should be blocked.
    ///
    /// # Returns
    /// `true` if terminal applications should be blocked, `false` otherwise.
    fn should_block_terminal(&self) -> bool;

    /// Indicates whether system settings should be blocked.
    ///
    /// # Returns
    /// `true` if system settings should be blocked, `false` otherwise.
    fn should_block_system_settings(&self) -> bool;

    /// Determines whether a process with the specified name should be blocked.
    ///
    /// This is applicable only to Windows platforms.
    ///
    /// # Parameters
    /// - `process_name`: The name of the process to check for blocking.
    ///
    /// # Returns
    /// `true` if the process should be blocked, `false` otherwise.
    #[cfg(target_os = "windows")]
    fn should_block_process(&self, process_name: &str) -> bool;

    /// Indicates whether sign-out buttons should be blocked.
    ///
    /// This is applicable only to Windows platforms.
    ///
    /// # Returns
    /// `true` if sign-out buttons should be blocked, `false` otherwise.
    #[cfg(target_os = "windows")]
    fn should_block_sign_out_buttons(&self) -> bool;

    /// Indicates whether installers should be blocked.
    ///
    /// This is applicable only to Windows platforms.
    ///
    /// # Returns
    /// `true` if installers should be blocked, `false` otherwise.
    #[cfg(target_os = "windows")]
    fn should_block_installers(&self) -> bool;

    /// Determines whether an application with the specified bundle ID should be blocked.
    ///
    /// This is applicable only to macOS platforms.
    ///
    /// # Parameters
    /// - `bundle_id`: The bundle ID of the application to check for blocking.
    ///
    /// # Returns
    /// `true` if the application should be blocked, `false` otherwise.
    #[cfg(target_os = "macos")]
    fn should_block_bundle_id(&self, bundle_id: &str);
}
