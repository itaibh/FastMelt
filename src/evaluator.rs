use std::path::PathBuf;
use crate::cache::TextureCache;
use crate::parser::{FilterType, KeyframeRect, MltProject, Playlist, ProducerSource};
use crate::pipeline::FrameBuffer;

pub struct TimelineEvaluator<'a> {
    project: &'a MltProject,
    cache: &'a mut TextureCache,
    nested_projects: std::collections::HashMap<PathBuf, MltProject>,
}

impl<'a> TimelineEvaluator<'a> {
    pub fn new(project: &'a MltProject, cache: &'a mut TextureCache) -> Self {
        Self {
            project,
            cache,
            nested_projects: std::collections::HashMap::new(),
        }
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

                if let Some((ref rect_kfs, ref rot_kfs, rc, _dist)) = qtblend_opt {
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
                        true,
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
        match &producer.source {
            ProducerSource::Image(rel_or_abs_path) => {
                let resolved_path = self.resolve_path(rel_or_abs_path);

                // Collect effect filters
                let all_filters: Vec<_> = extra_filters
                    .into_iter()
                    .flatten()
                    .chain(producer.filters.iter())
                    .collect();

                let mut drop_shadow = None;
                let mut gaussian_blur = None;
                let mut tint = None;
                let mut hflip = false;

                for f in &all_filters {
                    match &f.filter_type {
                        FilterType::DropShadow { radius, x_offset, y_offset, color } => {
                            if drop_shadow.is_none() {
                                drop_shadow = Some((*radius, *x_offset, *y_offset, *color));
                            }
                        }
                        FilterType::GaussianBlur { sigma } => {
                            if gaussian_blur.is_none() {
                                gaussian_blur = Some(*sigma);
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

                let tint_opt = tint.map(|(c, _)| c);
                let sigma_val = gaussian_blur.unwrap_or(0.0);

                if let Some(img) = self.cache.get_or_load_processed(&resolved_path, tint_opt, sigma_val, hflip).cloned() {
                    let src_w = img.width();
                    let src_h = img.height();
                    let canvas_w = fb.width as f32;
                    let canvas_h = fb.height as f32;

                    // 1. Resolve Producer-level transform
                    let mut prod_matrix = None;
                    let mut prod_opacity = 1.0f32;

                    for f in &producer.filters {
                        if let FilterType::QtBlend { rect_keyframes, rotation_keyframes, rotate_center, distort } = &f.filter_type {
                            let (rx, ry, rw, rh, op) = KeyframeRect::interpolate(rect_keyframes, clip_frame);
                            let rot = MltProject::interpolate_float(rotation_keyframes, clip_frame);
                            prod_opacity = op;

                            let is_ident = rot.abs() <= 0.001
                                && rx.abs() <= 0.001
                                && ry.abs() <= 0.001
                                && (rw - src_w as f32).abs() <= 1.0
                                && (rh - src_h as f32).abs() <= 1.0;

                            if !is_ident {
                                prod_matrix = Some(FrameBuffer::build_affine_matrix(
                                    src_w as f32,
                                    src_h as f32,
                                    rx,
                                    ry,
                                    rw,
                                    rh,
                                    rot,
                                    *rotate_center,
                                    *distort,
                                ));
                                break;
                            }
                        }
                    }

                    // 2. Resolve Playlist entry-level transform
                    let mut entry_matrix = None;
                    let mut entry_opacity = 1.0f32;

                    if let Some(extras) = extra_filters {
                        // Find active non-identity qtblend (prioritize specific transforms over 0 0 1080 1920)
                        let mut candidates = Vec::new();
                        for f in extras {
                            if let FilterType::QtBlend { rect_keyframes, rotation_keyframes, rotate_center, distort } = &f.filter_type {
                                let (rx, ry, rw, rh, op) = KeyframeRect::interpolate(rect_keyframes, clip_frame);
                                let rot = MltProject::interpolate_float(rotation_keyframes, clip_frame);
                                let is_ident = rot.abs() <= 0.001
                                    && rx.abs() <= 0.001
                                    && ry.abs() <= 0.001
                                    && (rw - canvas_w).abs() <= 1.0
                                    && (rh - canvas_h).abs() <= 1.0;
                                candidates.push((rx, ry, rw, rh, op, rot, *rotate_center, *distort, is_ident));
                            }
                        }

                        // Pick the first non-identity transform, or fallback to the last transform
                        if let Some(&(rx, ry, rw, rh, op, rot, rc, dist, is_ident)) = candidates.iter().find(|c| !c.8).or_else(|| candidates.last()) {
                            entry_opacity = op;
                            if !is_ident {
                                entry_matrix = Some((rx, ry, rw, rh, rot, rc, dist));
                            }
                        }
                    }

                    // 3. Compose final affine matrix
                    let (composite_matrix, total_opacity) = match (prod_matrix, entry_matrix) {
                        (Some(m1), Some((rx2, ry2, rw2, rh2, rot2, rc2, _dist2))) => {
                            let m2 = FrameBuffer::build_affine_matrix(canvas_w, canvas_h, rx2, ry2, rw2, rh2, rot2, rc2, true);
                            (m2 * m1, prod_opacity * entry_opacity)
                        }
                        (Some(m1), None) => (m1, prod_opacity * entry_opacity),
                        (None, Some((rx2, ry2, rw2, rh2, rot2, rc2, dist2))) => {
                            let m2 = FrameBuffer::build_affine_matrix(src_w as f32, src_h as f32, rx2, ry2, rw2, rh2, rot2, rc2, dist2);
                            (m2, entry_opacity)
                        }
                        (None, None) => {
                            let m = FrameBuffer::build_affine_matrix(src_w as f32, src_h as f32, 0.0, 0.0, canvas_w, canvas_h, 0.0, false, false);
                            (m, 1.0)
                        }
                    };

                    // 4. Render drop shadow if present
                    if let Some((radius, x_off, y_off, color)) = drop_shadow {
                        // Drop shadow geometry using affine bounds
                        let p0 = composite_matrix.transform_point2(glam::vec2(0.0, 0.0));
                        let p2 = composite_matrix.transform_point2(glam::vec2(src_w as f32, src_h as f32));
                        let shadow_x = (p0.x.min(p2.x) + x_off) as i32;
                        let shadow_y = (p0.y.min(p2.y) + y_off) as i32;
                        let shadow_w = (p2.x - p0.x).abs().round().max(1.0) as u32;
                        let shadow_h = (p2.y - p0.y).abs().round().max(1.0) as u32;

                        fb.render_drop_shadow(
                            &img,
                            shadow_x,
                            shadow_y,
                            shadow_w,
                            shadow_h,
                            radius,
                            color,
                            total_opacity * 0.8,
                        );
                    }

                    // 5. Blend processed image with composite matrix
                    fb.blend_affine_matrix(
                        img.as_raw(),
                        src_w,
                        src_h,
                        composite_matrix,
                        total_opacity,
                    );
                }
            }
            ProducerSource::Color(color) => {
                if color[3] == 0 {
                    // Completely transparent base track - do not clear or paint!
                    return;
                }
                let mut opacity = 1.0f32;
                let mut rect = (0.0f32, 0.0f32, fb.width as f32, fb.height as f32);

                if let Some(extras) = extra_filters {
                    for f in extras {
                        if let FilterType::QtBlend { rect_keyframes, .. } = &f.filter_type {
                            let (x, y, w, h, op) = KeyframeRect::interpolate(rect_keyframes, clip_frame);
                            rect = (x, y, w, h);
                            opacity = op;
                            break;
                        }
                    }
                }

                let a = (((color[3] as f32 / 255.0) * opacity * 255.0)).round().clamp(0.0, 255.0) as u8;
                let c = [color[0], color[1], color[2], a];

                if opacity >= 0.99 && a == 255 && rect.0 <= 0.0 && rect.1 <= 0.0 && rect.2 >= fb.width as f32 && rect.3 >= fb.height as f32 {
                    fb.clear(c);
                } else {
                    fb.fill_rect(rect.0 as i32, rect.1 as i32, rect.2 as u32, rect.3 as u32, c);
                }
            }
            ProducerSource::NestedTractor(tid) => {
                self.render_tractor(tid, clip_frame, fb, depth + 1);
            }
            ProducerSource::MltXml(rel_or_abs_path) => {
                let resolved_path = self.resolve_path(rel_or_abs_path);
                if !self.nested_projects.contains_key(&resolved_path) {
                    if let Ok(mut sub_proj) = MltProject::from_file(&resolved_path) {
                        if sub_proj.root_dir.is_none() {
                            sub_proj.root_dir = resolved_path.parent().map(|p| p.to_path_buf());
                        }
                        self.nested_projects.insert(resolved_path.clone(), sub_proj);
                    }
                }

                if let Some(sub_proj) = self.nested_projects.get(&resolved_path).cloned() {
                    let sub_root_id = sub_proj.root_tractor_id.clone()
                        .or_else(|| sub_proj.tractors.keys().next().cloned());

                    if let Some(root_id) = sub_root_id {
                        let mut sub_fb = FrameBuffer::new(fb.width, fb.height);
                        sub_fb.clear([0, 0, 0, 0]);

                        // Evaluate sub-tractor using our shared texture cache
                        {
                            let mut sub_evaluator = TimelineEvaluator {
                                project: &sub_proj,
                                cache: self.cache,
                                nested_projects: std::collections::HashMap::new(),
                            };
                            sub_evaluator.render_tractor(&root_id, clip_frame, &mut sub_fb, depth + 1);
                        }

                        // Collect extra filters and producer filters
                        let all_filters: Vec<_> = extra_filters
                            .into_iter()
                            .flatten()
                            .chain(producer.filters.iter())
                            .collect();

                        let mut qtblend_filter = None;

                        for f in &all_filters {
                            if let FilterType::QtBlend { rect_keyframes, rotation_keyframes, rotate_center, distort } = &f.filter_type {
                                if qtblend_filter.is_none() {
                                    qtblend_filter = Some((rect_keyframes.clone(), rotation_keyframes.clone(), *rotate_center, *distort));
                                }
                            }
                        }

                        if let Some((ref rect_kfs, ref rot_kfs, rc, _dist)) = qtblend_filter {
                            let (rx, ry, rw, rh, opacity) = KeyframeRect::interpolate(rect_kfs, clip_frame);
                            let rot = MltProject::interpolate_float(rot_kfs, clip_frame);

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
                                true,
                                opacity,
                            );
                        } else {
                            // Direct alpha composite onto fb
                            fb.blend_affine(
                                sub_fb.raw_bytes(),
                                sub_fb.width,
                                sub_fb.height,
                                0.0,
                                0.0,
                                fb.width as f32,
                                fb.height as f32,
                                0.0,
                                false,
                                true,
                                1.0,
                            );
                        }
                    }
                }
            }
            ProducerSource::Video(_) => {
                // Video decoder pass if present
            }
        }
    }

    fn resolve_path(&self, p: &std::path::Path) -> PathBuf {
        if p.is_absolute() && p.exists() {
            return p.to_path_buf();
        }
        if let Some(ref root) = self.project.root_dir {
            let candidate = root.join(p);
            if candidate.exists() {
                return candidate;
            }
        }
        if let Ok(home) = std::env::var("HOME") {
            let candidate = std::path::Path::new(&home).join(p);
            if candidate.exists() {
                return candidate;
            }
        }
        // Fallback: check if filename exists in root or library or common folders
        if let Some(fname) = p.file_name() {
            if let Some(ref root) = self.project.root_dir {
                let candidate = root.join(fname);
                if candidate.exists() {
                    return candidate;
                }
                let graphics = root.join("graphics").join(fname);
                if graphics.exists() {
                    return graphics;
                }
            }
            if let Ok(home) = std::env::var("HOME") {
                let base = std::path::Path::new(&home).join("Videos/rogue unfiltered");
                let c1 = base.join(fname);
                if c1.exists() { return c1; }
                let c2 = base.join("graphics").join(fname);
                if c2.exists() { return c2; }
                let c3 = base.join("library/graphics").join(fname);
                if c3.exists() { return c3; }
                let c4 = base.join("e01").join(fname);
                if c4.exists() { return c4; }
                let c5 = base.join("library").join(fname);
                if c5.exists() { return c5; }
            }
        }
        p.to_path_buf()
    }
}
