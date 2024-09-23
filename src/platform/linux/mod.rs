use crate::spec::BlockerSpec;

#[derive(Debug)]
pub struct LinuxBlocker;

impl LinuxBlocker {
    pub fn perform_block(spec: &impl BlockerSpec) {
        todo!("linux is unsupported.")
    }
}