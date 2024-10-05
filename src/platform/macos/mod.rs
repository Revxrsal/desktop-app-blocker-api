mod plist;
mod processes;

use crate::blocker::app::AppBlockAction;
use crate::spec::BlockerSpec;
use cacao::appkit::workspace::Workspace;

#[derive(Debug)]
pub struct MacosBlocker;

impl MacosBlocker {
    
    pub fn perform_block(spec: &impl BlockerSpec) {
        let workspace = Workspace::shared();
        let app_block_action = spec.app_block_action();
        let app = workspace.frontmost_application();
        if let Some(app) = app {
            let bundle_id = app.bundle_identifier();
            if let Some(bundle_id) = bundle_id {
                if spec.should_block_bundle_id(&bundle_id) {
                    match app_block_action {
                        AppBlockAction::Close => app.terminate(),
                        AppBlockAction::MinimizeWindow => app.hide()
                    }
                }
            }
        }
        if spec.should_block_terminal() {
            processes::close_terminal();
        }
        if spec.should_block_task_manager() {
            processes::close_activity_monitor();
        }
        if spec.should_block_system_settings() {
            processes::close_system_settings();
        }
    }
}
