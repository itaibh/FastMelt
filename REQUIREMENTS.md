# OliveMltBridge (Fast-Melt) — Requirements Specification

## 1. Executive Summary & Objective

**OliveMltBridge** is a high-performance GPU-accelerated bridge and drop-in replacement for MLT (`melt` CLI and `libmlt.so`) in Kdenlive.

### 1.1 The Core Problem
In stock Kdenlive, rendering and timeline evaluation are handled by the CPU-based MLT framework. Multi-track vertical HD timelines (e.g. animated lip-sync projects with 50+ tracks, 500+ layer blends, drop shadows, and 20px Gaussian blurs) suffer from severe rendering bottlenecks:
- Single-threaded CPU raster compositing (`qtblend`, `QPainter`).
- Inefficient CPU memory lookups for large-radius separable blurs and shadows.
- Render speeds throttle to **~7 fps** while modern GPU hardware (e.g., Intel Iris Xe / `/dev/dri/renderD128`) sits idle at ~1% utilization.
- Stock render of complex projects takes minutes (e.g., ~154 seconds for short clips).

### 1.2 Target Goal
Accelerate render speeds from **~7 fps** to **45–60+ fps** by moving layer transforms, blending, convolution blurs, and encoding to GPU hardware acceleration, while maintaining 100% drop-in compatibility with Kdenlive's CLI and Flatpak packaging.

---

## 2. System Architecture

```text
[ Kdenlive GUI / Flatpak ]
       │
       ▼ (Invokes: melt -loglevel error -progress2 %2Fvar%2Ftmp%2Fproject.mlt)
[ OliveMltBridge CLI (target/release/melt) ]
       │
       ├──► [ XML Parser (roxmltree) ]
       │         Extracts: Profiles, Producers, Playlists, Tractors, Keyframe Rects, Filters, Consumer Target
       │
       ├──► [ Timeline Frame Evaluator ]
       │         Maps frame t -> active clips, keyframe transforms, blend hierarchy
       │
       ├──► [ Hardware Acceleration Pipeline ]
       │         ├── Decoders: Image Texture Cache (qimage) / Video HW decode
       │         ├── Quad Shader: Affine keyframe transforms & alpha blending (qtblend)
       │         └── Compute Shaders: 2-Pass separable Gaussian blur & drop shadow
       │
       ├──► [ Consumer Hardware Encoder ]
       │         Direct VA-API Hardware Encoder (/dev/dri/renderD128, h264_vaapi / hevc_vaapi)
       │         Audio muxing (AAC / MP3) -> Output File (e.g., renders/e03.mp4)
       │
       └──► [ Fallback Manager (dlopen libmlt-7.so / melt-7) ]
                 Graceful fallback for unsupported MLT filters, transitions, or legacy services
```

---

## 3. Functional Requirements

### FR-1: Drop-In CLI Interface (`melt`)
- **FR-1.1 Compatible Path**: Must execute directly as the drop-in replacement binary located at `/app/bin/melt` in the Kdenlive Flatpak environment.
- **FR-1.2 Progress Reporting**: Must stream progress updates to `stdout` in the exact tab-delimited format expected by Kdenlive:
  ```text
  <current_frame>\t<fps>\t<elapsed_seconds>
  ```
- **FR-1.3 Path Decoding**: Must support and automatically decode URL-encoded file paths passed by Kdenlive (e.g., `%2Fvar%2Ftmp%2Fproject.mlt` -> `/var/tmp/project.mlt`).
- **FR-1.4 Version Probing**: Must respond to `-version`, `--version`, and `-v` with MLT-compliant version strings (e.g., `melt 7.43.0 (OliveMltBridge vX.Y.Z)`) so Kdenlive startup and render configuration probes pass.
- **FR-1.5 Logging Flags**: Must accept `-loglevel <error|warning|info|debug>` and standard MLT flags without failing.

### FR-2: MLT XML Project Parsing
- **FR-2.1 Profile Extraction**: Parse `<profile>` width, height, frame rate numerator/denominator, and display aspect ratio.
- **FR-2.2 Producers & Chains**: Parse image producers (`qimage`, `.png`, `.jpg`), video chains (`avformat`), color producers, in/out frame bounds, and attached filters.
- **FR-2.3 Playlists**: Parse `<playlist>` entries, clip cuts (`in`, `out`), and sequence sequencing.
- **FR-2.4 Tractors & Layering**: Parse multi-track `<tractor>` hierarchies, track ordering, hidden tracks, and identification of `kdenlive:projectTractor`.
- **FR-2.5 Consumer Configuration**: Parse `<consumer>` node to extract:
  - Target output file path (`target`).
  - Target video codec (`vcodec`), audio codec (`acodec`), bitrates, and encoding parameters.
