use std::collections::HashMap;
use std::path::PathBuf;

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
        rotation: f32,
    },
    DropShadow {
        radius: f32,
        x_offset: f32,
        y_offset: f32,
        color: u32, // RGBA
    },
    GaussianBlur {
        sigma: f32,
    },
    Passthrough(String),
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub id: String,
    pub service: String,
    pub filter_type: FilterType,
}

#[derive(Debug, Clone)]
pub enum ProducerSource {
    Image(PathBuf),
    Video(PathBuf),
    Color(u32),
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
                        let local_offset = timeline_frame - current_frame;
                        let clip_frame = entry.in_frame + local_offset;
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
}

#[derive(Debug, Clone)]
pub struct Consumer {
    pub target: String,
    pub vcodec: String,
    pub acodec: String,
    pub extra_args: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct MltProject {
    pub root_dir: Option<PathBuf>,
    pub profile: Profile,
    pub producers: HashMap<String, Producer>,
    pub playlists: HashMap<String, Playlist>,
    pub tractors: HashMap<String, Tractor>,
    pub root_tractor_id: Option<String>,
    pub consumer: Option<Consumer>,
}

impl MltProject {
    pub fn parse(xml_content: &str) -> Result<Self, String> {
        let doc = roxmltree::Document::parse(xml_content)
            .map_err(|e| format!("Failed to parse XML: {}", e))?;

        let root_dir = doc.root_element().attribute("root").map(PathBuf::from);

        let mut project = MltProject {
            root_dir,
            profile: Profile::default(),
            producers: HashMap::new(),
            playlists: HashMap::new(),
            tractors: HashMap::new(),
            root_tractor_id: None,
            consumer: None,
        };

        let mut last_tractor_id = None;

        for node in doc.descendants() {
            match node.tag_name().name() {
                "profile" => {
                    let w = node.attribute("width").and_then(|v| v.parse().ok()).unwrap_or(1080);
                    let h = node.attribute("height").and_then(|v| v.parse().ok()).unwrap_or(1920);
                    let fr_num = node.attribute("frame_rate_num").and_then(|v| v.parse().ok()).unwrap_or(25);
                    let fr_den = node.attribute("frame_rate_den").and_then(|v| v.parse().ok()).unwrap_or(1);
                    project.profile = Profile {
                        width: w,
                        height: h,
                        frame_rate_num: fr_num,
                        frame_rate_den: fr_den,
                        display_aspect_num: 9,
                        display_aspect_den: 16,
                    };
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

                        let source = if service == "qimage" || resource.ends_with(".png") || resource.ends_with(".jpg") {
                            ProducerSource::Image(PathBuf::from(resource))
                        } else if service == "color" {
                            ProducerSource::Color(0xFF000000)
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
                        for child in node.children() {
                            if child.tag_name().name() == "entry" {
                                let producer_id = child.attribute("producer").unwrap_or("").to_string();
                                let in_frame = child.attribute("in")
                                    .map(|s| Self::parse_time_str(s, project.profile.frame_rate_num, project.profile.frame_rate_den))
                                    .unwrap_or(0);
                                let out_frame = child.attribute("out")
                                    .map(|s| Self::parse_time_str(s, project.profile.frame_rate_num, project.profile.frame_rate_den))
                                    .unwrap_or(in_frame);
                                let filters = Self::parse_filters(child, project.profile.frame_rate_num, project.profile.frame_rate_den);
                                items.push(PlaylistItem::Entry(PlaylistEntry {
                                    producer_id,
                                    in_frame,
                                    out_frame,
                                    filters,
                                }));
                            } else if child.tag_name().name() == "blank" {
                                let length = child.attribute("length")
                                    .map(|s| Self::parse_time_str(s, project.profile.frame_rate_num, project.profile.frame_rate_den))
                                    .unwrap_or(0);
                                if length > 0 {
                                    items.push(PlaylistItem::Blank(length));
                                }
                            }
                        }
                        project.playlists.insert(id.to_string(), Playlist {
                            id: id.to_string(),
                            items,
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
                        
                        // Check if it's the root project tractor
                        for prop in node.children().filter(|c| c.tag_name().name() == "property") {
                            if prop.attribute("name") == Some("kdenlive:projectTractor") {
                                project.root_tractor_id = Some(id.to_string());
                            }
                        }

                        last_tractor_id = Some(id.to_string());

                        project.tractors.insert(id.to_string(), Tractor {
                            id: id.to_string(),
                            in_frame,
                            out_frame,
                            tracks,
                            filters,
                        });
                    }
                }
                "consumer" => {
                    let mut target = node.attribute("target").unwrap_or("").to_string();
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
                // HH:MM:SS or HH:MM:SS.mmm
                let h: f64 = colon_parts[0].parse().unwrap_or(0.0);
                let m: f64 = colon_parts[1].parse().unwrap_or(0.0);
                let s_val: f64 = colon_parts[2].parse().unwrap_or(0.0);
                let total_sec = h * 3600.0 + m * 60.0 + s_val;
                (total_sec * fps).round() as u64
            }
            4 => {
                // HH:MM:SS:FF
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
            let mut radius_str = String::new();
            let mut color_str = String::new();
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
                        "radius" => radius_str = val.to_string(),
                        "color" => color_str = val.to_string(),
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
                FilterType::QtBlend {
                    rect_keyframes: keyframes,
                    rotation: 0.0,
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
            } else if service.contains("gblur") {
                let sigma = radius_str.split('=').last().unwrap_or("5").parse().unwrap_or(5.0);
                FilterType::GaussianBlur { sigma }
            } else {
                FilterType::Passthrough(service.clone())
            };

            filters.push(Filter {
                id,
                service,
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
                        opacity: if nums.len() > 4 { nums[4] } else { 1.0 },
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
