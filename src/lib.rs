use spec::BlockerSpec;

pub mod platform;
pub mod blocker;
pub mod spec;

pub fn perform_block(spec: &impl BlockerSpec) {
    platform::PlatformBlocker::perform_block(spec);
}
