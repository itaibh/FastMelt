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
        fb.clear([0, 0, 0, 255]);

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

        // Skip audio tractors completely during video render pass
        if tractor.is_audio {
            return;
        }

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
                if !playlist.is_audio {
                    self.render_playlist(playlist, frame, fb, depth);
                }
            }
            // 2. Another Tractor directly
            else if let Some(sub_tractor) = self.project.tractors.get(prod_id) {
                if !sub_tractor.is_audio {
                    self.render_tractor(prod_id, frame, fb, depth + 1);
                }
            }
            // 3. A Producer directly
            else if let Some(producer) = self.project.producers.get(prod_id) {
                self.render_producer(producer, frame, None, fb, depth);
            }
        }
    }

    fn render_playlist(&mut self, playlist: &Playlist, timeline_frame: u64, fb: &mut FrameBuffer, depth: u32) {
        if playlist.is_audio {
            return;
        }

        if let Some((entry, clip_frame)) = playlist.get_active_clip_at(timeline_frame) {
            let prod_id = &entry.producer_id;

            // Check if entry points to a tractor
            if let Some(sub_tractor) = self.project.tractors.get(prod_id) {
                if sub_tractor.is_audio {
                    return;
                }

                // If the entry has transform filters (qtblend)
                let qtblend_opt = entry.filters.iter().find_map(|f| {
                    if let FilterType::QtBlend { ref rect_keyframes, ref rotation_keyframes, rotate_center, distort } = f.filter_type {
                        Some((rect_keyframes.clone(), rotation_keyframes.clone(), rotate_center, distort))
                    } else {
                        None
                    }
                });

                if let Some((ref rect_kfs, ref rot_kfs, rc, dist)) = qtblend_opt {
                    let (rx, ry, rw, rh, opacity) = KeyframeRect::interpolate(rect_kfs, clip_frame);
                    let rot = MltProject::interpolate_float(rot_kfs, clip_frame);

                    let mut sub_fb = FrameBuffer::new(fb.width, fb.height);
                    sub_fb.clear([0, 0, 0, 0]);
                    self.render_tractor(prod_id, clip_frame, &mut sub_fb, depth + 1);

                    fb.blend_affine(
                        sub_fb.raw_bytes(),
                        sub_fb.width,
                        sub_fb.height,
                        rx,
                        ry,
                        rw,
                        rh,
                        rot,
                        rc,
                        dist,
                        opacity,
                    );
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
        let (qtblend, drop_shadow, gaussian_blur, tint, hflip) = self.collect_filters(producer, extra_filters);

        let (rx, ry, rw, rh, opacity, rotation, rotate_center, distort) = if let Some((rect_kfs, rot_kfs, rc, dist)) = qtblend {
            let (x, y, w, h, op) = KeyframeRect::interpolate(rect_kfs, clip_frame);
            let rot = MltProject::interpolate_float(rot_kfs, clip_frame);
            (x, y, w, h, op, rot, rc, dist)
        } else {
            (0.0, 0.0, fb.width as f32, fb.height as f32, 1.0, 0.0, false, false)
        };

        match &producer.source {
            ProducerSource::Image(rel_or_abs_path) => {
                let resolved_path = self.resolve_path(rel_or_abs_path);
                let tint_opt = tint.map(|(c, _)| c);
                let sigma_val = gaussian_blur.unwrap_or(0.0);

                if let Some(img) = self.cache.get_or_load_processed(&resolved_path, tint_opt, sigma_val, hflip).cloned() {
                    let src_w = img.width();
                    let src_h = img.height();

                    // Calculate actual drawn geometry (accounting for distort = false aspect ratio preservation)
                    let (draw_x, draw_y, draw_w, draw_h) = if distort {
                        (rx, ry, rw, rh)
                    } else if rw > 0.0 && rh > 0.0 && src_w > 0 && src_h > 0 {
                        let src_aspect = src_w as f32 / src_h as f32;
                        let target_aspect = rw / rh;
                        let scale = if src_aspect >= target_aspect {
                            rw / src_w as f32
                        } else {
                            rh / src_h as f32
                        };
                        let dw = src_w as f32 * scale;
                        let dh = src_h as f32 * scale;
                        let fx = rx + (rw - dw) / 2.0;
                        let fy = ry + (rh - dh) / 2.0;
                        (fx, fy, dw, dh)
                    } else {
                        (rx, ry, rw, rh)
                    };

                    // 1. Drop shadow is rendered behind the image at (draw_x + x_off, draw_y + y_off)
                    if let Some((radius, x_off, y_off, color)) = drop_shadow {
                        fb.render_drop_shadow(
                            &img,
                            (draw_x + x_off) as i32,
                            (draw_y + y_off) as i32,
                            draw_w.round().max(1.0) as u32,
                            draw_h.round().max(1.0) as u32,
                            radius,
                            color,
                            opacity * 0.8,
                        );
                    }

                    // 2. Blend processed image with full 2D affine transform (rotation + aspect scaling)
                    fb.blend_affine(
                        img.as_raw(),
                        src_w,
                        src_h,
                        rx,
                        ry,
                        rw,
                        rh,
                        rotation,
                        rotate_center,
                        distort,
                        opacity,
                    );
                }
            }
            ProducerSource::Color(color) => {
                if color[3] == 0 {
                    // Completely transparent base track - do not clear or paint!
                    return;
                }
                let a = (((color[3] as f32 / 255.0) * opacity * 255.0)).round().clamp(0.0, 255.0) as u8;
                let c = [color[0], color[1], color[2], a];

                if opacity >= 0.99 && a == 255 && rx <= 0.0 && ry <= 0.0 && rw >= fb.width as f32 && rh >= fb.height as f32 {
                    fb.clear(c);
                } else {
                    fb.fill_rect(rx as i32, ry as i32, rw as u32, rh as u32, c);
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
        Option<(&'b Vec<KeyframeRect>, &'b Vec<(u64, f32)>, bool, bool)>, // (rect_kfs, rot_kfs, rotate_center, distort)
        Option<(f32, f32, f32, u32)>,                                     // drop shadow: (radius, x_off, y_off, color)
        Option<f32>,                                                      // blur: sigma
        Option<([u8; 4], f32)>,                                          // tint: (color, amount)
        bool,                                                             // hflip
    ) {
        let mut qtblend = None;
        let mut shadow = None;
        let mut blur = None;
        let mut tint = None;
        let mut hflip = false;

        let all_filters = extra_filters.into_iter().flatten().chain(producer.filters.iter());

        for f in all_filters {
            match &f.filter_type {
                FilterType::QtBlend { rect_keyframes, rotation_keyframes, rotate_center, distort } => {
                    if qtblend.is_none() {
                        qtblend = Some((rect_keyframes, rotation_keyframes, *rotate_center, *distort));
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
                FilterType::Tint { color, amount } => {
                    if tint.is_none() {
                        tint = Some((*color, *amount));
                    }
                }
                FilterType::HorizontalFlip => {
                    hflip = true;
                }
                _ => {}
            }
        }

        (qtblend, shadow, blur, tint, hflip)
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
