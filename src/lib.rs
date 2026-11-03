pub mod abi;
pub mod encoder;
pub mod fallback;
pub mod logger;
pub mod parser;
pub mod pipeline;

pub use abi::*;
pub use encoder::HardwareEncoder;
pub use fallback::UpstreamMlt;
pub use logger::{get_log_paths, log_msg};
pub use parser::MltProject;
pub use pipeline::FrameBuffer;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn initialize() {
    logger::log_msg("CORE", &format!("OliveMltBridge core initialized (v{})", VERSION));
}


