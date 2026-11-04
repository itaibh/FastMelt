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

    /// Create a Command that invokes upstream stock melt (either native binary or via flatpak run)
    pub fn create_stock_melt_command(args: &[String]) -> Command {
        let stock_melt_candidates = [
            "/app/bin/melt-7.stock",
            "/app/bin/melt-7.real",
            "/app/bin/melt-7",
            "/usr/bin/melt-7",
            "/usr/bin/melt",
        ];

        let current_exe = std::env::current_exe().ok();
        let binary = stock_melt_candidates
            .iter()
            .find(|p| {
                let path = std::path::Path::new(p);
                if !path.exists() {
                    return false;
                }
                if let (Ok(canonical), Some(curr)) = (path.canonicalize(), &current_exe) {
                    if let Ok(curr_canonical) = curr.canonicalize() {
                        if canonical == curr_canonical {
                            return false; // Don't call ourselves recursively!
                        }
                    }
                }
                true
            })
            .copied();

        if let Some(bin) = binary {
            let mut cmd = Command::new(bin);
            cmd.args(args);
            cmd
        } else {
            // Fallback for host development: invoke via flatpak melt-7
            let mut cmd = Command::new("flatpak");
            cmd.arg("run")
                .arg("--filesystem=/tmp")
                .arg("--command=melt-7")
                .arg("org.kde.kdenlive")
                .args(args);
            cmd
        }
    }

    /// Delegate entire rendering to the stock upstream melt binary if needed
    pub fn delegate_to_stock_melt(args: &[String]) -> std::io::Result<i32> {
        let mut cmd = Self::create_stock_melt_command(args);
        log_msg("FALLBACK", &format!("Delegating rendering job to upstream: {:?} with args: {:?}", cmd.get_program(), args));

        let mut child = cmd
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()?;

        let status = child.wait()?;
        let code = status.code().unwrap_or(1);
        log_msg("FALLBACK", &format!("Upstream exited with code: {}", code));
        Ok(code)
    }

}

