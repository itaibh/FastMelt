use crate::logger::log_msg;
use libloading::Library;
use std::ffi::{c_char, c_int, c_void, CString};
use std::process::Command;

pub struct UpstreamMlt {
    _lib: Option<Library>,
}

pub struct UpstreamClipRenderer {
    profile: *mut c_void,
    producer: *mut c_void,
    consumer: *mut c_void,
}

impl UpstreamClipRenderer {
    pub fn new_for_clip(
        producer_id: &str,
        resource: &str,
        producer_filters: &[crate::parser::Filter],
        entry_filters: &[crate::parser::Filter],
        canvas_w: u32,
        canvas_h: u32,
    ) -> Option<Self> {
        log_msg("FALLBACK", &format!("UpstreamClipRenderer::new_for_clip called for producer: {} resource: {}", producer_id, resource));
        unsafe {
            crate::shim::fastmelt_init_dispatch();

            if !crate::shim::real_mlt_factory_init.is_null() {
                let factory_init: extern "C" fn(*const c_char) -> *mut c_void =
                    std::mem::transmute(crate::shim::real_mlt_factory_init);
                factory_init(std::ptr::null());
            }

            if crate::shim::real_mlt_factory_producer.is_null()
                || crate::shim::real_mlt_factory_consumer.is_null()
                || crate::shim::real_mlt_consumer_connect.is_null()
                || crate::shim::real_mlt_profile_init.is_null()
            {
                log_msg("FALLBACK", "Upstream symbols missing for UpstreamClipRenderer");
                return None;
            }

            let profile_init: extern "C" fn(*const c_char) -> *mut c_void =
                std::mem::transmute(crate::shim::real_mlt_profile_init);
            let profile = profile_init(std::ptr::null());
            if profile.is_null() {
                return None;
            }

            // Build minimal MLT XML with exact structure
            let mut xml = String::new();
            xml.push_str("<mlt>\n");
            xml.push_str(&format!(
                "  <profile colorspace=\"709\" description=\"Project Profile\" display_aspect_den=\"{}\" display_aspect_num=\"{}\" frame_rate_den=\"1\" frame_rate_num=\"25\" height=\"{}\" progressive=\"1\" sample_aspect_den=\"1\" sample_aspect_num=\"1\" width=\"{}\"/>\n",
                canvas_h, canvas_w, canvas_h, canvas_w
            ));
            xml.push_str(&format!("  <producer id=\"{}\" in=\"00:00:00.000\" out=\"00:10:00.000\">\n", producer_id));
            xml.push_str(&format!("    <property name=\"resource\">{}</property>\n", resource));
            for f in producer_filters {
                xml.push_str(&format!("    <filter id=\"{}\">\n", f.id));
                for (k, v) in &f.properties {
                    let escaped = v.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");
                    xml.push_str(&format!("      <property name=\"{}\">{}</property>\n", k, escaped));
                }
                xml.push_str("    </filter>\n");
            }
            xml.push_str("  </producer>\n");
            xml.push_str("  <playlist id=\"pl_clip\">\n");
            xml.push_str(&format!("    <entry in=\"00:00:00.000\" out=\"00:10:00.000\" producer=\"{}\">\n", producer_id));
            for f in entry_filters {
                xml.push_str(&format!("      <filter id=\"{}\">\n", f.id));
                for (k, v) in &f.properties {
                    let escaped = v.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");
                    xml.push_str(&format!("        <property name=\"{}\">{}</property>\n", k, escaped));
                }
                xml.push_str("      </filter>\n");
            }
            xml.push_str("    </entry>\n");
            xml.push_str("  </playlist>\n");
            xml.push_str("</mlt>\n");

            let safe_id: String = producer_id.chars().map(|c| if c.is_alphanumeric() { c } else { '_' }).collect();
            let xml_path = format!("/tmp/fastmelt_clip_{}_{}.xml", std::process::id(), safe_id);
            if let Err(e) = std::fs::write(&xml_path, &xml) {
                log_msg("FALLBACK", &format!("Failed to write clip XML: {}", e));
                return None;
            }

            let factory_producer: extern "C" fn(*mut c_void, *const c_char, *const c_char) -> *mut c_void =
                std::mem::transmute(crate::shim::real_mlt_factory_producer);
            let factory_consumer: extern "C" fn(*mut c_void, *const c_char, *const c_char) -> *mut c_void =
                std::mem::transmute(crate::shim::real_mlt_factory_consumer);
            let consumer_connect: extern "C" fn(*mut c_void, *mut c_void) -> c_int =
                std::mem::transmute(crate::shim::real_mlt_consumer_connect);

            let serv_c = CString::new("xml").ok()?;
            let res_c = CString::new(xml_path.as_str()).ok()?;
            let producer = factory_producer(profile, serv_c.as_ptr(), res_c.as_ptr());
            if producer.is_null() {
                log_msg("FALLBACK", "Failed to create XML producer for clip");
                return None;
            }

            let cbrts_c = CString::new("cbrts").ok()?;
            let consumer = factory_consumer(profile, cbrts_c.as_ptr(), std::ptr::null());
            if consumer.is_null() {
                return None;
            }

            consumer_connect(consumer, producer);
            Some(Self {
                profile,
                producer,
                consumer,
            })
        }
    }

