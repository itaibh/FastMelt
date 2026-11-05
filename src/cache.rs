use std::collections::HashMap;
use std::path::{Path, PathBuf};
use image::RgbaImage;
use crate::logger::log_msg;
use crate::pipeline::FrameBuffer;

pub struct TextureCache {
    cache: HashMap<PathBuf, RgbaImage>,
    processed_cache: HashMap<(PathBuf, Option<[u8; 4]>, u32, bool), RgbaImage>,
    luma_cache: HashMap<PathBuf, (u32, u32, Vec<u8>)>,
    sequence_dir_cache: HashMap<PathBuf, Vec<PathBuf>>,
    video_frames: HashMap<(PathBuf, u64), RgbaImage>,
    video_info: HashMap<PathBuf, (u32, u32)>,
}

impl TextureCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            processed_cache: HashMap::new(),
            luma_cache: HashMap::new(),
            sequence_dir_cache: HashMap::new(),
            video_frames: HashMap::new(),
            video_info: HashMap::new(),
        }
    }

    pub fn get_or_load<P: AsRef<Path>>(&mut self, path: P) -> Option<&RgbaImage> {
        let p = path.as_ref().to_path_buf();
        if !self.cache.contains_key(&p) {
            if !p.exists() {
                log_msg("CACHE", &format!("Texture file not found: {}", p.display()));
                return None;
            }
            match image::open(&p) {
                Ok(img) => {
                    log_msg("CACHE", &format!("Loaded texture: {} ({}x{})", p.display(), img.width(), img.height()));
                    self.cache.insert(p.clone(), img.to_rgba8());
                }
                Err(e) => {
                    log_msg("CACHE", &format!("Failed to load texture '{}': {}", p.display(), e));
                    return None;
                }
            }
        }
        self.cache.get(&p)
    }

    pub fn get_or_load_processed<P: AsRef<Path>>(
        &mut self,
        path: P,
        tint: Option<[u8; 4]>,
        sigma: f32,
        hflip: bool,
    ) -> Option<&RgbaImage> {
        let p = path.as_ref().to_path_buf();
        let key_sigma = (sigma * 10.0).round() as u32;
        let key = (p.clone(), tint, key_sigma, hflip);

        if !self.processed_cache.contains_key(&key) {
            let mut img = self.get_or_load(&p)?.clone();

            // 1. Horizontal flip
            if hflip {
                let w = img.width() as usize;
                let h = img.height() as usize;
                let raw = img.as_mut();
                for y in 0..h {
                    let row_start = y * w * 4;
                    for x in 0..(w / 2) {
                        let i1 = row_start + x * 4;
                        let i2 = row_start + (w - 1 - x) * 4;
                        for c in 0..4 {
                            raw.swap(i1 + c, i2 + c);
                        }
                    }
                }
            }

            // 2. Tint
            if let Some(c) = tint {
                for pixel in img.pixels_mut() {
                    if pixel[3] > 0 {
                        pixel[0] = c[0];
                        pixel[1] = c[1];
                        pixel[2] = c[2];
                    }
                }
            }

            // 3. Gaussian Blur
            if sigma >= 0.5 {
                let (w, h) = (img.width(), img.height());
                let mut raw = img.into_raw();
                let mut scratch = vec![0u8; raw.len()];
                FrameBuffer::apply_separable_blur_buffer(&mut raw, &mut scratch, w, h, sigma);
                if let Some(blurred_img) = RgbaImage::from_raw(w, h, raw) {
                    img = blurred_img;
                } else {
                    return self.cache.get(&p);
                }
            }

            log_msg("CACHE", &format!("Cached processed texture for {} (tint={:?}, sigma={}, hflip={})", p.display(), tint, sigma, hflip));
            self.processed_cache.insert(key.clone(), img);
        }

        self.processed_cache.get(&key)
    }

    pub fn get_or_load_luma(&mut self, path_str: &str) -> Option<&(u32, u32, Vec<u8>)> {
        let candidates = [
            PathBuf::from(path_str),
            if path_str.starts_with("/app/") {
                PathBuf::from(path_str.replacen("/app/", "/var/lib/flatpak/app/org.kde.kdenlive/current/active/files/", 1))
            } else {
                PathBuf::from(path_str)
            },
            PathBuf::from(format!("/usr/share/kdenlive/lumas/{}", path_str.split('/').last().unwrap_or(""))),
        ];

        let p = candidates.into_iter().find(|p| p.exists())?;

        if !self.luma_cache.contains_key(&p) {
            match image::open(&p) {
                Ok(img) => {
                    let gray = img.to_luma8();
                    let w = gray.width();
                    let h = gray.height();
                    log_msg("CACHE", &format!("Loaded luma wipe map: {} ({}x{})", p.display(), w, h));
                    self.luma_cache.insert(p.clone(), (w, h, gray.into_raw()));
                }
                Err(e) => {
                    log_msg("CACHE", &format!("Failed to load luma map '{}': {}", p.display(), e));
                    return None;
                }
            }
        }
        self.luma_cache.get(&p)
    }

    pub fn is_sequence_pattern<P: AsRef<Path>>(path: P) -> bool {
        let p = path.as_ref();
        if let Some(name) = p.file_name().and_then(|s| s.to_str()) {
            name.starts_with(".all.") || name == ".all.png"
        } else {
            false
        }
    }

    pub fn get_or_load_sequence_frame<P: AsRef<Path>>(
        &mut self,
        seq_path: P,
        frame_idx: u64,
        tint: Option<[u8; 4]>,
        sigma: f32,
        hflip: bool,
    ) -> Option<&RgbaImage> {
        let p = seq_path.as_ref();
        let dir = p.parent()?;
        let ext = p.extension()?.to_str()?.to_lowercase();

        let files = self.sequence_dir_cache.entry(dir.to_path_buf()).or_insert_with(|| {
            let mut list = Vec::new();
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let ep = entry.path();
                    if ep.is_file() {
                        if let Some(e) = ep.extension().and_then(|s| s.to_str()) {
                            if e.to_lowercase() == ext && ep.file_name() != p.file_name() {
                                list.push(ep);
                            }
                        }
                    }
                }
            }
            list.sort();
            list
        });

        if files.is_empty() {
            log_msg("CACHE", &format!("No frames found for image sequence in dir: {}", dir.display()));
            return None;
        }

        let idx = (frame_idx as usize).min(files.len() - 1);
        let frame_path = files[idx].clone();
        self.get_or_load_processed(&frame_path, tint, sigma, hflip)
    }

    pub fn get_or_load_video_frame(
        &mut self,
        video_path: &Path,
        frame_idx: u64,
        fps_num: u32,
        fps_den: u32,
    ) -> Option<&RgbaImage> {
        let key = (video_path.to_path_buf(), frame_idx);
        if self.video_frames.contains_key(&key) {
            return self.video_frames.get(&key);
        }

        let (w, h) = match self.video_info.get(video_path) {
            Some(&dims) => dims,
            None => {
                let output = std::process::Command::new("ffprobe")
                    .args([
                        "-v", "error",
                        "-select_streams", "v:0",
                        "-show_entries", "stream=width,height",
                        "-of", "csv=s=x:p=0",
                    ])
                    .arg(video_path)
                    .output()
                    .ok()?;

                let s = String::from_utf8_lossy(&output.stdout);
                let parts: Vec<&str> = s.trim().split('x').collect();
                if parts.len() == 2 {
                    let w: u32 = parts[0].parse().ok()?;
                    let h: u32 = parts[1].parse().ok()?;
                    self.video_info.insert(video_path.to_path_buf(), (w, h));
                    (w, h)
                } else {
                    log_msg("CACHE", &format!("ffprobe failed to get dimensions for {}", video_path.display()));
                    return None;
                }
            }
        };

        let start_sec = (frame_idx as f64) * (fps_den as f64) / (fps_num as f64);
        let batch_count = 50;

        let output = std::process::Command::new("ffmpeg")
            .args([
                "-v", "error",
                "-ss", &format!("{:.3}", start_sec),
                "-i",
            ])
            .arg(video_path)
            .args([
                "-r", &format!("{}/{}", fps_num, fps_den),
                "-frames:v", &format!("{}", batch_count),
                "-f", "rawvideo",
                "-pix_fmt", "rgba",
                "pipe:1",
            ])
            .output()
            .ok()?;

        let frame_bytes = (w as usize) * (h as usize) * 4;
        let stdout = output.stdout;
        let num_decoded = stdout.len() / frame_bytes;

        for i in 0..num_decoded {
            let offset = i * frame_bytes;
            let slice = stdout[offset..offset + frame_bytes].to_vec();
            if let Some(img) = RgbaImage::from_raw(w, h, slice) {
                self.video_frames.insert((video_path.to_path_buf(), frame_idx + i as u64), img);
            }
        }

        self.video_frames.get(&key)
    }
}

impl Default for TextureCache {
    fn default() -> Self {
        Self::new()
    }
}
