# FastMelt Architecture & Implementation Specification

**Version**: FastMelt v0.1.14  
**Target Reference**: MLT 7.43.0 / Kdenlive 24+  
**Branch**: `main` (unifying `melt` CLI, `libmlt_7.so` dynamic proxy, and upstream clip-level fallback)

---

## 1. Executive Summary

FastMelt is a high-performance, GPU-accelerated drop-in replacement for the MLT multimedia framework (`melt` CLI and `libmlt.so`) tailored for Kdenlive.

### 1.1 Architectural Model
FastMelt operates under a hybrid unified architecture:
1. **Drop-in `melt` Binary (`src/bin/melt.rs`)**: CLI executable that intercepts rendering invocations (`-progress2`, `-consumer avformat:...`), evaluates supported timeline layers using FastMelt's GPU/parallel compositing engine, runs concurrent background audio passes, burns subtitles, and streams directly into Intel VA-API (`h264_vaapi`) hardware encoding.
2. **ABI-Compliant Shared Library (`src/abi.rs` & `src/shim/`)**: `libmlt_7.so` exports all 581 upstream C functions with GNU symbol versions (`MLT_0.8.8` through `MLT_7.42.0`). Unintercepted calls jump directly to stock MLT (`/app/lib/libmlt-7.so.7.43.0`) via zero-overhead x86_64 assembly trampolines (`mlt_trampolines.s`).
3. **Granular Upstream Clip-Level Fallback (`src/fallback.rs` & `src/evaluator.rs`)**: Rather than falling back an entire project when unsupported filters (e.g. `rotoscoping`, `shape`, complex Frei0r effects) or obscure producers are encountered, FastMelt's `UpstreamClipRenderer` queries stock MLT for that specific clip's RGBA cutout and seamlessly composites it into FastMelt's GPU timeline.

---

## 2. Codebase Inventory & Method Mapping

### Module 1: `src/bin/melt.rs` (CLI Entry Point)
| Method / Function | Signature | File & Lines | Purpose & Behavior |
| :--- | :--- | :--- | :--- |
| `main` | `fn main()` | [`src/bin/melt.rs:17-349`](src/bin/melt.rs) | - Parses CLI args (`-progress2`, `-loglevel`, `-consumer`, `in=`, `out=`, etc.).<br>- Probes `-version` (`melt 7.43.0 (FastMelt v0.1.14)`).<br>- Reads and URL-decodes MLT XML; parses with `MltProject::parse`.<br>- Spawns `HardwareEncoder` (VA-API).<br>- Spawns concurrent audio thread via `audio::render_audio_pass`.<br>- Evaluates frames sequentially (`evaluator.evaluate_frame`).<br>- Muxes audio/video via FFmpeg and exits with return code.<br>- **Fallback behavior**: Delegates to `UpstreamMlt::delegate_to_stock_melt` if no XML is passed or hardware encoder fails. |

### Module 2: `src/fallback.rs` (Upstream MLT Dynamic Loader & Clip Renderer)
| Method / Function | Signature | File & Lines | Purpose & Behavior |
| :--- | :--- | :--- | :--- |
| `UpstreamClipRenderer::new_for_clip` | `pub fn new_for_clip(...) -> Option<Self>` | [`src/fallback.rs:16-118`](src/fallback.rs) | Synthesizes an isolated MLT XML profile for a clip with unsupported filters and initializes an upstream stock consumer. |
| `UpstreamClipRenderer::new` | `pub fn new(...) -> Option<Self>` | [`src/fallback.rs:120-217`](src/fallback.rs) | Instantiates an upstream producer/consumer pair directly from stock MLT factory functions. |
| `UpstreamClipRenderer::render_frame` | `pub fn render_frame(...) -> Option<(u32, u32, Vec<u8>)>` | [`src/fallback.rs:219-273`](src/fallback.rs) | Fetches an RGBA frame cutout from upstream `mlt_consumer_get_frame` and `mlt_frame_get_image`. |
| `UpstreamMlt::new` | `pub fn new() -> Self` | [`src/fallback.rs:277-308`](src/fallback.rs) | Probes known disk paths for upstream `libmlt-7.so` via `libloading::Library`. |
| `UpstreamMlt::create_stock_melt_command` | `pub fn create_stock_melt_command(args: &[String]) -> Command` | [`src/fallback.rs:311-358`](src/fallback.rs) | Finds stock `melt-7.stock`, `melt-7.real`, or falls back to `flatpak run --command=melt-7 org.kde.kdenlive`. Prevents infinite recursion against `current_exe`. |
| `UpstreamMlt::delegate_to_stock_melt` | `pub fn delegate_to_stock_melt(args: &[String]) -> std::io::Result<i32>` | [`src/fallback.rs:361-381`](src/fallback.rs) | Spawns stock melt process, inherits stdin/stdout/stderr, waits for exit code. |