    pub fn new(resource: &str, filters: &[crate::parser::Filter]) -> Option<Self> {
        log_msg("FALLBACK", &format!("UpstreamClipRenderer::new called for resource: {} with {} filters", resource, filters.len()));
        unsafe {
            crate::shim::fastmelt_init_dispatch();

            if !crate::shim::real_mlt_factory_init.is_null() {
                let factory_init: extern "C" fn(*const c_char) -> *mut c_void =
                    std::mem::transmute(crate::shim::real_mlt_factory_init);
                factory_init(std::ptr::null());
            }

            if crate::shim::real_mlt_factory_producer.is_null()
                || crate::shim::real_mlt_factory_consumer.is_null()
                || crate::shim::real_mlt_consumer_connect.is_null()
            {
                log_msg("FALLBACK", "Upstream symbols missing for UpstreamClipRenderer");
                return None;
            }

            let factory_producer: extern "C" fn(*mut c_void, *const c_char, *const c_char) -> *mut c_void =
                std::mem::transmute(crate::shim::real_mlt_factory_producer);
            let factory_filter: extern "C" fn(*mut c_void, *const c_char, *const c_char) -> *mut c_void =
                std::mem::transmute(crate::shim::real_mlt_factory_filter);
            let service_attach: extern "C" fn(*mut c_void, *mut c_void) -> c_int =
                std::mem::transmute(crate::shim::real_mlt_service_attach);
            let filter_props: extern "C" fn(*mut c_void) -> *mut c_void =
                std::mem::transmute(crate::shim::real_mlt_filter_properties);
            let props_set: extern "C" fn(*mut c_void, *const c_char, *const c_char) -> c_int =
                std::mem::transmute(crate::shim::real_mlt_properties_set);
            let factory_consumer: extern "C" fn(*mut c_void, *const c_char, *const c_char) -> *mut c_void =
                std::mem::transmute(crate::shim::real_mlt_factory_consumer);
            let consumer_connect: extern "C" fn(*mut c_void, *mut c_void) -> c_int =
                std::mem::transmute(crate::shim::real_mlt_consumer_connect);

            let profile_init: extern "C" fn(*const c_char) -> *mut c_void =
                std::mem::transmute(crate::shim::real_mlt_profile_init);
            let profile = profile_init(std::ptr::null());
            if profile.is_null() {
                return None;
            }

            let service_type = if resource.ends_with(".png") || resource.ends_with(".jpg") || resource.ends_with(".jpeg") {
                "qimage"
            } else if resource.ends_with(".mlt") || resource.ends_with(".kdenlive") {
                "xml"
            } else {
                "avformat"
            };

            let serv_c = CString::new(service_type).ok()?;
            let res_c = CString::new(resource).ok()?;
            let producer = factory_producer(profile, serv_c.as_ptr(), res_c.as_ptr());
            if producer.is_null() {
                return None;
            }

            // Attach all filters in sequence (transforms and effect/passthrough filters)
            for f in filters {
                let service_str = f.properties.get("mlt_service").map(|s| s.as_str()).unwrap_or_else(|| {
                    match &f.filter_type {
                        crate::parser::FilterType::QtBlend { .. } => "qtblend",
                        crate::parser::FilterType::Passthrough { service, .. } => service.as_str(),
                        crate::parser::FilterType::DropShadow { .. } => "drop_shadow",
                        crate::parser::FilterType::GaussianBlur { .. } => "boxblur",
                        _ => "",
                    }
                });

                if !service_str.is_empty() {
                    if let Ok(service_c) = CString::new(service_str) {
                        let filter_obj = factory_filter(profile, service_c.as_ptr(), std::ptr::null());
                        if !filter_obj.is_null() {
                            let fp = filter_props(filter_obj);
                            for (k, v) in &f.properties {
                                if let (Ok(kc), Ok(vc)) = (CString::new(k.as_str()), CString::new(v.as_str())) {
                                    props_set(fp, kc.as_ptr(), vc.as_ptr());
                                }
                            }
                            service_attach(producer, filter_obj);
                        }
                    }
                }
            }

            let cbrts_c = CString::new("cbrts").ok()?;
            let consumer = factory_consumer(profile, cbrts_c.as_ptr(), std::ptr::null());
            if consumer.is_null() {
                return None;
            }

            consumer_connect(consumer, producer);
            Some(Self {
                profile,
                producer,
                consumer,
            })
        }
    }

