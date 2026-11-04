use std::collections::HashMap;
use std::path::{Path, PathBuf};
use roxmltree::Document;

#[derive(Debug, Clone)]
pub struct Profile {
    pub width: u32,
    pub height: u32,
    pub frame_rate_num: u32,
    pub frame_rate_den: u32,
    pub display_aspect_num: u32,
    pub display_aspect_den: u32,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            width: 1080,
            height: 1920,
            frame_rate_num: 25,
            frame_rate_den: 1,
            display_aspect_num: 9,
            display_aspect_den: 16,
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyframeRect {
    pub frame: u64,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub opacity: f32,
}

impl KeyframeRect {
    pub fn interpolate(keyframes: &[KeyframeRect], frame: u64) -> (f32, f32, f32, f32, f32) {
        if keyframes.is_empty() {
            return (0.0, 0.0, 1080.0, 1920.0, 1.0);
        }
        if keyframes.len() == 1 || frame <= keyframes[0].frame {
            let k = &keyframes[0];
            return (k.x, k.y, k.w, k.h, k.opacity);
        }
        let last = &keyframes[keyframes.len() - 1];
        if frame >= last.frame {
            return (last.x, last.y, last.w, last.h, last.opacity);
        }
        for i in 0..keyframes.len() - 1 {
            let k0 = &keyframes[i];
            let k1 = &keyframes[i + 1];
            if frame >= k0.frame && frame <= k1.frame {
                let span = (k1.frame - k0.frame) as f32;
                let factor = if span > 0.0 { (frame - k0.frame) as f32 / span } else { 0.0 };
                let x = k0.x + factor * (k1.x - k0.x);
                let y = k0.y + factor * (k1.y - k0.y);
                let w = k0.w + factor * (k1.w - k0.w);
                let h = k0.h + factor * (k1.h - k0.h);
                let opacity = k0.opacity + factor * (k1.opacity - k0.opacity);
                return (x, y, w, h, opacity);
            }
        }
        (last.x, last.y, last.w, last.h, last.opacity)
    }
}

#[derive(Debug, Clone)]
pub enum FilterType {
    QtBlend {
        rect_keyframes: Vec<KeyframeRect>,
        rotation_keyframes: Vec<(u64, f32)>,
        rotate_center: bool,
        distort: bool,
    },
    DropShadow {
        radius: f32,
        x_offset: f32,
        y_offset: f32,
        color: u32,
    },
    GaussianBlur {
        sigma: f32,
    },
    Tint {
        color: [u8; 4],
        amount: f32,
    },
    Subtitles(String),
    HorizontalFlip,
    Passthrough(String),
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub id: String,
    pub filter_type: FilterType,
}

#[derive(Debug, Clone)]
pub enum ProducerSource {
    Image(PathBuf),
    Color([u8; 4]),
    Video(PathBuf),
    MltXml(PathBuf),
    NestedTractor(String),
}

#[derive(Debug, Clone)]
pub struct Producer {
    pub id: String,
    pub source: ProducerSource,
    pub duration_frames: u64,
    pub filters: Vec<Filter>,
}

#[derive(Debug, Clone)]
pub enum PlaylistItem {
    Entry(PlaylistEntry),
    Blank(u64),
}

#[derive(Debug, Clone)]
pub struct PlaylistEntry {
    pub producer_id: String,
    pub in_frame: u64,
    pub out_frame: u64,
    pub filters: Vec<Filter>,
}

#[derive(Debug, Clone)]
pub struct Playlist {
    pub id: String,
    pub items: Vec<PlaylistItem>,
    pub is_audio: bool,
}

impl Playlist {
    pub fn get_active_clip_at(&self, timeline_frame: u64) -> Option<(&PlaylistEntry, u64)> {
        let mut current_frame = 0u64;
        for item in &self.items {
            match item {
                PlaylistItem::Blank(len) => {
                    if timeline_frame < current_frame + len {
                        return None;
                    }
                    current_frame += len;
                }
                PlaylistItem::Entry(entry) => {
                    let duration = if entry.out_frame >= entry.in_frame {
                        entry.out_frame - entry.in_frame + 1
                    } else {
                        1
                    };
                    if timeline_frame >= current_frame && timeline_frame < current_frame + duration {
                        let offset = timeline_frame - current_frame;
                        let clip_frame = entry.in_frame + offset;
                        return Some((entry, clip_frame));
                    }
                    current_frame += duration;
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct Track {
    pub producer_id: String,
    pub hide: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Tractor {
    pub id: String,
    pub in_frame: u64,
    pub out_frame: u64,
    pub tracks: Vec<Track>,
    pub filters: Vec<Filter>,
    pub is_audio: bool,
}

#[derive(Debug, Clone)]
pub struct Consumer {
    pub target: String,
    pub vcodec: String,
    pub acodec: String,
    pub in_frame: Option<u64>,
    pub out_frame: Option<u64>,
    pub extra_args: HashMap<String, String>,
}

#[derive(Debug, Clone, Default)]
pub struct MltProject {
    pub profile: Profile,
    pub root_tractor_id: Option<String>,
    pub producers: HashMap<String, Producer>,
    pub playlists: HashMap<String, Playlist>,
    pub tractors: HashMap<String, Tractor>,
    pub consumer: Option<Consumer>,
    pub root_dir: Option<PathBuf>,
}

impl MltProject {
    pub fn parse_color_str(s: &str) -> [u8; 4] {
        let s = s.trim();
        if s == "0" || s.eq_ignore_ascii_case("transparent") || s.eq_ignore_ascii_case("#00000000") {
            return [0, 0, 0, 0];
        }
        if s.eq_ignore_ascii_case("black") {
            return [0, 0, 0, 255];
        }
        if s.eq_ignore_ascii_case("white") {
            return [255, 255, 255, 255];
        }
        if let Some(hex) = s.strip_prefix("0x") {
            if let Ok(val) = u32::from_str_radix(hex, 16) {
                return [
                    ((val >> 24) & 0xFF) as u8,
                    ((val >> 16) & 0xFF) as u8,
                    ((val >> 8) & 0xFF) as u8,
                    (val & 0xFF) as u8,
                ];
            }
        }
        if let Some(hex) = s.strip_prefix("#") {
            if hex.len() == 6 {
                if let Ok(val) = u32::from_str_radix(hex, 16) {
                    return [
                        ((val >> 16) & 0xFF) as u8,
                        ((val >> 8) & 0xFF) as u8,
                        (val & 0xFF) as u8,
                        255,
                    ];
                }
            } else if hex.len() == 8 {
                if let Ok(val) = u32::from_str_radix(hex, 16) {
                    return [
                        ((val >> 24) & 0xFF) as u8,
                        ((val >> 16) & 0xFF) as u8,
                        ((val >> 8) & 0xFF) as u8,
                        (val & 0xFF) as u8,
                    ];
                }
            }
        }
        [0, 0, 0, 255]
    }

    pub fn parse_float_keyframes(s: &str, fps_num: u32, fps_den: u32) -> Vec<(u64, f32)> {
        let mut kfs = Vec::new();
        for part in s.split(';') {
            let part = part.trim();
            if part.is_empty() { continue; }
            let mut pieces = part.split('=');
            let time_part = pieces.next().unwrap_or("0").trim_end_matches('|');
            let frame = Self::parse_time_str(time_part, fps_num, fps_den);
            if let Some(val_str) = pieces.next() {
                if let Ok(v) = val_str.trim().parse::<f32>() {
                    kfs.push((frame, v));
                }
            }
        }
        if kfs.is_empty() {
            if let Ok(v) = s.trim().parse::<f32>() {
                kfs.push((0, v));
            }
        }
        kfs.sort_by_key(|k| k.0);
        kfs
    }

    pub fn interpolate_float(kfs: &[(u64, f32)], frame: u64) -> f32 {
        if kfs.is_empty() { return 0.0; }
        if kfs.len() == 1 || frame <= kfs[0].0 { return kfs[0].1; }
        let last = &kfs[kfs.len() - 1];
        if frame >= last.0 { return last.1; }
        for i in 0..kfs.len() - 1 {
            let (f0, v0) = kfs[i];
            let (f1, v1) = kfs[i + 1];
            if frame >= f0 && frame <= f1 {
                let span = (f1 - f0) as f32;
                let factor = if span > 0.0 { (frame - f0) as f32 / span } else { 0.0 };
                return v0 + factor * (v1 - v0);
            }
        }
        last.1
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(&path)?;
        let mut proj = Self::parse_xml(&content)?;
        if let Some(parent) = path.as_ref().parent() {
            proj.root_dir = Some(parent.to_path_buf());
        }
        Ok(proj)
    }

    pub fn parse(xml_content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Self::parse_xml(xml_content)
    }

    pub fn parse_xml(xml_content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let doc = Document::parse(xml_content)?;
        let mut project = MltProject::default();
        let mut last_tractor_id = None;

        for node in doc.descendants() {
            match node.tag_name().name() {
                "profile" => {
                    project.profile.width = node.attribute("width").and_then(|v| v.parse().ok()).unwrap_or(1080);
                    project.profile.height = node.attribute("height").and_then(|v| v.parse().ok()).unwrap_or(1920);
                    project.profile.frame_rate_num = node.attribute("frame_rate_num").and_then(|v| v.parse().ok()).unwrap_or(25);
                    project.profile.frame_rate_den = node.attribute("frame_rate_den").and_then(|v| v.parse().ok()).unwrap_or(1);
                    project.profile.display_aspect_num = node.attribute("display_aspect_num").and_then(|v| v.parse().ok()).unwrap_or(9);
                    project.profile.display_aspect_den = node.attribute("display_aspect_den").and_then(|v| v.parse().ok()).unwrap_or(16);
                }
                "producer" => {
                    if let Some(id) = node.attribute("id") {
                        let mut resource = String::new();
                        let mut service = String::new();

                        for prop in node.children().filter(|c| c.tag_name().name() == "property") {
                            if let Some(name) = prop.attribute("name") {
                                let val = prop.text().unwrap_or("").trim();
                                if name == "resource" {
                                    resource = val.to_string();
                                } else if name == "mlt_service" {
                                    service = val.to_string();
                                }
                            }
                        }

                        let is_image = service == "qimage"
                            || resource.ends_with(".png")
                            || resource.ends_with(".jpg")
                            || resource.ends_with(".jpeg");

                        let is_xml = service == "xml"
                            || resource.ends_with(".mlt")
                            || resource.ends_with(".kdenlive");

                        let source = if is_image {
                            ProducerSource::Image(PathBuf::from(resource))
                        } else if service == "color" {
                            ProducerSource::Color(Self::parse_color_str(&resource))
                        } else if is_xml {
                            ProducerSource::MltXml(PathBuf::from(resource))
                        } else {
                            ProducerSource::Video(PathBuf::from(resource))
                        };

                        let filters = Self::parse_filters(node, project.profile.frame_rate_num, project.profile.frame_rate_den);

                        project.producers.insert(id.to_string(), Producer {
                            id: id.to_string(),
                            source,
                            duration_frames: 1000,
                            filters,
                        });
                    }
                }
                "playlist" => {
                    if let Some(id) = node.attribute("id") {
                        let mut items = Vec::new();
                        let mut is_audio = false;

                        for prop in node.children().filter(|c| c.tag_name().name() == "property") {
                            if prop.attribute("name") == Some("kdenlive:audio_track") && prop.text().unwrap_or("").trim() == "1" {
                                is_audio = true;
                            }
                        }

                        for child in node.children() {
                            match child.tag_name().name() {
                                "blank" => {
                                    let length = child.attribute("length")
                                        .map(|s| Self::parse_time_str(s, project.profile.frame_rate_num, project.profile.frame_rate_den))
                                        .unwrap_or(0);
                                    if length > 0 {
                                        items.push(PlaylistItem::Blank(length));
                                    }
                                }
                                "entry" => {
                                    let producer_id = child.attribute("producer").unwrap_or("").to_string();
                                    let in_frame = child.attribute("in")
                                        .map(|s| Self::parse_time_str(s, project.profile.frame_rate_num, project.profile.frame_rate_den))
                                        .unwrap_or(0);
                                    let out_frame = child.attribute("out")
                                        .map(|s| Self::parse_time_str(s, project.profile.frame_rate_num, project.profile.frame_rate_den))
                                        .unwrap_or(0);
                                    let filters = Self::parse_filters(child, project.profile.frame_rate_num, project.profile.frame_rate_den);

                                    items.push(PlaylistItem::Entry(PlaylistEntry {
                                        producer_id,
                                        in_frame,
                                        out_frame,
                                        filters,
                                    }));
                                }
                                _ => {}
                            }
                        }
                        project.playlists.insert(id.to_string(), Playlist {
                            id: id.to_string(),
                            items,
                            is_audio,
                        });
                    }
                }
                "tractor" => {
                    if let Some(id) = node.attribute("id") {
                        let in_frame = node.attribute("in")
                            .map(|s| Self::parse_time_str(s, project.profile.frame_rate_num, project.profile.frame_rate_den))
                            .unwrap_or(0);
                        let out_frame = node.attribute("out")
                            .map(|s| Self::parse_time_str(s, project.profile.frame_rate_num, project.profile.frame_rate_den))
                            .unwrap_or(0);

                        let mut tracks = Vec::new();
                        for child in node.children() {
                            if child.tag_name().name() == "track" {
                                let producer_id = child.attribute("producer").unwrap_or("").to_string();
                                let hide = child.attribute("hide").map(|s| s.to_string());
                                tracks.push(Track { producer_id, hide });
                            }
                        }
                        let filters = Self::parse_filters(node, project.profile.frame_rate_num, project.profile.frame_rate_den);
                        let mut is_audio = false;

                        // Check properties
                        for prop in node.children().filter(|c| c.tag_name().name() == "property") {
                            if prop.attribute("name") == Some("kdenlive:projectTractor") {
                                project.root_tractor_id = Some(id.to_string());
                            } else if prop.attribute("name") == Some("kdenlive:audio_track") && prop.text().unwrap_or("").trim() == "1" {
                                is_audio = true;
                            }
                        }

                        last_tractor_id = Some(id.to_string());

                        project.tractors.insert(id.to_string(), Tractor {
                            id: id.to_string(),
                            in_frame,
                            out_frame,
                            tracks,
                            filters,
                            is_audio,
                        });
                    }
                }
                "consumer" => {
                    let mut target = node.attribute("target").unwrap_or("").to_string();
                    let mut in_frame = node.attribute("in")
                        .map(|s| Self::parse_time_str(s, project.profile.frame_rate_num, project.profile.frame_rate_den));
                    let mut out_frame = node.attribute("out")
                        .map(|s| Self::parse_time_str(s, project.profile.frame_rate_num, project.profile.frame_rate_den));
                    let mut vcodec = String::new();
                    let mut acodec = String::new();
                    let mut extra_args = HashMap::new();

                    for prop in node.children().filter(|c| c.tag_name().name() == "property") {
                        if let Some(name) = prop.attribute("name") {
                            let val = prop.text().unwrap_or("").trim().to_string();
                            match name {
                                "target" if target.is_empty() => target = val,
                                "vcodec" => vcodec = val,
                                "acodec" => acodec = val,
                                "in" if in_frame.is_none() => {
                                    in_frame = Some(Self::parse_time_str(&val, project.profile.frame_rate_num, project.profile.frame_rate_den));
                                }
                                "out" if out_frame.is_none() => {
                                    out_frame = Some(Self::parse_time_str(&val, project.profile.frame_rate_num, project.profile.frame_rate_den));
                                }
                                _ => {
                                    extra_args.insert(name.to_string(), val);
                                }
                            }
                        }
                    }

                    project.consumer = Some(Consumer {
                        target,
                        vcodec,
                        acodec,
                        in_frame,
                        out_frame,
                        extra_args,
                    });
                }
                _ => {}
            }
        }

        if project.root_tractor_id.is_none() {
            project.root_tractor_id = last_tractor_id;
        }

        Ok(project)
    }

    pub fn parse_time_str(time_str: &str, fps_num: u32, fps_den: u32) -> u64 {
        let s = time_str.trim();
        if s.is_empty() {
            return 0;
        }
        if let Ok(f) = s.parse::<u64>() {
            return f;
        }
        let fps = (fps_num as f64) / (fps_den.max(1) as f64);
        let colon_parts: Vec<&str> = s.split(':').collect();
        match colon_parts.len() {
            3 => {
                let h: f64 = colon_parts[0].parse().unwrap_or(0.0);
                let m: f64 = colon_parts[1].parse().unwrap_or(0.0);
                let s_val: f64 = colon_parts[2].parse().unwrap_or(0.0);
                let total_sec = h * 3600.0 + m * 60.0 + s_val;
                (total_sec * fps).round() as u64
            }
            4 => {
                let h: u64 = colon_parts[0].parse().unwrap_or(0);
                let m: u64 = colon_parts[1].parse().unwrap_or(0);
                let s_val: u64 = colon_parts[2].parse().unwrap_or(0);
                let f_val: u64 = colon_parts[3].parse().unwrap_or(0);
                let total_sec = (h * 3600 + m * 60 + s_val) as f64;
                (total_sec * fps).round() as u64 + f_val
            }
            _ => 0,
        }
    }

    pub fn get_total_frames(&self) -> u64 {
        if let Some(ref root_id) = self.root_tractor_id {
            if let Some(t) = self.tractors.get(root_id) {
                if t.out_frame > t.in_frame {
                    return t.out_frame - t.in_frame + 1;
                }
            }
        }
        self.tractors
            .values()
            .map(|t| if t.out_frame > t.in_frame { t.out_frame - t.in_frame + 1 } else { 0 })
            .max()
            .unwrap_or(100)
    }

    /// Resolves start and end frames for rendering based on CLI args, consumer in/out, and root tractor
    pub fn get_render_bounds(&self, cli_in: Option<u64>, cli_out: Option<u64>) -> (u64, u64) {
        let start = cli_in
            .or_else(|| self.consumer.as_ref().and_then(|c| c.in_frame))
            .or_else(|| self.root_tractor_id.as_ref().and_then(|id| self.tractors.get(id)).map(|t| t.in_frame))
            .unwrap_or(0);

        let end = cli_out
            .or_else(|| self.consumer.as_ref().and_then(|c| c.out_frame))
            .or_else(|| self.root_tractor_id.as_ref().and_then(|id| self.tractors.get(id)).and_then(|t| if t.out_frame > t.in_frame { Some(t.out_frame) } else { None }))
            .unwrap_or_else(|| start + self.get_total_frames().saturating_sub(1));

        (start, end)
    }

    /// Finds any active subtitle file attached to root or nested tractors
    pub fn get_subtitle_file(&self) -> Option<PathBuf> {
        // First check root tractor
        if let Some(ref root_id) = self.root_tractor_id {
            if let Some(root_t) = self.tractors.get(root_id) {
                for f in &root_t.filters {
                    if let FilterType::Subtitles(ref s) = f.filter_type {
                        if !s.is_empty() {
                            return Some(PathBuf::from(s));
                        }
                    }
                }
            }
        }
        // Check any tractor
        for t in self.tractors.values() {
            for f in &t.filters {
                if let FilterType::Subtitles(ref s) = f.filter_type {
                    if !s.is_empty() {
                        return Some(PathBuf::from(s));
                    }
                }
            }
        }
        None
    }

    pub fn get_consumer_target(&self) -> Option<String> {
        self.consumer.as_ref().and_then(|c| {
            if !c.target.is_empty() && c.target != "null" {
                Some(c.target.clone())
            } else {
                None
            }
        })
    }

    fn parse_filters(parent: roxmltree::Node, fps_num: u32, fps_den: u32) -> Vec<Filter> {
        let mut filters = Vec::new();
        for child in parent.children().filter(|c| c.tag_name().name() == "filter") {
            let id = child.attribute("id").unwrap_or("").to_string();
            let mut service = String::new();
            let mut rect_str = String::new();
            let mut rotation_str = String::new();
            let mut rotate_center = false;
            let mut distort = false;
            let mut radius_str = String::new();
            let mut sigma_str = String::new();
            let mut color_str = String::new();
            let mut tint_color_str = String::new();
            let mut tint_amount_str = String::new();
            let mut filename_str = String::new();
            let mut x_str = String::new();
            let mut y_str = String::new();
            let mut disabled = false;

            for prop in child.children().filter(|c| c.tag_name().name() == "property") {
                if let Some(name) = prop.attribute("name") {
                    let val = prop.text().unwrap_or("").trim();
                    match name {
                        "disable" if val == "1" => disabled = true,
                        "mlt_service" => service = val.to_string(),
                        "rect" => rect_str = val.to_string(),
                        "rotation" => rotation_str = val.to_string(),
                        "rotate_center" => rotate_center = val == "1",
                        "distort" => distort = val == "1",
                        "radius" => radius_str = val.to_string(),
                        "sigma" | "av.sigma" => sigma_str = val.to_string(),
                        "color" => color_str = val.to_string(),
                        "Map white to" | "Map black to" => tint_color_str = val.to_string(),
                        "Tint amount" => tint_amount_str = val.to_string(),
                        "av.filename" | "filename" => filename_str = val.to_string(),
                        "x" => x_str = val.to_string(),
                        "y" => y_str = val.to_string(),
                        _ => {}
                    }
                }
            }

            if disabled {
                continue;
            }

            let filter_type = if service == "qtblend" {
                let keyframes = Self::parse_rect_keyframes(&rect_str, fps_num, fps_den);
                let rotation_kfs = Self::parse_float_keyframes(&rotation_str, fps_num, fps_den);
                FilterType::QtBlend {
                    rect_keyframes: keyframes,
                    rotation_keyframes: rotation_kfs,
                    rotate_center,
                    distort,
                }
            } else if service == "dropshadow" {
                let radius = radius_str.split('=').last().unwrap_or("20").parse().unwrap_or(20.0);
                let x_offset = x_str.split('=').last().unwrap_or("4").parse().unwrap_or(4.0);
                let y_offset = y_str.split('=').last().unwrap_or("4").parse().unwrap_or(4.0);
                let raw_color = color_str.split('=').last().unwrap_or("0x000000FF");
                let color = if raw_color.starts_with("0x") {
                    u32::from_str_radix(&raw_color[2..], 16).unwrap_or(0x000000FF)
                } else {
                    0x000000FF
                };
                FilterType::DropShadow {
                    radius,
                    x_offset,
                    y_offset,
                    color,
                }
            } else if service == "avfilter.subtitles" || service == "subtitles" {
                FilterType::Subtitles(filename_str)
            } else if service == "frei0r.tint0r" {
                let raw_c = tint_color_str.split('=').last().unwrap_or("0x000000ff").trim();
                let color = Self::parse_color_str(raw_c);
                let amount = tint_amount_str.split('=').last().unwrap_or("1.0").trim().parse().unwrap_or(1.0);
                FilterType::Tint { color, amount }
            } else if service == "avfilter.hflip" {
                FilterType::HorizontalFlip
            } else if service.contains("gblur") {
                let s_str = if !sigma_str.is_empty() {
                    &sigma_str
                } else {
                    &radius_str
                };
                let sigma = s_str.split('=').last().unwrap_or("5.0").trim().parse().unwrap_or(5.0);
                FilterType::GaussianBlur { sigma }
            } else {
                FilterType::Passthrough(service.clone())
            };

            filters.push(Filter {
                id,
                filter_type,
            });
        }
        filters
    }

    fn parse_rect_keyframes(rect_str: &str, fps_num: u32, fps_den: u32) -> Vec<KeyframeRect> {
        let mut keyframes = Vec::new();
        for part in rect_str.split(';') {
            let part = part.trim();
            if part.is_empty() { continue; }
            let mut pieces = part.split('=');
            let time_part = pieces.next().unwrap_or("0").trim_end_matches('|');
            let frame = Self::parse_time_str(time_part, fps_num, fps_den);
            if let Some(coords) = pieces.next() {
                let nums: Vec<f32> = coords.split_whitespace().filter_map(|s| s.parse().ok()).collect();
                if nums.len() >= 4 {
                    keyframes.push(KeyframeRect {
                        frame,
                        x: nums[0],
                        y: nums[1],
                        w: nums[2],
                        h: nums[3],
                        opacity: if nums.len() >= 5 { nums[4] } else { 1.0 },
                    });
                }
            }
        }
        if keyframes.is_empty() {
            keyframes.push(KeyframeRect {
                frame: 0,
                x: 0.0,
                y: 0.0,
                w: 1080.0,
                h: 1920.0,
                opacity: 1.0,
            });
        }
        keyframes.sort_by_key(|k| k.frame);
        keyframes
    }
}