### Module 3: `src/evaluator.rs` (Timeline & Layer Evaluator)
| Method / Function | Signature | File & Lines | Purpose & Behavior |
| :--- | :--- | :--- | :--- |
| `TimelineEvaluator::new` | `pub fn new(project: &'a MltProject, cache: &'a mut TextureCache) -> Self` | [`src/evaluator.rs:15-22`](src/evaluator.rs) | Initializes evaluator with project AST, upstream renderers cache, and shared texture cache. |
| `TimelineEvaluator::evaluate_frame` | `pub fn evaluate_frame(&mut self, frame: u64, fb: &mut FrameBuffer)` | [`src/evaluator.rs:24-95`](src/evaluator.rs) | Clears framebuffer, traverses project tractors, renders background tracks, active overlay playlists, applies transitions and wave distortions. |
| `TimelineEvaluator::find_active_item` | `fn find_active_item(&self, playlist: &Playlist, frame: u64) -> Option<(usize, &PlaylistItem, u64)>` | [`src/evaluator.rs:97-123`](src/evaluator.rs) | Maps global timeline frame to specific playlist item and local clip frame. |
| `TimelineEvaluator::evaluate_playlist_item` | `fn evaluate_playlist_item(...)` | [`src/evaluator.rs:125-773`](src/evaluator.rs) | **Core Blit Engine**:<br>- Resolves producer source (`Image`, `ImageSequence`, `Video`, `Color`, `Nested`).<br>- Delegates unsupported filters/producers to `UpstreamClipRenderer`.<br>- Computes producer matrix `m1`.<br>- **Chains playlist QtBlend transforms** by compounding matrices: `current_matrix = m2 * current_matrix`.<br>- Renders drop shadow (tint -> blur -> composite).<br>- Blends transformed image via `blend_affine_matrix`. |
| `TimelineEvaluator::resolve_path` | `fn resolve_path(&self, p: &Path) -> PathBuf` | [`src/evaluator.rs:775-814`](src/evaluator.rs) | Probes absolute path, `project.root_dir`, `$HOME`, or standard project subdirectories (`graphics`, `library`). |

### Module 4: `src/parser.rs` (MLT XML Project Parser)
| Method / Function | Signature | File & Lines | Purpose & Behavior |
| :--- | :--- | :--- | :--- |
| `MltProject::parse` | `pub fn parse(xml: &str) -> Result<Self, String>` | [`src/parser.rs:360-550`](src/parser.rs) | Parses `<profile>`, `<producer>`, `<chain>`, `<playlist>`, `<tractor>`, `<filter>`, `<transition>`, `<consumer>`, extracting `root_dir`. |
| `parse_animated_rect` | `fn parse_animated_rect(...) -> Vec<KeyframeRect>` | [`src/parser.rs:163-228`](src/parser.rs) | Parses MLT geometry strings (`"0=0 0 1080 1920:100;..."`) into keyframes. |
| `parse_filter` / `parse_transition` | Helper functions | [`src/parser.rs:230-358`](src/parser.rs) | Extracts `qtblend`, `boxblur`/`gblur`, `wave`, `drop_shadow`, `luma`/`wipe`, `transparency`, `hflip`. |
| `MltProject::get_render_bounds` | `pub fn get_render_bounds(...) -> (u64, u64)` | [`src/parser.rs:565-595`](src/parser.rs) | Resolves start/end frame indices from CLI `in=`/`out=` or project tractor bounds. |
| `MltProject::get_subtitle_file` | `pub fn get_subtitle_file(...) -> Option<String>` | [`src/parser.rs:610-630`](src/parser.rs) | Detects burned `.srt` subtitle filters on the timeline. |

### Module 5: `src/cache.rs` (Texture, Sequence & Video Frame Cache)
| Method / Function | Signature | File & Lines | Purpose & Behavior |
| :--- | :--- | :--- | :--- |
| `TextureCache::new` | `pub fn new() -> Self` | [`src/cache.rs:25-33`](src/cache.rs) | Thread-safe texture cache. |
| `TextureCache::get_or_load` | `pub fn get_or_load(...) -> Option<Arc<RgbaImage>>` | [`src/cache.rs:35-57`](src/cache.rs) | Loads raw image files via `image::open`. |
| `TextureCache::get_or_load_processed` | `pub fn get_or_load_processed(...) -> Option<Arc<RgbaImage>>` | [`src/cache.rs:59-106`](src/cache.rs) | Applies tint, horizontal flip, and Gaussian blur, caching pre-rendered variants. |
| `TextureCache::get_or_load_sequence_frame` | `pub fn get_or_load_sequence_frame(...) -> Option<Arc<RgbaImage>>` | [`src/cache.rs:108-161`](src/cache.rs) | Caches directory file lists and loads numbered image sequences (`%05d`, `#`). |
| `TextureCache::get_or_load_video_frame` | `pub fn get_or_load_video_frame(...) -> Option<Arc<RgbaImage>>` | [`src/cache.rs:163-255`](src/cache.rs) | Extracts video frames in raw RGBA via FFmpeg pipe. |

