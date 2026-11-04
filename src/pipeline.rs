use image::RgbaImage;
use rayon::prelude::*;
use glam::{vec2, Affine2};

#[derive(Clone)]
pub struct FrameBuffer {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    scratch: Vec<u8>,
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width as usize) * (height as usize) * 4;
        Self {
            width,
            height,
            data: vec![0u8; size],
            scratch: vec![0u8; size],
        }
    }

    pub fn clear(&mut self, color: [u8; 4]) {
        let stride = (self.width as usize) * 4;
        self.data
            .par_chunks_exact_mut(stride)
            .for_each(|row| {
                for pixel in row.chunks_exact_mut(4) {
                    pixel.copy_from_slice(&color);
                }
            });
    }

    pub fn raw_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn blend_layer(
        &mut self,
        source: &RgbaImage,
        dst_x: i32,
        dst_y: i32,
        dst_w: u32,
        dst_h: u32,
        opacity: f32,
    ) {
        self.blend_raw(
            source.as_raw(),
            source.width(),
            source.height(),
            dst_x,
            dst_y,
            dst_w,
            dst_h,
            opacity,
        );
    }

    /// Parallel integer fixed-point alpha-over blitter
    pub fn blend_raw(
        &mut self,
        src_data: &[u8],
        src_w: u32,
        src_h: u32,
        dst_x: i32,
        dst_y: i32,
        dst_w: u32,
        dst_h: u32,
        opacity: f32,
    ) {
        if opacity <= 0.001 || dst_w == 0 || dst_h == 0 || src_w == 0 || src_h == 0 {
            return;
        }

        let target_w = self.width as i32;
        let target_h = self.height as i32;

        let start_y = dst_y.max(0) as usize;
        let end_y = (dst_y + dst_h as i32).clamp(0, target_h) as usize;
        let start_x = dst_x.max(0) as usize;
        let end_x = (dst_x + dst_w as i32).clamp(0, target_w) as usize;

        if start_x >= end_x || start_y >= end_y {
            return;
        }

        let scale_x = src_w as f32 / dst_w as f32;
        let scale_y = src_h as f32 / dst_h as f32;

        let stride = (self.width as usize) * 4;
        let src_stride = (src_w as usize) * 4;
        let opacity_u32 = (opacity.clamp(0.0, 1.0) * 255.0).round() as u32;

        let active_rows = &mut self.data[(start_y * stride)..(end_y * stride)];

        active_rows
            .par_chunks_exact_mut(stride)
            .enumerate()
            .for_each(|(row_idx, row)| {
                let y = start_y + row_idx;
                let src_y_f = (y as f32 - dst_y as f32) * scale_y;
                let src_y = (src_y_f as usize).min(src_h as usize - 1);
                let src_row_offset = src_y * src_stride;

                for x in start_x..end_x {
                    let src_x_f = (x as f32 - dst_x as f32) * scale_x;
                    let src_x = (src_x_f as usize).min(src_w as usize - 1);
                    let s_idx = src_row_offset + src_x * 4;

                    let sa = src_data[s_idx + 3] as u32;
                    let eff_a = (sa * opacity_u32 + 128) >> 8;
                    if eff_a <= 1 {
                        continue;
                    }

                    let d_idx = x * 4;
                    let sr = src_data[s_idx] as u32;
                    let sg = src_data[s_idx + 1] as u32;
                    let sb = src_data[s_idx + 2] as u32;

                    if eff_a >= 254 {
                        row[d_idx] = sr as u8;
                        row[d_idx + 1] = sg as u8;
                        row[d_idx + 2] = sb as u8;
                        row[d_idx + 3] = 255;
                    } else {
                        let inv_a = 255 - eff_a;
                        let dr = row[d_idx] as u32;
                        let dg = row[d_idx + 1] as u32;
                        let db = row[d_idx + 2] as u32;
                        let da = row[d_idx + 3] as u32;

                        row[d_idx] = ((sr * eff_a + dr * inv_a + 128) / 255) as u8;
                        row[d_idx + 1] = ((sg * eff_a + dg * inv_a + 128) / 255) as u8;
                        row[d_idx + 2] = ((sb * eff_a + db * inv_a + 128) / 255) as u8;
                        row[d_idx + 3] = (eff_a + ((da * inv_a + 128) / 255)).min(255) as u8;
                    }
                }
            });
    }

    /// Build 2D Affine Matrix for qtblend geometry with optional DAR override
    pub fn build_affine_matrix_with_dar(
        src_w: f32,
        src_h: f32,
        rect_x: f32,
        rect_y: f32,
        rect_w: f32,
        rect_h: f32,
        rotation_deg: f32,
        rotate_center: bool,
        distort: bool,
        dar_opt: Option<f32>,
    ) -> Affine2 {
        let mut m = Affine2::from_translation(vec2(rect_x, rect_y));
        let rad = rotation_deg.to_radians();
        if rotate_center {
            m = m * Affine2::from_translation(vec2(rect_w / 2.0, rect_h / 2.0));
            m = m * Affine2::from_angle(rad);
            m = m * Affine2::from_translation(vec2(-rect_w / 2.0, -rect_h / 2.0));
        } else {
            m = m * Affine2::from_angle(rad);
        }

        if distort {
            m = m * Affine2::from_scale(vec2(rect_w / src_w, rect_h / src_h));
        } else {
            let dar = dar_opt.unwrap_or_else(|| src_w / src_h);
            let target_aspect = rect_w / rect_h;
            let scale = if dar >= target_aspect {
                rect_w / src_w
            } else {
                rect_h / src_h
            };
            let draw_w = src_w * scale;
            let draw_h = src_h * scale;
            m = m * Affine2::from_translation(vec2((rect_w - draw_w) / 2.0, (rect_h - draw_h) / 2.0));
            m = m * Affine2::from_scale(vec2(scale, scale));
        }
        m
    }

    /// Build 2D Affine Matrix for qtblend geometry
    pub fn build_affine_matrix(
        src_w: f32,
        src_h: f32,
        rect_x: f32,
        rect_y: f32,
        rect_w: f32,
        rect_h: f32,
        rotation_deg: f32,
        rotate_center: bool,
        distort: bool,
    ) -> Affine2 {
        Self::build_affine_matrix_with_dar(
            src_w,
            src_h,
            rect_x,
            rect_y,
            rect_w,
            rect_h,
            rotation_deg,
            rotate_center,
            distort,
            None,
        )
    }

    /// Full 2D Affine Transformation Blitter using precomputed Affine2 matrix
    pub fn blend_affine_matrix(
        &mut self,
        src_data: &[u8],
        src_w: u32,
        src_h: u32,
        m: Affine2,
        opacity: f32,
    ) {
        if opacity <= 0.001 || src_w == 0 || src_h == 0 {
            return;
        }

        let p0 = m.transform_point2(vec2(0.0, 0.0));
        let p1 = m.transform_point2(vec2(src_w as f32, 0.0));
        let p2 = m.transform_point2(vec2(src_w as f32, src_h as f32));
        let p3 = m.transform_point2(vec2(0.0, src_h as f32));

        let min_x = p0.x.min(p1.x).min(p2.x).min(p3.x).floor().max(0.0) as usize;
        let max_x = p0.x.max(p1.x).max(p2.x).max(p3.x).ceil().min(self.width as f32) as usize;
        let min_y = p0.y.min(p1.y).min(p2.y).min(p3.y).floor().max(0.0) as usize;
        let max_y = p0.y.max(p1.y).max(p2.y).max(p3.y).ceil().min(self.height as f32) as usize;

        if min_x >= max_x || min_y >= max_y {
            return;
        }

        let inv = m.inverse();
        let stride = (self.width as usize) * 4;
        let src_stride = (src_w as usize) * 4;
        let opacity_u32 = (opacity.clamp(0.0, 1.0) * 255.0).round() as u32;

        let active_rows = &mut self.data[(min_y * stride)..(max_y * stride)];

        active_rows
            .par_chunks_exact_mut(stride)
            .enumerate()
            .for_each(|(row_idx, row)| {
                let y = min_y + row_idx;
                for x in min_x..max_x {
                    let src_p = inv.transform_point2(vec2(x as f32 + 0.5, y as f32 + 0.5));
                    if src_p.x < 0.0 || src_p.x >= (src_w as f32) || src_p.y < 0.0 || src_p.y >= (src_h as f32) {
                        continue;
                    }

                    // Bilinear sampling
                    let x0 = src_p.x.floor() as usize;
                    let y0 = src_p.y.floor() as usize;
                    let x1 = (x0 + 1).min(src_w as usize - 1);
                    let y1 = (y0 + 1).min(src_h as usize - 1);
                    let fx = src_p.x - x0 as f32;
                    let fy = src_p.y - y0 as f32;
                    let fx_u32 = (fx * 256.0) as u32;
                    let fy_u32 = (fy * 256.0) as u32;
                    let inv_fx = 256 - fx_u32;
                    let inv_fy = 256 - fy_u32;

                    let w00 = (inv_fx * inv_fy) >> 8;
                    let w10 = (fx_u32 * inv_fy) >> 8;
                    let w01 = (inv_fx * fy_u32) >> 8;
                    let w11 = (fx_u32 * fy_u32) >> 8;

                    let idx00 = y0 * src_stride + x0 * 4;
                    let idx10 = y0 * src_stride + x1 * 4;
                    let idx01 = y1 * src_stride + x0 * 4;
                    let idx11 = y1 * src_stride + x1 * 4;

                    let a00 = src_data[idx00 + 3] as u32;
                    let a10 = src_data[idx10 + 3] as u32;
                    let a01 = src_data[idx01 + 3] as u32;
                    let a11 = src_data[idx11 + 3] as u32;

                    let wa00 = w00 * a00;
                    let wa10 = w10 * a10;
                    let wa01 = w01 * a01;
                    let wa11 = w11 * a11;

                    let sum_wa = wa00 + wa10 + wa01 + wa11;
                    let sa = sum_wa >> 8;

                    let eff_a = (sa * opacity_u32 + 128) >> 8;
                    if eff_a <= 1 {
                        continue;
                    }

                    // Weight RGB by alpha to prevent transparent pixel color bleeding / dark halos
                    let (sr, sg, sb) = if sum_wa > 0 {
                        (
                            ((src_data[idx00] as u32 * wa00
                                + src_data[idx10] as u32 * wa10
                                + src_data[idx01] as u32 * wa01
                                + src_data[idx11] as u32 * wa11)
                                / sum_wa)
                                .min(255),
                            ((src_data[idx00 + 1] as u32 * wa00
                                + src_data[idx10 + 1] as u32 * wa10
                                + src_data[idx01 + 1] as u32 * wa01
                                + src_data[idx11 + 1] as u32 * wa11)
                                / sum_wa)
                                .min(255),
                            ((src_data[idx00 + 2] as u32 * wa00
                                + src_data[idx10 + 2] as u32 * wa10
                                + src_data[idx01 + 2] as u32 * wa01
                                + src_data[idx11 + 2] as u32 * wa11)
                                / sum_wa)
                                .min(255),
                        )
                    } else {
                        (0, 0, 0)
                    };

                    let d_idx = x * 4;
                    let dr = row[d_idx] as u32;
                    let dg = row[d_idx + 1] as u32;
                    let db = row[d_idx + 2] as u32;
                    let da = row[d_idx + 3] as u32;

                    if eff_a >= 254 && da >= 254 {
                        row[d_idx] = sr as u8;
                        row[d_idx + 1] = sg as u8;
                        row[d_idx + 2] = sb as u8;
                        row[d_idx + 3] = 255;
                    } else {
                        let inv_a = 255 - eff_a;
                        row[d_idx] = ((sr * eff_a + dr * inv_a + 128) / 255) as u8;
                        row[d_idx + 1] = ((sg * eff_a + dg * inv_a + 128) / 255) as u8;
                        row[d_idx + 2] = ((sb * eff_a + db * inv_a + 128) / 255) as u8;
                        row[d_idx + 3] = (eff_a + ((da * inv_a + 128) / 255)).min(255) as u8;
                    }
                }
            });
    }

    /// Full 2D Affine Transformation Blitter (supports rotation, scaling, aspect ratio preservation)
    pub fn blend_affine(
        &mut self,
        src_data: &[u8],
        src_w: u32,
        src_h: u32,
        rect_x: f32,
        rect_y: f32,
        rect_w: f32,
        rect_h: f32,
        rotation_deg: f32,
        rotate_center: bool,
        distort: bool,
        opacity: f32,
    ) {
        if opacity <= 0.001 || rect_w <= 0.0 || rect_h <= 0.0 || src_w == 0 || src_h == 0 {
            return;
        }

        // Fast path: axis-aligned (rotation == 0)
        if rotation_deg.abs() <= 0.001 {
            let (final_x, final_y, final_w, final_h) = if distort {
                (rect_x, rect_y, rect_w, rect_h)
            } else {
                let src_aspect = src_w as f32 / src_h as f32;
                let target_aspect = rect_w / rect_h;
                let scale = if src_aspect >= target_aspect {
                    rect_w / src_w as f32
                } else {
                    rect_h / src_h as f32
                };
                let draw_w = src_w as f32 * scale;
                let draw_h = src_h as f32 * scale;
                let fx = rect_x + (rect_w - draw_w) / 2.0;
                let fy = rect_y + (rect_h - draw_h) / 2.0;
                (fx, fy, draw_w, draw_h)
            };

            self.blend_raw(
                src_data,
                src_w,
                src_h,
                final_x.round() as i32,
                final_y.round() as i32,
                final_w.round().max(1.0) as u32,
                final_h.round().max(1.0) as u32,
                opacity,
            );
            return;
        }

        let m = Self::build_affine_matrix(
            src_w as f32,
            src_h as f32,
            rect_x,
            rect_y,
            rect_w,
            rect_h,
            rotation_deg,
            rotate_center,
            distort,
        );
        self.blend_affine_matrix(src_data, src_w, src_h, m, opacity);
    }

    /// Fill a rectangle with solid or transparent color
    pub fn fill_rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: [u8; 4]) {
        let target_w = self.width as i32;
        let target_h = self.height as i32;

        let start_y = y.max(0) as usize;
        let end_y = (y + h as i32).clamp(0, target_h) as usize;
        let start_x = x.max(0) as usize;
        let end_x = (x + w as i32).clamp(0, target_w) as usize;

        if start_x >= end_x || start_y >= end_y || color[3] == 0 {
            return;
        }

        let stride = (self.width as usize) * 4;
        let active_rows = &mut self.data[(start_y * stride)..(end_y * stride)];
        let a = color[3] as u32;

        active_rows
            .par_chunks_exact_mut(stride)
            .for_each(|row| {
                if a == 255 {
                    for col in start_x..end_x {
                        let d_idx = col * 4;
                        row[d_idx] = color[0];
                        row[d_idx + 1] = color[1];
                        row[d_idx + 2] = color[2];
                        row[d_idx + 3] = 255;
                    }
                } else {
                    let inv_a = 255 - a;
                    let cr = color[0] as u32;
                    let cg = color[1] as u32;
                    let cb = color[2] as u32;
                    for col in start_x..end_x {
                        let d_idx = col * 4;
                        row[d_idx] = ((cr * a + row[d_idx] as u32 * inv_a + 128) / 255) as u8;
                        row[d_idx + 1] = ((cg * a + row[d_idx + 1] as u32 * inv_a + 128) / 255) as u8;
                        row[d_idx + 2] = ((cb * a + row[d_idx + 2] as u32 * inv_a + 128) / 255) as u8;
                        row[d_idx + 3] = (a + ((row[d_idx + 3] as u32 * inv_a + 128) / 255)).min(255) as u8;
                    }
                }
            });
    }

    /// Fast Two-Pass Separable Gaussian Blur
    pub fn apply_separable_blur(&mut self, radius: f32) {
        Self::apply_separable_blur_buffer(
            &mut self.data,
            &mut self.scratch,
            self.width,
            self.height,
            radius,
        );
    }

    /// Static 2-pass separable blur for any RGBA buffer
    pub fn apply_separable_blur_buffer(
        data: &mut [u8],
        scratch: &mut [u8],
        width: u32,
        height: u32,
        radius: f32,
    ) {
        if radius < 0.5 {
            return;
        }

        let radius_int = (radius * 2.5).ceil().max(1.0) as i32;
        let kernel_size = (radius_int * 2 + 1) as usize;
        let mut kernel = vec![0.0f32; kernel_size];
        let sigma = radius;
        let two_sigma_sq = 2.0 * sigma * sigma;
        let mut sum = 0.0;

        for i in -radius_int..=radius_int {
            let weight = (-((i * i) as f32) / two_sigma_sq).exp();
            kernel[(i + radius_int) as usize] = weight;
            sum += weight;
        }
        for w in kernel.iter_mut() {
            *w /= sum;
        }

        let w = width as usize;
        let h = height as usize;
        let stride = w * 4;

        if scratch.len() < stride * h {
            return;
        }

        // Pass 1: Horizontal Blur (parallel across rows)
        let k_ref = &kernel;
        let data_ref = &*data;
        scratch[..stride * h]
            .par_chunks_exact_mut(stride)
            .enumerate()
            .for_each(|(y, out_row)| {
                let in_row = &data_ref[y * stride..(y + 1) * stride];
                for x in 0..w {
                    let mut r = 0.0f32;
                    let mut g = 0.0f32;
                    let mut b = 0.0f32;
                    let mut a = 0.0f32;

                    for (ki, &weight) in k_ref.iter().enumerate() {
                        let offset = ki as i32 - radius_int;
                        let sample_x = (x as i32 + offset).clamp(0, w as i32 - 1) as usize;
                        let s_idx = sample_x * 4;
                        r += in_row[s_idx] as f32 * weight;
                        g += in_row[s_idx + 1] as f32 * weight;
                        b += in_row[s_idx + 2] as f32 * weight;
                        a += in_row[s_idx + 3] as f32 * weight;
                    }

                    let d_idx = x * 4;
                    out_row[d_idx] = r.round().clamp(0.0, 255.0) as u8;
                    out_row[d_idx + 1] = g.round().clamp(0.0, 255.0) as u8;
                    out_row[d_idx + 2] = b.round().clamp(0.0, 255.0) as u8;
                    out_row[d_idx + 3] = a.round().clamp(0.0, 255.0) as u8;
                }
            });

        // Pass 2: Vertical Blur (cache-tiled parallel rows reading from scratch)
        let scratch_ref = &scratch[..stride * h];
        data[..stride * h]
            .par_chunks_exact_mut(stride)
            .enumerate()
            .for_each(|(y, out_row)| {
                for x_chunk in (0..w).step_by(64) {
                    let x_end = (x_chunk + 64).min(w);
                    let len = x_end - x_chunk;
                    let mut acc_r = [0.0f32; 64];
                    let mut acc_g = [0.0f32; 64];
                    let mut acc_b = [0.0f32; 64];
                    let mut acc_a = [0.0f32; 64];

                    for (ki, &weight) in k_ref.iter().enumerate() {
                        let offset = ki as i32 - radius_int;
                        let sample_y = (y as i32 + offset).clamp(0, h as i32 - 1) as usize;
                        let sample_slice = &scratch_ref[sample_y * stride + x_chunk * 4..sample_y * stride + x_end * 4];

                        for i in 0..len {
                            let s = i * 4;
                            acc_r[i] += sample_slice[s] as f32 * weight;
                            acc_g[i] += sample_slice[s + 1] as f32 * weight;
                            acc_b[i] += sample_slice[s + 2] as f32 * weight;
                            acc_a[i] += sample_slice[s + 3] as f32 * weight;
                        }
                    }

                    for i in 0..len {
                        let d = (x_chunk + i) * 4;
                        out_row[d] = acc_r[i].round().clamp(0.0, 255.0) as u8;
                        out_row[d + 1] = acc_g[i].round().clamp(0.0, 255.0) as u8;
                        out_row[d + 2] = acc_b[i].round().clamp(0.0, 255.0) as u8;
                        out_row[d + 3] = acc_a[i].round().clamp(0.0, 255.0) as u8;
                    }
                }
            });
    }

    /// Fast Drop Shadow with correct scaled geometry and padded blur
    pub fn render_drop_shadow(
        &mut self,
        source: &RgbaImage,
        dst_x: i32,
        dst_y: i32,
        dst_w: u32,
        dst_h: u32,
        radius: f32,
        color: u32,
        opacity: f32,
    ) {
        if opacity <= 0.001 || dst_w == 0 || dst_h == 0 {
            return;
        }

        let cr = ((color >> 24) & 0xFF) as u8;
        let cg = ((color >> 16) & 0xFF) as u8;
        let cb = ((color >> 8) & 0xFF) as u8;
        let ca = (color & 0xFF) as f32 / 255.0;

        let eff_opacity = opacity * ca;

        if radius < 0.5 {
            let src_raw = source.as_raw();
            let mut sil = vec![0u8; (source.width() * source.height() * 4) as usize];
            for i in 0..(source.width() * source.height()) as usize {
                let a = src_raw[i * 4 + 3];
                if a > 0 {
                    sil[i * 4] = cr;
                    sil[i * 4 + 1] = cg;
                    sil[i * 4 + 2] = cb;
                    sil[i * 4 + 3] = a;
                }
            }
            self.blend_raw(&sil, source.width(), source.height(), dst_x, dst_y, dst_w, dst_h, eff_opacity);
        } else {
            let pad = (radius * 2.5).ceil() as u32;
            let pad_i32 = pad as i32;
            let sh_w = dst_w + pad * 2;
            let sh_h = dst_h + pad * 2;

            let mut sil = vec![0u8; (sh_w * sh_h * 4) as usize];
            let src_raw = source.as_raw();
            let src_w = source.width();
            let src_h = source.height();
            let scale_x = src_w as f32 / dst_w as f32;
            let scale_y = src_h as f32 / dst_h as f32;
            let sil_stride = (sh_w as usize) * 4;

            for sy in 0..dst_h {
                let sil_y = (sy + pad) as usize;
                let gy = (sy as f32 + 0.5) * scale_y - 0.5;
                let y0 = (gy.floor() as usize).clamp(0, src_h as usize - 1);
                let y1 = (y0 + 1).min(src_h as usize - 1);
                let fy = (gy - gy.floor()).clamp(0.0, 1.0);

                for sx in 0..dst_w {
                    let gx = (sx as f32 + 0.5) * scale_x - 0.5;
                    let x0 = (gx.floor() as usize).clamp(0, src_w as usize - 1);
                    let x1 = (x0 + 1).min(src_w as usize - 1);
                    let fx = (gx - gx.floor()).clamp(0.0, 1.0);

                    let a00 = src_raw[(y0 * (src_w as usize) + x0) * 4 + 3] as f32;
                    let a10 = src_raw[(y0 * (src_w as usize) + x1) * 4 + 3] as f32;
                    let a01 = src_raw[(y1 * (src_w as usize) + x0) * 4 + 3] as f32;
                    let a11 = src_raw[(y1 * (src_w as usize) + x1) * 4 + 3] as f32;

                    let a = ((1.0 - fx) * (1.0 - fy) * a00
                        + fx * (1.0 - fy) * a10
                        + (1.0 - fx) * fy * a01
                        + fx * fy * a11).round() as u8;

                    if a > 0 {
                        let sil_idx = sil_y * sil_stride + ((sx + pad) as usize) * 4;
                        sil[sil_idx] = cr;
                        sil[sil_idx + 1] = cg;
                        sil[sil_idx + 2] = cb;
                        sil[sil_idx + 3] = a;
                    }
                }
            }

            let mut scratch = vec![0u8; sil.len()];
            Self::apply_separable_blur_buffer(&mut sil, &mut scratch, sh_w, sh_h, radius);

            self.blend_raw(&sil, sh_w, sh_h, dst_x - pad_i32, dst_y - pad_i32, sh_w, sh_h, eff_opacity);
        }
    }

    /// Blend another FrameBuffer onto self with given opacity
    pub fn blend_framebuffer(&mut self, other: &FrameBuffer, opacity: f32) {
        if opacity <= 0.001 {
            return;
        }
        self.blend_raw(
            &other.data,
            other.width,
            other.height,
            0,
            0,
            other.width,
            other.height,
            opacity,
        );
    }

    /// Sine wave distortion effect matching MLT filter_wave.c
    pub fn apply_wave(
        &mut self,
        position: u64,
        factor: f32,
        speed: f32,
        deform_x: bool,
        deform_y: bool,
    ) {
        if factor <= 0.0 || (!deform_x && !deform_y) {
            return;
        }
        let w = self.width as i32;
        let h = self.height as i32;
        let amplitude = factor;
        let pulsation = 0.5 / factor;
        let phase = (position as f32) * pulsation * speed / 10.0;
        let src = self.data.clone();

        self.data
            .par_chunks_exact_mut((w as usize) * 4)
            .enumerate()
            .for_each(|(y_idx, row)| {
                let y = y_idx as i32;
                let decal_x = if deform_x {
                    ((pulsation * y as f32 + phase).sin() * amplitude).round() as i32
                } else {
                    0
                };
                for x in 0..w {
                    let decal_y = if deform_y {
                        ((pulsation * x as f32 + phase).sin() * amplitude).round() as i32
                    } else {
                        0
                    };
                    let sx = (x + decal_x).rem_euclid(w);
                    let sy = (y + decal_y).rem_euclid(h);
                    let src_idx = ((sy * w + sx) * 4) as usize;
                    let dst_slice = &mut row[(x as usize) * 4..(x as usize + 1) * 4];
                    dst_slice.copy_from_slice(&src[src_idx..src_idx + 4]);
                }
            });
    }

    /// Luma wipe transition alpha mask matching MLT composite/wipe
    pub fn apply_luma_wipe(
        &mut self,
        luma_map: &[u8],
        luma_w: u32,
        luma_h: u32,
        progress: f32,
        softness: f32,
        invert: bool,
    ) {
        let w = self.width as usize;
        let h = self.height as usize;
        let lw = luma_w as usize;
        let lh = luma_h as usize;
        let soft = softness.max(0.001);
        let prog = if invert { 1.0 - progress } else { progress };

        self.data
            .par_chunks_exact_mut(w * 4)
            .enumerate()
            .for_each(|(y, row)| {
                let ly = (y * lh / h).min(lh - 1);
                let luma_row = &luma_map[ly * lw..(ly + 1) * lw];
                for x in 0..w {
                    let lx = (x * lw / w).min(lw - 1);
                    let l_val = luma_row[lx] as f32 / 255.0;
                    let diff = prog - l_val;
                    let alpha_mult = ((diff + soft / 2.0) / soft).clamp(0.0, 1.0);
                    let px = &mut row[x * 4..(x + 1) * 4];
                    px[3] = (px[3] as f32 * alpha_mult).round() as u8;
                }
            });
    }
}
