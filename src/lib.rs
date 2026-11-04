pub mod abi;
pub mod audio;
pub mod cache;
pub mod encoder;
pub mod evaluator;
pub mod fallback;
pub mod logger;
pub mod parser;
pub mod pipeline;

pub use abi::*;
pub use audio::*;
pub use cache::TextureCache;
pub use encoder::HardwareEncoder;
pub use evaluator::TimelineEvaluator;
pub use fallback::UpstreamMlt;
pub use logger::{get_log_paths, log_msg};
pub use parser::MltProject;
pub use pipeline::FrameBuffer;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn initialize() {
    logger::log_msg("CORE", &format!("OliveMltBridge core initialized (v{})", VERSION));
}