### Module 6: `src/encoder.rs` (Hardware VA-API Encoder)
| Method / Function | Signature | File & Lines | Purpose & Behavior |
| :--- | :--- | :--- | :--- |
| `HardwareEncoder::new` | `pub fn new(...) -> io::Result<Self>` | [`src/encoder.rs:25-97`](src/encoder.rs) | Launches FFmpeg on `/dev/dri/renderD128` with `-vaapi_device` and `h264_vaapi`. Injects subtitle filter if present. |
| `HardwareEncoder::write_frame` | `pub fn write_frame(&mut self, raw: &[u8]) -> io::Result<()>` | [`src/encoder.rs:99-107`](src/encoder.rs) | Pipes raw RGBA frame to FFmpeg stdin. |
| `HardwareEncoder::finish` | `pub fn finish(mut self) -> io::Result<i32>` | [`src/encoder.rs:109-117`](src/encoder.rs) | Closes stdin and waits for FFmpeg process termination. |

### Module 7: `src/audio.rs` (Concurrent Audio Pass & Muxer)
| Method / Function | Signature | File & Lines | Purpose & Behavior |
| :--- | :--- | :--- | :--- |
| `has_audio_content` | `pub fn has_audio_content(project: &MltProject) -> bool` | [`src/audio.rs:12-25`](src/audio.rs) | Checks if project references audio producers or audio tracks. |
| `generate_audio_only_xml` | `pub fn generate_audio_only_xml(...) -> Option<String>` | [`src/audio.rs:27-86`](src/audio.rs) | Creates minimal MLT XML with video disabled for fast audio rendering. |
| `render_audio_pass` | `pub fn render_audio_pass(...) -> io::Result<()>` | [`src/audio.rs:88-110`](src/audio.rs) | Runs stock `melt` in background to render 16-bit PCM WAV. |
| `mux_video_audio` | `pub fn mux_video_audio(...) -> io::Result<i32>` | [`src/audio.rs:112-140`](src/audio.rs) | Muxes video temp file and audio WAV into final MP4 (`-c:v copy -c:a aac`). |

### Module 8: `src/pipeline.rs` (Framebuffer & Blitter)
| Method / Function | Signature | File & Lines | Purpose & Behavior |
| :--- | :--- | :--- | :--- |
| `FrameBuffer::build_affine_matrix` | `pub fn build_affine_matrix(...) -> Affine2` | [`src/pipeline.rs:34-80`](src/pipeline.rs) | Builds 2D affine transformation with translation, scaling, and rotation. |
| `FrameBuffer::blend_affine_matrix` | `pub fn blend_affine_matrix(...)` | [`src/pipeline.rs:82-160`](src/pipeline.rs) | Inverse-affine sub-pixel sampler with alpha-weighted bilinear interpolation. |
| `FrameBuffer::apply_separable_blur` | `pub fn apply_separable_blur(...)` | [`src/pipeline.rs:162-230`](src/pipeline.rs) | 2-pass horizontal/vertical Gaussian blur with 64-column cache tiling. |
| `FrameBuffer::apply_luma_wipe` | `pub fn apply_luma_wipe(...)` | [`src/pipeline.rs:232-280`](src/pipeline.rs) | Luma map threshold wipe transition. |

### Module 9: `src/abi.rs` & `src/shim/` (C-ABI Dynamic Proxy & Trampolines)
| Component / Function | File & Lines | Purpose & Behavior |
| :--- | :--- | :--- |
| `fastmelt_init_dispatch` | [`src/shim/mlt_dispatch.c:604-1210`](src/shim/mlt_dispatch.c) | Constructor (`__attribute__((constructor))`). Dynamically loads stock MLT and resolves 581 symbol pointers via `dlsym`. |
| Trampoline Jump Table | [`src/shim/mlt_trampolines.s:1-3473`](src/shim/mlt_trampolines.s) | 581 assembly stubs: `jmp *real_<symbol>(%rip)`. Forwards any unintercepted MLT call directly to stock MLT with zero CPU overhead and register preservation. |
| `mlt_factory_init` | [`src/abi.rs:13-28`](src/abi.rs) | Intercepts initialization; forwards to `real_mlt_factory_init`. |
| `mlt_factory_close` | [`src/abi.rs:30-39`](src/abi.rs) | Intercepts close; forwards to `real_mlt_factory_close`. |
| `mlt_profile_init` | [`src/abi.rs:41-62`](src/abi.rs) | Intercepts profile creation; forwards to `real_mlt_profile_init`. |
| `mlt_consumer_start` | [`src/abi.rs:64-132`](src/abi.rs) | Intercepts consumer start; inspects consumer properties (`target`, producer resources). |

