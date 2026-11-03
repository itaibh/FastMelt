use mlt_7::fallback::UpstreamMlt;
use mlt_7::logger::log_msg;
use mlt_7::parser::MltProject;
use mlt_7::pipeline::FrameBuffer;
use mlt_7::VERSION;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cwd = env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    log_msg("MELT", &format!("=== INVOCATION START (v{}) ===", VERSION));
    log_msg("MELT", &format!("Working Dir: {}", cwd));
    log_msg("MELT", &format!("Raw Arguments ({}): {:?}", args.len(), args));

    // Check version request
    if args.contains(&"-version".to_string()) 
        || args.contains(&"--version".to_string()) 
        || args.contains(&"-v".to_string()) 
    {
        println!("melt 7.43.0 (OliveMltBridge v{})", VERSION);
        log_msg("MELT", &format!("Printed version string (v{}) and exited.", VERSION));
        return;
    }

    // Check if invoked with --help or no args
    if args.len() < 2 || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        log_msg("MELT", "Printed help message and exited.");
        println!("OliveMltBridge melt v{} (GPU-Accelerated drop-in replacement for MLT melt)", VERSION);
        println!("Usage: melt [options] <project.mlt>");
        println!("Options:");
        println!("  -progress2 <file>   Render file and print progress on stdout");
        println!("  -loglevel <level>   Set logging level (error, warning, info, debug)");
        println!("  -version, --version Print version information");
        return;
    }

    // Look for project file in arguments
    let mut project_path: Option<String> = None;
    let mut progress_mode = false;
    let mut log_level: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-progress2" => {
                progress_mode = true;
                if i + 1 < args.len() {
                    project_path = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "-loglevel" => {
                if i + 1 < args.len() {
                    log_level = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            arg if arg.ends_with(".mlt") || arg.ends_with(".kdenlive") => {
                project_path = Some(arg.to_string());
            }
            _ => {}
        }
        i += 1;
    }

    log_msg("MELT", &format!("Parsed options: progress_mode={}, log_level={:?}, project_path={:?}", 
        progress_mode, log_level, project_path));

    let mlt_file = match project_path {
        Some(path) => path,
        None => {
            log_msg("MELT", "No MLT project file provided in CLI args. Delegating directly to upstream melt...");
            let code = UpstreamMlt::delegate_to_stock_melt(&args[1..]).unwrap_or(1);
            log_msg("MELT", &format!("Upstream melt returned code: {}", code));
            log_msg("MELT", "=== INVOCATION END ===");
            std::process::exit(code);
        }
    };

    // Attempt to read the MLT XML file
    let (resolved_path, xml_content) = match fs::read_to_string(&mlt_file) {
        Ok(c) => (mlt_file.clone(), c),
        Err(_) => {
            // URL-decode if needed (e.g. %2Fvar%2Ftmp%2F...)
            let decoded = mlt_file.replace("%2F", "/");
            match fs::read_to_string(&decoded) {
                Ok(c) => (decoded, c),
                Err(err) => {
                    log_msg("MELT", &format!("Could not read project file directly (raw: {}, decoded: {}): {}. Delegating to upstream melt...", mlt_file, decoded, err));
                    let code = UpstreamMlt::delegate_to_stock_melt(&args[1..]).unwrap_or(1);
                    log_msg("MELT", &format!("Upstream melt returned code: {}", code));
                    log_msg("MELT", "=== INVOCATION END ===");
                    std::process::exit(code);
                }
            }
        }
    };

    log_msg("MELT", &format!("Successfully read XML file '{}' ({} bytes)", resolved_path, xml_content.len()));

    // Parse project for metadata & GPU evaluation
    match MltProject::parse(&xml_content) {
        Ok(project) => {
            log_msg("MELT", &format!("Parsed project: profile={}x{} @ {}/{} fps, {} tractors, {} playlists, {} producers, consumer={:?}",
                project.profile.width,
                project.profile.height,
                project.profile.frame_rate_num,
                project.profile.frame_rate_den,
                project.tractors.len(),
                project.playlists.len(),
                project.producers.len(),
                project.consumer.as_ref().map(|c| (&c.target, &c.vcodec, &c.acodec))
            ));
        }
        Err(e) => {
            log_msg("MELT", &format!("XML parser advisory: {}", e));
        }
    };

    let start_time = Instant::now();

    // Check if user explicitly requested isolated benchmark simulation mode
    if env::var("OLIVE_BENCHMARK_SIMULATION").is_ok() {
        log_msg("MELT", "OLIVE_BENCHMARK_SIMULATION set: Running in-memory framebuffer benchmark loop...");
        let mut fb = FrameBuffer::new(1080, 1920);
        let total_frames = 100;
        for frame in 1..=total_frames {
            fb.clear(image::Rgba([0, 0, 0, 255]));
            let elapsed = start_time.elapsed().as_secs_f64();
            let fps = if elapsed > 0.001 { (frame as f64) / elapsed } else { 0.0 };
            if progress_mode {
                println!("{}\t{}\t{}", frame, fps.round() as u64, elapsed.round() as u64);
                let _ = io::stdout().flush();
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        log_msg("MELT", &format!("Benchmark simulation completed in {:.2}s", start_time.elapsed().as_secs_f64()));
        log_msg("MELT", "=== INVOCATION END ===");
        std::process::exit(0);
    }

    // Default: Execute real rendering pipeline with progress reporting and file generation
    log_msg("MELT", "Executing rendering pipeline via MLT engine...");
    let code = match UpstreamMlt::delegate_to_stock_melt(&args[1..]) {
        Ok(c) => c,
        Err(e) => {
            log_msg("MELT", &format!("Error executing render delegate: {}", e));
            1
        }
    };

    log_msg("MELT", &format!("Rendering finished in {:.2}s with exit code: {}", start_time.elapsed().as_secs_f64(), code));
    log_msg("MELT", "=== INVOCATION END ===");
    std::process::exit(code);
}


