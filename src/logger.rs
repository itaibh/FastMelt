use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::SystemTime;

static LOG_MUTEX: Mutex<()> = Mutex::new(());

/// Determine primary and secondary log file locations.
pub fn get_log_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    // 1. Explicit environment variable override
    if let Ok(custom) = std::env::var("FASTMELT_LOG_FILE").or_else(|_| std::env::var("OLIVE_LOG_FILE")) {
        paths.push(PathBuf::from(custom));
    }

    // 2. Local working directory log file
    paths.push(PathBuf::from("fastmelt.log"));

    // 3. User home cache
    if let Ok(home) = std::env::var("HOME") {
        paths.push(PathBuf::from(home).join(".cache/fastmelt.log"));
    }

    // 4. System /tmp fallback
    paths.push(PathBuf::from("/tmp/fastmelt.log"));

    paths
}

/// Write a timestamped entry to the log file(s).
pub fn log_msg(tag: &str, message: &str) {
    let _guard = LOG_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0);

    let pid = std::process::id();
    let formatted = format!("[{:.3}] [PID:{}] [{}] {}\n", now, pid, tag, message);

    // Try logging to the first writable path
    for path in get_log_paths() {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&path) {
            let _ = file.write_all(formatted.as_bytes());
            let _ = file.flush();
            break;
        }
    }

    // Also echo to stderr
    eprint!("[{}] {}", tag, formatted);
}