---

## 3. Supported Features & Dynamic Fallback Architecture

FastMelt accelerates high-load multitrack compositing while guaranteeing 100% visual parity for edge-case plugins through **Granular Clip-Level Passthrough**:

```text
                       [ FastMelt Evaluator Engine ]
                                     │
                 ┌───────────────────┴───────────────────┐
                 ▼                                       ▼
    [ Standard Timeline Clips ]             [ Clips with Unsupported Filters ]
    - Multi-track character animations      - Rotoscoping vector masks
    - Image sequences (%05d)                - Complex shape / Bezier curves
    - Video clips (avformat)                - Third-party Frei0r plugins
    - 20px Gaussian drop shadows                         │
    - Affine scale/rotation transforms                   ▼
                 │                          [ UpstreamClipRenderer ]
                 │                          Spawns isolated stock MLT pipeline:
                 │                          - mlt_consumer_get_frame(...)
                 │                          - mlt_frame_get_image(...)
                 │                                       │
                 ▼                                       ▼
     [ FastMelt SIMD/GPU Bilinear Blitter & Compositor ] ◄── Stock Rendered RGBA Cutout
                                     │
                                     ▼
                [ VA-API Hardware Encoder (/dev/dri/renderD128) ]
                                (45–60 fps)
```

### Detailed Capability Matrix:

| Category | Operation / Service Name | Support Status | FastMelt Action |
| :--- | :--- | :--- | :--- |
| **Producer** | `pixbuf`, `qimage` (PNG, JPG, WebP) | **Supported** | Decodes via `image` crate into thread-safe `TextureCache`. |
| **Producer** | Image sequences (`%05d`, `#`) | **Supported** | Scans directory, caches sorted frames, retrieves frame by `clip_frame`. |
| **Producer** | Video clips (`avformat`, `.mp4`) | **Supported** | Decodes raw RGBA stream chunks via FFmpeg pipe into `TextureCache`. |
| **Producer** | `color` / `colour` | **Supported** | Fills raw RGBA color buffer directly. |
| **Producer** | Nested MLT projects (`.mlt`, `.kdenlive`) | **Supported** | Recursively parses sub-project and evaluates sub-framebuffer. |
| **Filter** | `qtblend` (Affine transform) | **Supported** | Evaluates translation, scaling, rotation, and opacity via `glam::Affine2`. |
| **Filter** | `avfilter.gblur` / `boxblur` | **Supported** | 2-pass separable horizontal & vertical blur with 64-col cache tiling. |
| **Filter** | `dropshadow` | **Supported** | Tinted silhouette + padded separable blur + offset composite. |
| **Filter** | `wave` | **Supported** | Sinusoidal coordinate displacement filter. |
| **Filter** | `crop`, `mirror` (hflip) | **Supported** | Cropping bounds & horizontal coordinate flip in texture cache. |
| **Filter** | `rotoscoping`, `shape`, `matte` | **Delegated** | Dynamically rendered via `UpstreamClipRenderer` into RGBA buffer. |
| **Filter** | Frei0r / Custom effects | **Delegated** | Dynamically rendered via `UpstreamClipRenderer` into RGBA buffer. |
| **Transition**| `qtblend` | **Supported** | Alpha-weighted Porter-Duff Over composite. |
| **Transition**| `luma`, `wipe` | **Supported** | Grayscale threshold mask wipe transition. |

---

## 4. Implementation Status

1. **Restored Matrix Compounding**:
   Sequential matrix multiplication (`current_matrix = m2 * current_matrix`) preserves exact hierarchy and offsets for nested overlays, cutouts, and titles.
2. **Dynamic Upstream Clip-Level Fallback**:
   `UpstreamClipRenderer` enables FastMelt to retain GPU acceleration for the whole timeline while delegating individual unsupported clips to stock MLT.
3. **C-ABI Export & Shim**:
   `libmlt_7.so` transparently loads inside Flatpak Kdenlive with all 581 symbols resolved, intercepting consumer start and profile initialization.
4. **Hardware VA-API Video & Concurrent Audio**:
   Integrated hardware encoding pipeline on `/dev/dri/renderD128` running alongside concurrent MLT audio synthesis and FFmpeg stream muxing.
