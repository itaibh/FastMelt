use std::path::PathBuf;
use crate::cache::TextureCache;
use crate::parser::{FilterType, KeyframeRect, MltProject, Playlist, ProducerSource};
use crate::pipeline::FrameBuffer;

pub struct TimelineEvaluator<'a> {
    project: &'a MltProject,
    cache: &'a mut TextureCache,
}

impl<'a> TimelineEvaluator<'a> {
    pub fn new(project: &'a MltProject, cache: &'a mut TextureCache) -> Self {
        Self { project, cache }
    }

    pub fn evaluate_frame(&mut self, frame: u64, fb: &mut FrameBuffer) {
        // Clear background with black opaque color
        fb.clear(image::Rgba([0, 0, 0, 255]));

        if let Some(ref root_id) = self.project.root_tractor_id {
            self.render_tractor(root_id, frame, fb, 0);
        }
    }

    fn render_tractor(&mut self, tractor_id: &str, frame: u64, fb: &mut FrameBuffer, depth: u32) {
        if depth > 16 {
            return;
        }

        let tractor = match self.project.tractors.get(tractor_id) {
            Some(t) => t,
            None => return,
        };

        // If frame is out of bounds for this tractor
        if frame < tractor.in_frame || (tractor.out_frame > tractor.in_frame && frame > tractor.out_frame) {
            return;
        }

        // Iterate tracks from bottom (0) to top
        for track in &tractor.tracks {
            if let Some(ref hide) = track.hide {
                if hide == "video" || hide == "both" {
                    continue;
                }
            }

            let prod_id = &track.producer_id;

            // Track can point to:
            // 1. A Playlist
            if let Some(playlist) = self.project.playlists.get(prod_id) {
                self.render_playlist(playlist, frame, fb, depth);
            }
            // 2. Another Tractor directly
            else if self.project.tractors.contains_key(prod_id) {
                self.render_tractor(prod_id, frame, fb, depth + 1);
            }
            // 3. A Producer directly
            else if let Some(producer) = self.project.producers.get(prod_id) {
                self.render_producer(producer, frame, None, fb, depth);
            }
        }
    }

    fn render_playlist(&mut self, playlist: &Playlist, timeline_frame: u64, fb: &mut FrameBuffer, depth: u32) {
        if let Some((entry, clip_frame)) = playlist.get_active_clip_at(timeline_frame) {
            let prod_id = &entry.producer_id;

            // Check if entry points to a tractor
            if self.project.tractors.contains_key(prod_id) {
                // If the entry has transform filters (qtblend)
                let qtblend_opt = entry.filters.iter().find_map(|f| {
                    if let FilterType::QtBlend { ref rect_keyframes, .. } = f.filter_type {
                        Some(rect_keyframes.clone())
                    } else {
                        None
                    }
                });

                if let Some(ref qtblend) = qtblend_opt {
                    let (x, y, w, h, opacity) = KeyframeRect::interpolate(qtblend, clip_frame);
                    let mut sub_fb = FrameBuffer::new(fb.width, fb.height);
                    sub_fb.clear(image::Rgba([0, 0, 0, 0]));
                    self.render_tractor(prod_id, clip_frame, &mut sub_fb, depth + 1);
                    fb.blend_layer(&sub_fb.data, x as i32, y as i32, w as u32, h as u32, opacity);
                } else {
                    self.render_tractor(prod_id, clip_frame, fb, depth + 1);
                }
            }
            // Or if entry points to a producer
            else if let Some(producer) = self.project.producers.get(prod_id) {
                self.render_producer(producer, clip_frame, Some(&entry.filters), fb, depth);
            }
        }
    }

    fn render_producer(
        &mut self,
        producer: &crate::parser::Producer,
        clip_frame: u64,
        extra_filters: Option<&[crate::parser::Filter]>,
        fb: &mut FrameBuffer,
        depth: u32,
    ) {
        let (rect_keyframes, drop_shadow, gaussian_blur) = self.collect_filters(producer, extra_filters);

        let (x, y, w, h, opacity) = if let Some(kfs) = rect_keyframes {
            KeyframeRect::interpolate(kfs, clip_frame)
        } else {
            (0.0, 0.0, fb.width as f32, fb.height as f32, 1.0)
        };

        match &producer.source {
            ProducerSource::Image(rel_or_abs_path) => {
                let resolved_path = self.resolve_path(rel_or_abs_path);
                if let Some(img) = self.cache.get_or_load(&resolved_path).cloned() {
                    if let Some((radius, x_off, y_off, color)) = drop_shadow {
                        fb.render_drop_shadow(&img, radius, x_off, y_off, color);
                    }
                    fb.blend_layer(&img, x as i32, y as i32, w as u32, h as u32, opacity);
                    if let Some(sigma) = gaussian_blur {
                        fb.apply_separable_blur(sigma);
                    }
                }
            }
            ProducerSource::Color(color) => {
                let r = ((color >> 24) & 0xFF) as u8;
                let g = ((color >> 16) & 0xFF) as u8;
                let b = ((color >> 8) & 0xFF) as u8;
                let a = (color & 0xFF) as u8;

                if opacity >= 0.99 && x <= 0.0 && y <= 0.0 && w >= fb.width as f32 && h >= fb.height as f32 {
                    fb.clear(image::Rgba([r, g, b, a]));
                }
            }
            ProducerSource::NestedTractor(tid) => {
                self.render_tractor(tid, clip_frame, fb, depth + 1);
            }
            ProducerSource::Video(_) => {
                // Video decoder pass if present
            }
        }
    }

    fn collect_filters<'b>(
        &self,
        producer: &'b crate::parser::Producer,
        extra_filters: Option<&'b [crate::parser::Filter]>,
    ) -> (
        Option<&'b Vec<KeyframeRect>>,
        Option<(f32, f32, f32, u32)>, // drop shadow
        Option<f32>,                  // blur
    ) {
        let mut kfs = None;
        let mut shadow = None;
        let mut blur = None;

        let all_filters = extra_filters.into_iter().flatten().chain(producer.filters.iter());

        for f in all_filters {
            match &f.filter_type {
                FilterType::QtBlend { rect_keyframes, .. } => {
                    if kfs.is_none() {
                        kfs = Some(rect_keyframes);
                    }
                }
                FilterType::DropShadow { radius, x_offset, y_offset, color } => {
                    if shadow.is_none() {
                        shadow = Some((*radius, *x_offset, *y_offset, *color));
                    }
                }
                FilterType::GaussianBlur { sigma } => {
                    if blur.is_none() {
                        blur = Some(*sigma);
                    }
                }
                _ => {}
            }
        }

        (kfs, shadow, blur)
    }

    fn resolve_path(&self, p: &std::path::Path) -> PathBuf {
        if p.is_absolute() {
            p.to_path_buf()
        } else if let Some(ref root) = self.project.root_dir {
            root.join(p)
        } else {
            p.to_path_buf()
        }
    }
}
