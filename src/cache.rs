use std::collections::HashMap;
use std::path::{Path, PathBuf};
use image::RgbaImage;
use crate::logger::log_msg;
use crate::pipeline::FrameBuffer;

pub struct TextureCache {
    cache: HashMap<PathBuf, RgbaImage>,
    processed_cache: HashMap<(PathBuf, Option<[u8; 4]>, u32, bool), RgbaImage>,
}

impl TextureCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            processed_cache: HashMap::new(),
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
}

impl Default for TextureCache {
    fn default() -> Self {
        Self::new()
    }
}
