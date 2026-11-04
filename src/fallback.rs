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

    /// Renders a single frame for an isolated producer and its filters into an RgbaImage using upstream melt.
    pub fn render_producer_frame(
        profile: &crate::parser::Profile,
        producer_xml: &str,
        extra_filters_xml: &[String],
        producer_id: &str,
        frame: u64,
        root_dir: Option<&std::path::Path>,
    ) -> Option<image::RgbaImage> {
        let tmp_dir = std::env::temp_dir();
        let unique_id = format!("mlt_pt_{}_{}_{}", std::process::id(), producer_id, frame);
        let tmp_xml_path = tmp_dir.join(format!("{}.mlt", unique_id));

        let filters_block = extra_filters_xml.join("\n");
        let xml_content = format!(
            r#"<mlt>
  <profile width="{w}" height="{h}" frame_rate_num="{fps_n}" frame_rate_den="{fps_d}" display_aspect_num="{dar_n}" display_aspect_den="{dar_d}" />
  {prod_xml}
  <playlist id="pt_pl">
    <entry producer="{prod_id}" in="{frame}" out="{frame}">
      {filters_block}
    </entry>
  </playlist>
  <tractor id="pt_root">
    <track producer="pt_pl"/>
  </tractor>
</mlt>"#,
            w = profile.width,
            h = profile.height,
            fps_n = profile.frame_rate_num,
            fps_d = profile.frame_rate_den,
            dar_n = profile.display_aspect_num,
            dar_d = profile.display_aspect_den,
            prod_xml = producer_xml,
            prod_id = producer_id,
            frame = frame,
            filters_block = filters_block
        );

        if std::fs::write(&tmp_xml_path, xml_content).is_err() {
            return None;
        }

        let args = [
            tmp_xml_path.to_str()?.to_string(),
            "-consumer".to_string(),
            "avformat:pipe:1".to_string(),
            "f=rawvideo".to_string(),
            "pix_fmt=rgba".to_string(),
        ];

        let mut cmd = Self::create_stock_melt_command(&args);
        if let Some(dir) = root_dir {
            cmd.current_dir(dir);
        }

        log_msg("FALLBACK", &format!("Invoking upstream passthrough renderer for producer '{}' at frame {}", producer_id, frame));
        let output = cmd.output().ok();
        let _ = std::fs::remove_file(&tmp_xml_path);

        let out = output?;
        let expected_size = (profile.width as usize) * (profile.height as usize) * 4;
        if out.stdout.len() >= expected_size {
            let frame_bytes = out.stdout[..expected_size].to_vec();
            image::RgbaImage::from_raw(profile.width, profile.height, frame_bytes)
        } else {
            log_msg("FALLBACK", &format!(
                "Upstream melt pipe returned {} bytes for frame {}, expected {}",
                out.stdout.len(), frame, expected_size
            ));
            None
        }
    }

}

