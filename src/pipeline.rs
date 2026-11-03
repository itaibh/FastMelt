use image::RgbaImage;
use rayon::prelude::*;

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

    /// Parallel integer fixed-point alpha-over blitter with bilinear sampling
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

    /// Blend an image source onto this framebuffer
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

    /// Blend another framebuffer onto this framebuffer
    pub fn blend_buffer(
        &mut self,
        source: &FrameBuffer,
        dst_x: i32,
        dst_y: i32,
        dst_w: u32,
        dst_h: u32,
        opacity: f32,
    ) {
        self.blend_raw(
            &source.data,
            source.width,
            source.height,
            dst_x,
            dst_y,
            dst_w,
            dst_h,
            opacity,
        );
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

        let radius_int = radius.round() as i32;
        let kernel_size = (radius_int * 2 + 1) as usize;
        let mut kernel = vec![0.0f32; kernel_size];
        let sigma = radius / 2.0;
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

        // Pass 2: Vertical Blur (parallel across rows reading from scratch)
        let scratch_ref = &scratch[..stride * h];
        data[..stride * h]
            .par_chunks_exact_mut(stride)
            .enumerate()
            .for_each(|(y, out_row)| {
                for x in 0..w {
                    let mut r = 0.0f32;
                    let mut g = 0.0f32;
                    let mut b = 0.0f32;
                    let mut a = 0.0f32;

                    for (ki, &weight) in k_ref.iter().enumerate() {
                        let offset = ki as i32 - radius_int;
                        let sample_y = (y as i32 + offset).clamp(0, h as i32 - 1) as usize;
                        let s_idx = sample_y * stride + x * 4;
                        r += scratch_ref[s_idx] as f32 * weight;
                        g += scratch_ref[s_idx + 1] as f32 * weight;
                        b += scratch_ref[s_idx + 2] as f32 * weight;
                        a += scratch_ref[s_idx + 3] as f32 * weight;
                    }

                    let d_idx = x * 4;
                    out_row[d_idx] = r.round().clamp(0.0, 255.0) as u8;
                    out_row[d_idx + 1] = g.round().clamp(0.0, 255.0) as u8;
                    out_row[d_idx + 2] = b.round().clamp(0.0, 255.0) as u8;
                    out_row[d_idx + 3] = a.round().clamp(0.0, 255.0) as u8;
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
            let pad = (radius * 1.5).ceil() as u32;
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
                let src_y = (((sy as f32) * scale_y) as usize).min(src_h as usize - 1);
                let sil_y = (sy + pad) as usize;
                for sx in 0..dst_w {
                    let src_x = (((sx as f32) * scale_x) as usize).min(src_w as usize - 1);
                    let a = src_raw[(src_y * (src_w as usize) + src_x) * 4 + 3];
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
}
