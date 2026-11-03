use crate::logger::log_msg;
use libloading::Library;
use std::process::Command;

pub struct UpstreamMlt {
    _lib: Option<Library>,
}

impl UpstreamMlt {
    pub fn new() -> Self {
        // Try to locate upstream libmlt-7.so in system or flatpak locations
        let candidates = [
            "/app/lib/libmlt-7.so.7.43.0",
            "/var/lib/flatpak/app/org.kde.kdenlive/current/active/files/lib/libmlt-7.so.7.43.0",
            "/var/lib/flatpak/app/org.kde.kdenlive/current/active/files/lib/libmlt-7.so",
            "/usr/lib/x86_64-linux-gnu/libmlt-7.so",
            "/usr/lib/libmlt-7.so",
            "libmlt-7.so",
        ];

        for path in candidates {
            if std::path::Path::new(path).exists() {
                unsafe {
                    if let Ok(lib) = Library::new(path) {
                        log_msg("FALLBACK", &format!("Successfully dynamically loaded upstream MLT from: {}", path));
                        return Self { _lib: Some(lib) };
                    }
                }
            }
        }

        log_msg("FALLBACK", "Could not locate upstream libmlt-7.so. Fallback will use CLI delegate.");
        Self { _lib: None }
    }

    /// Delegate entire rendering to the stock upstream melt binary if needed
    pub fn delegate_to_stock_melt(args: &[String]) -> std::io::Result<i32> {
        let stock_melt_candidates = [
            "/app/bin/melt-7",
            "/var/lib/flatpak/app/org.kde.kdenlive/current/active/files/bin/melt-7",
            "/usr/bin/melt-7",
            "/var/lib/flatpak/app/org.kde.kdenlive/current/active/files/bin/melt",
            "/usr/bin/melt",
            "/app/bin/melt",
        ];

        let binary = stock_melt_candidates
            .iter()
            .find(|p| std::path::Path::new(p).exists())
            .unwrap_or(&"/usr/bin/melt");

        log_msg("FALLBACK", &format!("Delegating unaccelerated job to upstream binary: {} with args: {:?}", binary, args));

        let mut child = Command::new(binary)
            .args(args)
            .spawn()?;

        let status = child.wait()?;
        let code = status.code().unwrap_or(1);
        log_msg("FALLBACK", &format!("Upstream binary {} exited with code: {}", binary, code));
        Ok(code)
    }
}

