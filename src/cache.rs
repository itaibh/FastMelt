use std::collections::HashMap;
use std::path::{Path, PathBuf};
use image::RgbaImage;
use crate::logger::log_msg;
use crate::pipeline::FrameBuffer;

pub struct TextureCache {
    cache: HashMap<PathBuf, RgbaImage>,
    blurred_cache: HashMap<(PathBuf, u32), RgbaImage>,
}

impl TextureCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            blurred_cache: HashMap::new(),
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

    pub fn get_or_load_blurred<P: AsRef<Path>>(&mut self, path: P, sigma: f32) -> Option<&RgbaImage> {
        let p = path.as_ref().to_path_buf();
        let key_sigma = (sigma * 10.0).round() as u32;
        let key = (p.clone(), key_sigma);

        if !self.blurred_cache.contains_key(&key) {
            let base_img = self.get_or_load(&p)?.clone();
            let (w, h) = (base_img.width(), base_img.height());
            let mut raw = base_img.into_raw();
            let mut scratch = vec![0u8; raw.len()];
            FrameBuffer::apply_separable_blur_buffer(&mut raw, &mut scratch, w, h, sigma);
            if let Some(blurred_img) = RgbaImage::from_raw(w, h, raw) {
                log_msg("CACHE", &format!("Cached blurred texture for {} (sigma={})", p.display(), sigma));
                self.blurred_cache.insert(key.clone(), blurred_img);
            } else {
                return self.cache.get(&p);
            }
        }
        self.blurred_cache.get(&key)
    }
}

impl Default for TextureCache {
    fn default() -> Self {
        Self::new()
    }
}
