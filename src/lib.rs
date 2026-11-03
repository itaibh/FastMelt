pub mod abi;
pub mod fallback;
pub mod parser;
pub mod pipeline;

pub use abi::*;
pub use fallback::UpstreamMlt;
pub use parser::MltProject;
pub use pipeline::FrameBuffer;

pub fn initialize() {
    env_logger::init();
    log::info!("OliveMltBridge core initialized");
}
