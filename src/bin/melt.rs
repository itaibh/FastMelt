use mlt_7::cache::TextureCache;
use mlt_7::encoder::HardwareEncoder;
use mlt_7::evaluator::TimelineEvaluator;
use mlt_7::fallback::UpstreamMlt;
use mlt_7::logger::log_msg;
use mlt_7::parser::MltProject;
use mlt_7::pipeline::FrameBuffer;
use mlt_7::VERSION;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
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
        println!("melt 7.43.0 (FastMelt v{})", VERSION);
        log_msg("MELT", &format!("Printed version string (v{}) and exited.", VERSION));
        return;
    }

    // Check if invoked with --help or no args
    if args.len() < 2 || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        log_msg("MELT", "Printed help message and exited.");
        println!("FastMelt melt v{} (GPU-Accelerated drop-in replacement for MLT melt)", VERSION);
        println!("Usage: melt [options] <project.mlt>");
        println!("Options:");
        println!("  -progress2 <file>   Render file and print progress on stdout");
        println!("  -loglevel <level>   Set logging level (error, warning, info, debug)");
        println!("  -version, --version Print version information");
        return;
    }

    // Look for project file and options in arguments
    let mut project_path: Option<String> = None;
    let mut progress_mode = false;
    let mut log_level: Option<String> = None;
    let mut cli_in: Option<u64> = None;
    let mut cli_out: Option<u64> = None;
    let mut cli_target: Option<String> = None;
    let mut is_null_consumer = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-progress2" => {
                progress_mode = true;
                if i + 1 < args.len() && !args[i + 1].starts_with('-') {
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
            "-consumer" => {
                if i + 1 < args.len() {
                    let consumer_val = &args[i + 1];
                    if consumer_val == "null" {
                        is_null_consumer = true;
                    } else if let Some(target) = consumer_val.strip_prefix("avformat:") {
                        cli_target = Some(target.to_string());
                    } else if consumer_val.starts_with("target=") {
                        cli_target = Some(consumer_val[7..].to_string());
                    }
                    i += 1;
                }
            }
            arg if arg.starts_with("in=") => {
                cli_in = arg[3..].parse::<u64>().ok();
            }
            arg if arg.starts_with("out=") => {
                cli_out = arg[4..].parse::<u64>().ok();
            }
            arg if arg.ends_with(".mlt") || arg.ends_with(".kdenlive") => {
                project_path = Some(arg.to_string());
            }
            _ => {}
        }
        i += 1;
    }

    log_msg("MELT", &format!(
        "Parsed options: progress_mode={}, log_level={:?}, project_path={:?}, cli_target={:?}, cli_in={:?}, cli_out={:?}, is_null_consumer={}", 
        progress_mode, log_level, project_path, cli_target, cli_in, cli_out, is_null_consumer
    ));

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
    let project = match MltProject::parse(&xml_content) {
        Ok(p) => {
            log_msg("MELT", &format!("Parsed project: profile={}x{} @ {}/{} fps, {} tractors, {} playlists, {} producers, consumer={:?}, total_frames={}",
                p.profile.width,
                p.profile.height,
                p.profile.frame_rate_num,
                p.profile.frame_rate_den,
                p.tractors.len(),
                p.playlists.len(),
                p.producers.len(),
                p.consumer.as_ref().map(|c| (&c.target, &c.vcodec, &c.acodec)),
                p.get_total_frames()
            ));
            Some(p)
        }
        Err(e) => {
            log_msg("MELT", &format!("XML parser advisory: {}", e));
            None
        }
    };

    let start_time = Instant::now();

    // Check if user explicitly requested forced fallback to stock melt
    if env::var("FASTMELT_FORCE_FALLBACK").is_ok() || env::var("OLIVE_FORCE_FALLBACK").is_ok() {
        log_msg("MELT", "Force fallback set: Delegating directly to stock melt...");
        let code = UpstreamMlt::delegate_to_stock_melt(&args[1..]).unwrap_or(1);
        log_msg("MELT", &format!("Upstream melt returned code: {}", code));
        log_msg("MELT", "=== INVOCATION END ===");
        std::process::exit(code);
    }

    // Check if user explicitly requested isolated benchmark simulation mode
    if env::var("FASTMELT_BENCHMARK_SIMULATION").is_ok() || env::var("OLIVE_BENCHMARK_SIMULATION").is_ok() {
        log_msg("MELT", "Benchmark simulation set: Running in-memory framebuffer benchmark loop...");
        let mut fb = FrameBuffer::new(1080, 1920);
        let total_frames = 100;
        for frame in 1..=total_frames {
            fb.clear([0, 0, 0, 255]);
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

    // Check if project has unsupported tractor-level features
    if let Some(proj) = project.as_ref() {
        if let Some(reason) = proj.check_unsupported_tractor_features() {
            log_msg("MELT", &format!("Detected unsupported tractor feature: {}. Delegating rendering to upstream MLT engine...", reason));
            let code = UpstreamMlt::delegate_to_stock_melt(&args[1..]).unwrap_or(1);
            log_msg("MELT", &format!("Upstream melt returned code: {}", code));
            log_msg("MELT", "=== INVOCATION END ===");
            std::process::exit(code);
        }
    }

    // Determine target output file
    let target_file = cli_target.or_else(|| project.as_ref().and_then(|p| p.get_consumer_target()));

    // Phase 1 Hardware Encoder Pipeline:
    // If we have a project and a target video file (and not a null probe), execute direct hardware encoder sink
    if let (Some(proj), Some(target)) = (project.as_ref(), target_file.as_ref()) {
        if !is_null_consumer && !target.is_empty() && target != "null" {
            log_msg("MELT", &format!("Phase 1: Initializing direct hardware encoder pipeline for target: {}", target));

            let width = proj.profile.width;
            let height = proj.profile.height;
            let fps_num = proj.profile.frame_rate_num;
            let fps_den = proj.profile.frame_rate_den;

            let (start_frame, end_frame) = proj.get_render_bounds(cli_in, cli_out);
            let total_frames = if end_frame >= start_frame { end_frame - start_frame + 1 } else { 1 };

            log_msg("MELT", &format!(
                "Starting hardware render pipeline: {}x{} @ {}/{} fps, frames {}..={} ({} total frames) -> {}",
                width, height, fps_num, fps_den, start_frame, end_frame, total_frames, target
            ));

            let proj_dir = Path::new(&resolved_path).parent();
            let sub_file = proj.get_subtitle_file(proj_dir);

            // Check if project has audio tracks to render
            let has_audio = mlt_7::audio::has_audio_content(proj);

            let (video_encode_target, audio_thread) = if has_audio {
                let temp_video = format!("{}.video.tmp.mp4", target);
                let temp_audio_wav = format!("{}.audio.tmp.wav", target);
                let temp_audio_xml = format!("{}.audio_proj.kdenlive", target);

                let audio_thread_handle = if let Some(audio_xml) = mlt_7::audio::generate_audio_only_xml(&xml_content) {
                    if fs::write(&temp_audio_xml, audio_xml).is_ok() {
                        let xml_path_clone = temp_audio_xml.clone();
                        let wav_path_clone = temp_audio_wav.clone();
                        let start_f = start_frame;
                        let end_f = end_frame;
                        Some(std::thread::spawn(move || {
                            let res = mlt_7::audio::render_audio_pass(&xml_path_clone, start_f, end_f, &wav_path_clone);
                            let _ = fs::remove_file(&xml_path_clone);
                            res
                        }))
                    } else {
                        None
                    }
                } else {
                    None
                };

                (temp_video, Some((temp_audio_wav, audio_thread_handle)))
            } else {
                (target.clone(), None)
            };

            match HardwareEncoder::new(width, height, fps_num, fps_den, &video_encode_target, sub_file.as_deref()) {
                Ok(mut encoder) => {
                    let mut cache = TextureCache::new();
                    let mut evaluator = TimelineEvaluator::new(proj, &mut cache);
                    let mut fb = FrameBuffer::new(width, height);

                    let mut render_success = true;

                    for frame in 1..=total_frames {
                        let timeline_frame = start_frame + frame - 1;
                        evaluator.evaluate_frame(timeline_frame, &mut fb);

                        if let Err(e) = encoder.write_frame(fb.raw_bytes()) {
                            log_msg("MELT", &format!("Hardware encoder pipe error at frame {}: {}", frame, e));
                            render_success = false;
                            break;
                        }

                        let elapsed = start_time.elapsed().as_secs_f64();
                        let fps = if elapsed > 0.001 { (frame as f64) / elapsed } else { 0.0 };

                        if progress_mode {
                            let pct = if total_frames > 0 { ((frame as f64 / total_frames as f64) * 100.0).round() as u64 } else { 0 };
                            eprintln!("Current Frame: {:10}, percentage: {:10}", frame, pct);
                            let _ = io::stderr().flush();

                            println!("{}\t{}\t{}", frame, fps.round() as u64, elapsed.round() as u64);
                            let _ = io::stdout().flush();
                        }
                    }

                    if render_success {
                        match encoder.finish() {
                            Ok(code) => {
                                // If audio was rendered concurrently, mux video and audio together
                                if let Some((temp_audio_wav, audio_handle_opt)) = audio_thread {
                                    if let Some(handle) = audio_handle_opt {
                                        let _ = handle.join();
                                    }

                                    let audio_exists = std::path::Path::new(&temp_audio_wav).exists()
                                        && fs::metadata(&temp_audio_wav).map(|m| m.len() > 0).unwrap_or(false);

                                    if audio_exists {
                                        log_msg("MELT", "Muxing hardware video with rendered audio track...");
                                        let mux_code = mlt_7::audio::mux_video_audio(&video_encode_target, &temp_audio_wav, target)
                                            .unwrap_or(1);
                                        let _ = fs::remove_file(&video_encode_target);
                                        let _ = fs::remove_file(&temp_audio_wav);
                                        if mux_code != 0 {
                                            log_msg("MELT", &format!("Warning: FFmpeg mux returned code {}", mux_code));
                                        }
                                    } else {
                                        log_msg("MELT", "No audio track generated; renaming raw video to target...");
                                        let _ = fs::rename(&video_encode_target, target);
                                        let _ = fs::remove_file(&temp_audio_wav);
                                    }
                                }

                                let total_time = start_time.elapsed().as_secs_f64();
                                let avg_fps = if total_time > 0.001 { (total_frames as f64) / total_time } else { 0.0 };
                                log_msg("MELT", &format!(
                                    "Direct hardware rendering finished successfully: {} frames in {:.2}s ({:.1} fps) with exit code: {}",
                                    total_frames, total_time, avg_fps, code
                                ));
                                log_msg("MELT", "=== INVOCATION END ===");
                                std::process::exit(code);
                            }
                            Err(e) => {
                                log_msg("MELT", &format!("Error waiting for encoder completion: {}. Falling back to upstream melt...", e));
                            }
                        }
                    } else {
                        log_msg("MELT", "Hardware encoder pipe write failed. Falling back to upstream melt...");
                    }
                }
                Err(e) => {
                    log_msg("MELT", &format!("Could not spawn hardware encoder: {}. Falling back to upstream melt...", e));
                }
            }
        }
    }

    // Default Fallback: Execute rendering pipeline via stock MLT engine
    log_msg("MELT", "Delegating rendering job to upstream MLT engine...");
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


