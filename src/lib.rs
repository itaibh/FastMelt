pub mod abi;
pub mod fallback;
pub mod logger;
pub mod parser;
pub mod pipeline;

pub use abi::*;
pub use fallback::UpstreamMlt;
pub use logger::{get_log_paths, log_msg};
pub use parser::MltProject;
pub use pipeline::FrameBuffer;

pub fn initialize() {
    logger::log_msg("CORE", "OliveMltBridge core initialized");
}

