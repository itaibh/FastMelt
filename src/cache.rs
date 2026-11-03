use std::collections::HashMap;
use std::path::{Path, PathBuf};
use image::RgbaImage;
use crate::logger::log_msg;

pub struct TextureCache {
    cache: HashMap<PathBuf, RgbaImage>,
}

impl TextureCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
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
}

impl Default for TextureCache {
    fn default() -> Self {
        Self::new()
    }
}