- **FR-2.6 Keyframe Geometry**: Parse MLT animated geometry strings (`frame=x y w h opacity`) and perform linear/smooth keyframe interpolation.

### FR-3: Accelerated Rendering & Compositing Engine
- **FR-3.1 Layer Compositing (`qtblend` replacement)**:
  - Bilinear affine scaling, rotation, translation, and alpha-over blending.
  - Target: Execute via GPU quad shaders / compute blitter.
- **FR-3.2 Gaussian Blur (`avfilter.gblur` replacement)**:
  - Two-pass separable Gaussian convolution (horizontal pass + vertical pass).
  - Target: $O(2 \times R)$ complexity instead of CPU $O(R^2)$ memory lookups.
- **FR-3.3 Drop Shadow (`dropshadow` replacement)**:
  - Offset coordinate rendering ($X, Y$), blur expansion, color application, and under-layer compositing.
- **FR-3.4 Texture Cache**:
  - Pre-decode and cache static image producers (e.g., PNG lip-sync mouth shapes, character poses) in GPU memory/RAM textures to avoid re-decoding per frame.

### FR-4: Output Encoding & File Generation (Consumer)
- **FR-4.1 Hardware Video Encoding**:
  - Encode composited framebuffers directly via Intel VA-API (`h264_vaapi` or `hevc_vaapi`) on `/dev/dri/renderD128`, or via high-performance piped FFmpeg sink.
  - Must write valid, playable video files directly to the target destination path specified in the MLT `<consumer target="...">`.
- **FR-4.2 Audio Multiplexing**:
  - Mix and mux timeline audio tracks (`.mp3`, `.wav`, audio chains) into the final container.
- **FR-4.3 Render Log**:
  - Ensure standard output and error stream properly so Kdenlive creates the render log file (`<target>.mp4.log`).

### FR-5: Transparent Dynamic Fallback
- **FR-5.1 Upstream Engine Proxy**:
  - When encountering unhandled complex MLT trees or unsupported transitions, gracefully delegate execution to `/app/bin/melt-7` with full argument fidelity and inherited pipes.
- **FR-5.2 Infinite Recursion Protection**:
  - Ensure fallback candidates strictly exclude the running executable to prevent recursion.
- **FR-5.3 Dynamic Library Proxy (`libmlt-7.so`)**:
  - Dynamically load upstream `libmlt-7.so` via `dlopen` to proxy unsupported individual filters.

### FR-6: C-ABI Compatibility (`libmlt.so`)
- **FR-6.1 Exported ABI Symbols**: Export standard MLT symbols for timeline preview and Kdenlive linkage:
  - `mlt_factory_init`, `mlt_factory_close`
  - `mlt_profile_init`, `mlt_profile_load_file`
  - `mlt_producer_init`, `mlt_tractor_init`, `mlt_consumer_init`
  - `mlt_consumer_start`, `mlt_consumer_stop`

### FR-7: Observability & Diagnostics
- **FR-7.1 Dedicated Log File**: Record all invocations, timestamps, PIDs, raw CLI arguments, parsed XML stats, and execution timings in `olive_mlt.log`.
- **FR-7.2 Version Stamping**: Every binary build must carry a bumped version string (`v0.1.X`), logged on every invocation header.
- **FR-7.3 Status Scripting**: Maintain `./switch.sh status` to inspect current symlinks and binary targets.

---

## 4. Non-Functional Requirements

- **Performance**: Render throughput $\ge 45\text{--}60\text{ fps}$ on 1080x1920 vertical timelines (versus ~7 fps stock baseline).
- **GPU Acceleration**: Active utilization of `/dev/dri/renderD128` (Intel Iris Xe) for compositing and encoding.
- **Flatpak Compatibility**: Zero external host dependencies; must run inside Flatpak's sandboxed filesystem.
- **Robustness**: If hardware acceleration fails, automatically fall back to stock MLT without failing the render job.
- **Integrity**: Exact visual alignment with Kdenlive coordinates and alpha compositing.

---

## 5. Current Implementation Gap Analysis

