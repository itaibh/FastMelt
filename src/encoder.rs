use std::io::{self, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, Command, Stdio};
use crate::logger::log_msg;

pub struct HardwareEncoder {
    child: Child,
    stdin: ChildStdin,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub frames_written: u64,
    target: String,
}

impl HardwareEncoder {
    pub fn new(
        width: u32,
        height: u32,
        fps_num: u32,
        fps_den: u32,
        target_path: &str,
        subtitle_path: Option<&Path>,
    ) -> io::Result<Self> {
        let fps = (fps_num as f64) / (fps_den.max(1) as f64);
        let fps_str = format!("{}/{}", fps_num, fps_den.max(1));
        let res_str = format!("{}x{}", width, height);

        // Find ffmpeg binary
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

        let vaapi_device = "/dev/dri/renderD128";
        let has_vaapi = Path::new(vaapi_device).exists();

        let mut cmd = Command::new(ffmpeg_bin);
        cmd.arg("-y")
            .arg("-f").arg("rawvideo")
            .arg("-pix_fmt").arg("rgba")
            .arg("-s").arg(&res_str)
            .arg("-r").arg(&fps_str)
            .arg("-i").arg("-");

        let sub_filter = if let Some(sub) = subtitle_path {
            if sub.exists() {
                let escaped = sub.to_string_lossy().replace(':', "\\:").replace('\'', "\\'");
                Some(format!("subtitles='{}'", escaped))
            } else {
                None
            }
        } else {
            None
        };

        if has_vaapi {
            log_msg(
                "ENCODER",
                &format!(
                    "Spawning FFmpeg VA-API hardware encoder on {} ({} @ {} fps -> {}){}",
                    vaapi_device, res_str, fps, target_path,
                    if sub_filter.is_some() { " with burned subtitles" } else { "" }
                ),
            );
            cmd.arg("-vaapi_device").arg(vaapi_device);
            if let Some(ref sf) = sub_filter {
                cmd.arg("-vf").arg(format!("{},format=nv12,hwupload", sf));
            } else {
                cmd.arg("-vf").arg("format=nv12,hwupload");
            }
            cmd.arg("-c:v").arg("h264_vaapi");
        } else {
            log_msg(
                "ENCODER",
                &format!(
                    "VA-API device not found; using fallback x264 encoder ({} @ {} fps -> {}){}",
                    res_str, fps, target_path,
                    if sub_filter.is_some() { " with burned subtitles" } else { "" }
                ),
            );
            if let Some(ref sf) = sub_filter {
                cmd.arg("-vf").arg(sf);
            }
            cmd.arg("-c:v").arg("libx264")
                .arg("-preset").arg("ultrafast")
                .arg("-pix_fmt").arg("yuv420p");
        }

        cmd.arg(target_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::piped());

        let mut child = cmd.spawn()?;
        let stdin = child.stdin.take().ok_or_else(|| {
            io::Error::new(io::ErrorKind::BrokenPipe, "Failed to capture ffmpeg stdin")
        })?;

        Ok(Self {
            child,
            stdin,
            width,
            height,
            fps,
            frames_written: 0,
            target: target_path.to_string(),
        })
    }

    pub fn write_frame(&mut self, data: &[u8]) -> io::Result<()> {
        self.stdin.write_all(data)?;
        self.frames_written += 1;
        Ok(())
    }

    pub fn finish(self) -> io::Result<i32> {
        drop(self.stdin); // Close pipe to signal EOF to FFmpeg
        let output = self.child.wait_with_output()?;
        let code = output.status.code().unwrap_or(0);
        if code != 0 {
            let stderr_str = String::from_utf8_lossy(&output.stderr);
            log_msg(
                "ENCODER",
                &format!("FFmpeg encoder exited with error code {}: {}", code, stderr_str),
            );
        } else {
            log_msg(
                "ENCODER",
                &format!(
                    "FFmpeg encoder completed successfully ({} frames written to {})",
                    self.frames_written, self.target
                ),
            );
        }
        Ok(code)
    }
}
