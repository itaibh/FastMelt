use crate::logger::log_msg;
use std::collections::HashSet;
use std::io;
use std::path::Path;
use std::process::Command;

/// Check if an MLT project has active audio content
pub fn has_audio_content(project: &crate::parser::MltProject) -> bool {
    // Check if any playlist or tractor is marked as audio
    if project.playlists.values().any(|pl| pl.is_audio) {
        return true;
    }
    if project.tractors.values().any(|tr| tr.is_audio) {
        return true;
    }
    false
}

/// Generate a lightweight, audio-only MLT XML
/// Sets profile to 16x16 dummy resolution and strips video tracks, transitions and filters
pub fn generate_audio_only_xml(raw_xml: &str) -> Option<String> {
    let doc = roxmltree::Document::parse(raw_xml).ok()?;
    let root = doc.root_element();

    // 1. Collect all known audio IDs (playlists, tractors, audio files)
    let mut audio_ids = HashSet::new();

    for node in root.descendants() {
        if node.is_element() {
            let tag = node.tag_name().name();
            let id = node.attribute("id").unwrap_or("");
            if id.is_empty() {
                continue;
            }

            // Check if element has kdenlive:audio_track = 1
            let is_audio_prop = node.children().any(|c| {
                c.tag_name().name() == "property"
                    && c.attribute("name") == Some("kdenlive:audio_track")
                    && c.text().unwrap_or("").trim() == "1"
            });

            if is_audio_prop {
                audio_ids.insert(id.to_string());
            }

            // Check if resource is an audio file
            if tag == "producer" || tag == "chain" {
                for p in node.children().filter(|c| c.tag_name().name() == "property") {
                    if p.attribute("name") == Some("resource") {
                        if let Some(text) = p.text() {
                            let lower = text.to_lowercase();
                            if [".mp3", ".wav", ".aac", ".m4a", ".ogg", ".flac", ".opus", ".wma"]
                                .iter()
                                .any(|ext| lower.ends_with(ext))
                            {
                                audio_ids.insert(id.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    // 2. Build the output XML
    let mut out = String::with_capacity(raw_xml.len());
    out.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");

    serialize_clean_element(root, &audio_ids, &mut out);
    Some(out)
}

fn serialize_clean_element(
    node: roxmltree::Node,
    audio_ids: &HashSet<String>,
    out: &mut String,
) {
    let tag = node.tag_name().name();

    // Profile: override to 16x16 to ensure stock MLT allocates minimal video memory
    if tag == "profile" {
        out.push_str(" <profile width=\"16\" height=\"16\"");
        for attr in node.attributes() {
            if attr.name() != "width" && attr.name() != "height" {
                out.push_str(&format!(" {}=\"{}\"", attr.name(), escape_xml(attr.value())));
            }
        }
        out.push_str(" />\n");
        return;
    }

    // If this is a track inside a tractor, check if it's an audio track
    if tag == "track" {
        let producer_id = node.attribute("producer").unwrap_or("");
        if !audio_ids.contains(producer_id) {
            // Check if producer is a tractor that itself contains audio
            if let Some(parent_doc) = node.document().root_element().descendants().find(|n| {
                n.is_element() && n.tag_name().name() == "tractor" && n.attribute("id") == Some(producer_id)
            }) {
                let has_sub_audio = parent_doc.children().any(|c| {
                    c.tag_name().name() == "track"
                        && audio_ids.contains(c.attribute("producer").unwrap_or(""))
                });
                if !has_sub_audio {
                    return; // Skip non-audio track
                }
            } else {
                return; // Skip non-audio track
            }
        }
    }

    // If this is a transition inside a tractor, drop video transitions
    if tag == "transition" {
        let is_video_trans = node.children().any(|c| {
            c.tag_name().name() == "property"
                && c.attribute("name") == Some("mlt_service")
                && [
                    "qtblend", "composite", "affine", "frei0r", "movit", "wipe", "luma",
                ]
                .contains(&c.text().unwrap_or("").trim())
        });
        if is_video_trans {
            return; // Skip video transition
        }
    }

    // If this is a filter, drop video filters
    if tag == "filter" {
        let is_video_filter = node.children().any(|c| {
            c.tag_name().name() == "property"
                && c.attribute("name") == Some("mlt_service")
                && [
                    "qtblend", "avfilter.subtitles", "subtitles", "frei0r", "movit",
                    "crop", "boxblur", "gaussianblur",
                ]
                .contains(&c.text().unwrap_or("").trim())
        });
        if is_video_filter {
            return; // Skip video filter
        }
    }

    // Write element start tag
    out.push_str(&format!("<{}", tag));
    for attr in node.attributes() {
        out.push_str(&format!(" {}=\"{}\"", attr.name(), escape_xml(attr.value())));
    }

    let mut has_children = false;
    for child in node.children() {
        if child.is_element() {
            if !has_children {
                out.push_str(">\n");
                has_children = true;
            }
            serialize_clean_element(child, audio_ids, out);
        } else if child.is_text() {
            let text = child.text().unwrap_or("");
            if !text.trim().is_empty() {
                if !has_children {
                    out.push('>');
                    has_children = true;
                }
                out.push_str(&escape_xml(text));
            }
        }
    }

    if has_children {
        out.push_str(&format!("</{}>\n", tag));
    } else {
        out.push_str(" />\n");
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Run stock melt to render audio pass to WAV
pub fn render_audio_pass(
    audio_xml_path: &str,
    in_frame: u64,
    out_frame: u64,
    target_wav_path: &str,
) -> io::Result<i32> {
    log_msg(
        "AUDIO",
        &format!(
            "Starting background audio render: frames {}..={} -> {}",
            in_frame, out_frame, target_wav_path
        ),
    );

    let args = [
        audio_xml_path.to_string(),
        format!("in={}", in_frame),
        format!("out={}", out_frame),
        "-consumer".to_string(),
        format!("avformat:{}", target_wav_path),
        "vn=1".to_string(),
    ];

    let mut cmd = crate::fallback::UpstreamMlt::create_stock_melt_command(&args);

    cmd.stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped());

    let output = cmd.output()?;
    let code = output.status.code().unwrap_or(1);
    if code != 0 {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log_msg("AUDIO", &format!("Upstream audio render exited with code {}: {}", code, stderr));
    } else {
        log_msg("AUDIO", &format!("Audio render completed successfully -> {}", target_wav_path));
    }
    Ok(code)
}

/// Mux video and audio streams using FFmpeg
pub fn mux_video_audio(
    video_path: &str,
    audio_path: &str,
    target_path: &str,
) -> io::Result<i32> {
    log_msg(
        "AUDIO",
        &format!("Muxing video ({}) and audio ({}) into final target ({})", video_path, audio_path, target_path),
    );

    let ffmpeg_candidates = [
        "ffmpeg",
        "/app/bin/ffmpeg",
        "/usr/bin/ffmpeg",
        "/var/lib/flatpak/app/org.kde.kdenlive/current/active/files/bin/ffmpeg",
    ];

    let ffmpeg_bin = ffmpeg_candidates
        .iter()
        .find(|&&p| p == "ffmpeg" || Path::new(p).exists())
        .copied()
        .unwrap_or("ffmpeg");

    let is_webm = target_path.ends_with(".webm");

    let mut cmd = Command::new(ffmpeg_bin);
    cmd.arg("-y")
        .arg("-i").arg(video_path)
        .arg("-i").arg(audio_path)
        .arg("-c:v").arg("copy");

    if is_webm {
        cmd.arg("-c:a").arg("libopus")
            .arg("-b:a").arg("128k");
    } else {
        cmd.arg("-c:a").arg("aac")
            .arg("-b:a").arg("192k");
    }

    cmd.arg("-shortest")
        .arg(target_path)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped());

    let output = cmd.output()?;
    let code = output.status.code().unwrap_or(1);
    if code != 0 {
        let stderr = String::from_utf8_lossy(&output.stderr);
        log_msg("AUDIO", &format!("FFmpeg muxing failed with code {}: {}", code, stderr));
    } else {
        log_msg("AUDIO", &format!("Muxing finished successfully -> {}", target_path));
    }
    Ok(code)
}
