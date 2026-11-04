use std::collections::HashMap;
use std::path::{Path, PathBuf};
use image::RgbaImage;
use crate::logger::log_msg;
use crate::pipeline::FrameBuffer;

pub struct TextureCache {
    cache: HashMap<PathBuf, RgbaImage>,
    processed_cache: HashMap<(PathBuf, Option<[u8; 4]>, u32, bool), RgbaImage>,
    luma_cache: HashMap<PathBuf, (u32, u32, Vec<u8>)>,
    passthrough_cache: HashMap<(String, u64), RgbaImage>,
}

impl TextureCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            processed_cache: HashMap::new(),
            luma_cache: HashMap::new(),
            passthrough_cache: HashMap::new(),
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

    pub fn get_or_render_passthrough(
        &mut self,
        profile: &crate::parser::Profile,
        producer: &crate::parser::Producer,
        extra_filters: &[crate::parser::Filter],
        clip_frame: u64,
        root_dir: Option<&Path>,
    ) -> Option<&RgbaImage> {
        let is_static = producer.is_static() && extra_filters.is_empty();
        let cache_frame = if is_static { 0 } else { clip_frame };
        let cache_key = (producer.id.clone(), cache_frame);

        if !self.passthrough_cache.contains_key(&cache_key) {
            let extra_filters_xml: Vec<String> = extra_filters.iter().map(|f| f.raw_xml.clone()).collect();
            if let Some(rendered) = crate::fallback::UpstreamMlt::render_producer_frame(
                profile,
                &producer.raw_xml,
                &extra_filters_xml,
                &producer.id,
                clip_frame,
                root_dir,
            ) {
                log_msg("CACHE", &format!("Cached passthrough frame for producer '{}' (frame {}, is_static={})", producer.id, cache_frame, is_static));
                self.passthrough_cache.insert(cache_key.clone(), rendered);
            } else {
                return None;
            }
        }
        self.passthrough_cache.get(&cache_key)
    }
}

impl Default for TextureCache {
    fn default() -> Self {
        Self::new()
    }
}