    /// Render a frame of this clip and return an RGBA image buffer (width, height, raw RGBA bytes)
    pub fn render_frame(&self, clip_frame: u64, target_w: u32, target_h: u32) -> Option<(u32, u32, Vec<u8>)> {
        unsafe {
            if self.consumer.is_null()
                || crate::shim::real_mlt_consumer_get_frame.is_null()
                || crate::shim::real_mlt_frame_get_image.is_null()
                || crate::shim::real_mlt_frame_close.is_null()
            {
                return None;
            }

            if !crate::shim::real_mlt_producer_seek.is_null() && !self.producer.is_null() {
                let producer_seek: extern "C" fn(*mut c_void, c_int) -> c_int =
                    std::mem::transmute(crate::shim::real_mlt_producer_seek);
                producer_seek(self.producer, clip_frame as c_int);
            }

            let consumer_get_frame: extern "C" fn(*mut c_void) -> *mut c_void =
                std::mem::transmute(crate::shim::real_mlt_consumer_get_frame);
            let frame_get_image: extern "C" fn(
                *mut c_void,
                *mut *mut u8,
                *mut c_int,
                *mut c_int,
                *mut c_int,
                c_int,
            ) -> c_int = std::mem::transmute(crate::shim::real_mlt_frame_get_image);
            let frame_close: extern "C" fn(*mut c_void) =
                std::mem::transmute(crate::shim::real_mlt_frame_close);

            let frame = consumer_get_frame(self.consumer);
            if frame.is_null() {
                return None;
            }

            let mut buffer: *mut u8 = std::ptr::null_mut();
            let mut fmt: c_int = 2; // mlt_image_rgb24a (RGBA)
            let mut w: c_int = target_w as c_int;
            let mut h: c_int = target_h as c_int;

            let ret = frame_get_image(frame, &mut buffer, &mut fmt, &mut w, &mut h, 0);
            if ret != 0 || buffer.is_null() || w <= 0 || h <= 0 {
                frame_close(frame);
                return None;
            }

            let len = (w as usize) * (h as usize) * 4;
            let slice = std::slice::from_raw_parts(buffer, len);
            let image_data = slice.to_vec();

            frame_close(frame);
            Some((w as u32, h as u32, image_data))
        }
    }
}

impl Drop for UpstreamClipRenderer {
    fn drop(&mut self) {
        unsafe {
            if !self.consumer.is_null() && !crate::shim::real_mlt_consumer_close.is_null() {
                let consumer_close: extern "C" fn(*mut c_void) =
                    std::mem::transmute(crate::shim::real_mlt_consumer_close);
                consumer_close(self.consumer);
            }
            if !self.producer.is_null() && !crate::shim::real_mlt_producer_close.is_null() {
                let producer_close: extern "C" fn(*mut c_void) =
                    std::mem::transmute(crate::shim::real_mlt_producer_close);
                producer_close(self.producer);
            }
        }
    }
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