| Subsystem | Existing State | Gap / Needed Work |
| :--- | :--- | :--- |
| **CLI Dispatch** | CLI argument parsing, path decoding, logging, versioning, progress streaming. | Complete for render jobs and version queries. |
| **XML Parsing** | Profiles, producers, playlists, tractors, filters, consumer target, frame calculation. | Extract audio chains and volume curves. |
| **Compositor** | Parallel multi-threaded SIMD blitter, 2-pass separable blur, geometry-accurate drop shadow, texture & blur caching. | Complete (Phase 3). |
| **Timeline Engine** | Frame-by-frame slice evaluator, linear keyframe interpolation, active track stacking. | Complete (Phase 2). |
| **Consumer / Sink** | Direct VA-API hardware encoder subprocess (`/dev/dri/renderD128`) with piped RGBA framebuffers. | Complete (Phase 1). |
| **Audio** | Not implemented. | Mix timeline audio and mux with video track in container. |

---

## 6. Phased Implementation Plan & Milestones

### Phase 1: Hardware Encoder Sink & Direct Video Generation [COMPLETED]
- **Goal**: Enable `OliveMltBridge` to produce real `.mp4` video files directly without falling back to stock `melt-7`.
- **Implementation**:
  - Connect `FrameBuffer` to a hardware-accelerated FFmpeg VA-API consumer subprocess (`ffmpeg -f rawvideo -pix_fmt rgba -s <w>x<h> -r <fps> -i - -vaapi_device /dev/dri/renderD128 -vf 'format=nv12,hwupload' -c:v h264_vaapi <target>`).
  - Feed composited frames into the encoder pipe sequentially.
  - Stream progress to `stdout` in real time.
- **Status**: Verified working with hardware VA-API acceleration on `/dev/dri/renderD128`, outputting valid H.264 MP4 videos directly.

### Phase 2: Frame-by-Frame Timeline Slice Evaluator [COMPLETED]
- **Goal**: Accurately evaluate active timeline producers, cuts, and keyframe transitions at frame $t$.
- **Implementation**:
  - Implemented `TimelineEvaluator` in `src/evaluator.rs` with `evaluate_frame(t, fb)`.
  - Added `TextureCache` in `src/cache.rs` to cache static image textures in memory.
  - Resolved keyframe rects and opacity curves using linear interpolation for affine transforms (`qtblend`).
  - Implemented playlist active clip resolution taking `<blank length="..."/>` and clip `in`/`out` cuts into account.
  - Stack and blend all active layers for frame $t$ in correct track order (nested tractors, playlists, tracks).
- **Status**: Verified working with unit tests (`tests/evaluator_test.rs`) and end-to-end multi-track animated rendering test at 64.7 fps.

### Phase 3: GPU/SIMD Blitter & Shader Integration [COMPLETED]
- **Goal**: Reach target render throughput of $\ge 45\text{--}60\text{ fps}$ with accurate layer compositing.
- **Implementation**:
  - Implemented parallel integer fixed-point alpha-over blitter with bilinear sampling (`blend_raw`, `blend_layer`, `blend_buffer`).
  - Implemented zero-allocation two-pass separable Gaussian blur (`apply_separable_blur_buffer`) across all worker threads.
  - Fixed drop shadow geometry: scales silhouette to transformed dimensions, applies padded separable blur, and composites at exact offset.
  - Added blurred texture cache in `TextureCache` to avoid re-blurring static assets on every frame.
  - Added solid color & transparency rectangle fills (`fill_rect`).
- **Status**: Verified working with 5 unit tests in `tests/evaluator_test.rs`.

### Phase 4: Audio Multiplexing & Full Feature Parity [CURRENT]
- **Goal**: Full audio/video sync and production delivery.
- **Implementation**:
  - Extract audio producers and volume keyframes.
  - Multi-threaded mixing or direct FFmpeg audio stream muxing into the final container.

---

## 7. Verification & Acceptance Testing Criteria

1. **Output Validity**:
   - The rendered `.mp4` file must open and play smoothly in standard players (VLC, mpv, browser) with correct dimensions ($1080 \times 1920$) and frame rate.
2. **Visual Fidelity**:
   - Rendered frames must visually match stock Kdenlive MLT output within negligible color-rounding tolerances ($\Delta E < 1.0$).
3. **Performance Benchmark**:
   - Measure execution time on the benchmark project (`e03.kdenlive` / `e03.mlt`).
   - Baseline stock MLT: ~154 seconds (~7 fps).
   - Target acceptance: $\le 30\text{--}40\text{ seconds}$ ($\ge 35\text{--}60\text{ fps}$).
4. **Integration Compatibility**:
   - Kdenlive Render Queue displays accurate progress bar and finishes with status "Finished".
   - Kdenlive render log (`<target>.mp4.log`) is properly populated.
   - Zero crashes, infinite loops, or zombie processes.


