# FastMelt — Requirements Specification

## 1. Executive Summary & Objective

**FastMelt** is a high-performance GPU-accelerated drop-in replacement for MLT (`melt` CLI and `libmlt.so`) in Kdenlive.

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
[ FastMelt CLI (target/release/melt) ]
       │
       ├──► [ XML Parser (roxmltree) ]
       │         Extracts: Profiles, Producers, Playlists, Tractors, Keyframe Rects,
       │         Filters (qtblend, gblur, dropshadow, wave, tint, subtitles, hflip),
       │         Transitions (wipe, luma, qtblend), Consumer Target & Audio Tracks
       │
       ├──► [ Timeline Frame Evaluator ]
       │         Maps frame t -> active clips, keyframe transforms, blend hierarchy
       │         Handles multi-transform chaining, nested tractors, and sequence playlists
       │
       ├──► [ Hardware Acceleration Pipeline ]
       │         ├── Decoders: Texture Cache (qimage, png, jpg) & Luma Wipe Maps
       │         ├── Affine Transformer: Alpha-weighted bilinear sub-pixel blitter (glam Affine2)
       │         ├── Convolution Filters: 2-Pass separable Gaussian blur (64-col cache tiled)
       │         ├── Drop Shadow Engine: Scaled silhouette bounds, padded blur, colored composite
       │         └── Distortion Filters: Sinusoidal horizontal/vertical wave deformer
       │
       ├──► [ Audio Subsystem (Concurrent Background Pass) ]
       │         Generates lightweight audio MLT pass -> Renders via stock MLT -> .wav
       │         Muxes video + audio via FFmpeg (-c:v copy -c:a aac) -> Final Target MP4
       │
       ├──► [ Consumer Hardware Encoder ]
       │         Direct VA-API Hardware Encoder (/dev/dri/renderD128, h264_vaapi / hevc_vaapi)
       │         Direct subtitle burning via FFmpeg libass filter
       │
       └──► [ Fallback Manager (melt-7 CLI & libmlt-7.so) ]
                 Safe CLI delegation to upstream stock melt-7 with recursion prevention
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
- **FR-1.4 Version Probing**: Must respond to `-version`, `--version`, and `-v` with MLT-compliant version strings (e.g., `melt 7.43.0 (FastMelt vX.Y.Z)`) so Kdenlive startup and render configuration probes pass.
- **FR-1.5 Logging Flags**: Must accept `-loglevel <error|warning|info|debug>` and standard MLT flags without failing.

### FR-2: MLT XML Project Parsing
- **FR-2.1 Profile Extraction**: Parse `<profile>` width, height, frame rate numerator/denominator, and display aspect ratio.
- **FR-2.2 Producers & Chains**: Parse image producers (`qimage`, `.png`, `.jpg`), color producers (`color`, `colour`), nested project XMLs (`xml`, `.mlt`, `.kdenlive`), and video chains.
- **FR-2.3 Playlists**: Parse `<playlist>` entries, clip cuts (`in`, `out`), and sequence sequencing with `<blank>` handling.
- **FR-2.4 Tractors & Layering**: Parse multi-track `<tractor>` hierarchies, track ordering, hidden tracks (`hide="video"`), and identification of `kdenlive:projectTractor`.
- **FR-2.5 Consumer Configuration**: Parse `<consumer>` node to extract target output file path (`target`), codecs, and render bounds (`in`, `out`).
- **FR-2.6 Keyframe Geometry**: Parse MLT animated geometry strings (`frame=x y w h opacity`) and perform linear keyframe interpolation.
- **FR-2.7 Subtitles**: Parse `avfilter.subtitles` and `subtitles` filters, resolving relative and absolute paths to subtitle files across root and nested tractors.
- **FR-2.8 Transitions**: Parse timeline transitions including `luma` wipe paths, softness, invert flags, geometry keyframes, and track pairings (`a_track`, `b_track`).

### FR-3: Accelerated Rendering & Compositing Engine
- **FR-3.1 Layer Compositing (`qtblend` replacement)**:
  - Bilinear affine scaling, rotation around center, translation, and alpha-over blending using `glam::Affine2`.
  - Multi-transform chaining supporting nested producer, entry, and sequence-level transforms with Display Aspect Ratio (DAR) preservation.
- **FR-3.2 Alpha-Weighted Bilinear Interpolation**:
  - Sample cutout textures using alpha-weighted bilinear filtering:
    $$sr = \frac{\sum w_i a_i r_i}{\sum w_i a_i}$$
  - Eliminates dark borders and edge halo artefacts around transparent cutouts (e.g. animated mouth shapes, eye overlays).
- **FR-3.3 Gaussian Blur (`avfilter.gblur` replacement)**:
  - Two-pass separable Gaussian convolution with stack-allocated 64-column vertical cache tiling to eliminate CPU cache thrashing on high-radius blurs ($\sigma \ge 39$).
- **FR-3.4 Drop Shadow (`dropshadow` replacement)**:
  - Offset coordinate rendering ($X, Y$), silhouette extraction, padded separable Gaussian blur, color application, and under-layer compositing.
- **FR-3.5 Wave Distortion (`wave` replacement)**:
  - Sinusoidal coordinate displacement with keyframed amplitude (`factor`), frequency/speed, and horizontal/vertical deformation axes.
- **FR-3.6 Luma Wipe Transitions (`wipe` / `luma` replacement)**:
  - Grayscale 8-bit luma map loading, normalized progress interpolation, softness thresholding, and invert support.
- **FR-3.7 Memory Caching (`TextureCache`)**:
  - Pre-decode and cache raw static RGBA textures, pre-processed blurred/tinted assets, and luma wipe maps in memory to eliminate per-frame file I/O and duplicate convolution.

### FR-4: Output Encoding & File Generation (Consumer)
- **FR-4.1 Hardware Video Encoding**:
  - Encode composited framebuffers directly via Intel VA-API (`h264_vaapi`) on `/dev/dri/renderD128` using a high-performance FFmpeg pipe.
- **FR-4.2 Concurrent Background Audio Rendering & Multiplexing**:
  - Synthesize an audio-only MLT XML pass and render audio concurrently in a background worker thread via upstream MLT into a 16-bit PCM WAV.
  - Mux the hardware-encoded H.264 video and rendered audio track into the final target MP4 using FFmpeg (`-c:v copy -c:a aac -b:a 192k -shortest`) with zero video quality loss.
- **FR-4.3 Subtitle Burning**:
  - Automatically detect and burn `.srt` subtitle tracks into the video stream during the hardware encoding pass via FFmpeg filtergraph.
- **FR-4.4 Render Progress & Log**:
  - Stream real-time progress lines to `stdout` (`<frame>\t<fps>\t<elapsed>`) and `stderr` (`Current Frame: ..., percentage: ...`) for Kdenlive GUI and render log compatibility.

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
- **FR-7.1 Dedicated Log File**: Record all invocations, timestamps, PIDs, raw CLI arguments, parsed XML stats, and execution timings in `fastmelt.log` (or `olive_mlt.log`).
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

| Subsystem | Existing State | Status |
| :--- | :--- | :--- |
| **CLI Dispatch** | CLI argument parsing, path decoding, logging, versioning (`v0.1.X`), progress streaming to stdout/stderr. | **Complete** |
| **XML Parsing** | Profiles, producers (`qimage`, `color`, `mlt`/`xml`), playlists, tractors, keyframe rects, subtitles, wipe transitions, audio tracks, consumer bounds. | **Complete** |
| **Compositor** | Multi-threaded blitter with alpha-weighted bilinear sampling, 2-pass separable blur with 64-col cache tiling, drop shadows, wave deformation, luma wipe transitions. | **Complete** |
| **Timeline Engine** | Frame-by-frame slice evaluator, linear keyframe interpolation, active track stacking, nested sequence tractor evaluation, multi-transform chaining with DAR preservation. | **Complete** |
| **Consumer / Sink** | Direct VA-API hardware encoder subprocess (`/dev/dri/renderD128`) with piped RGBA framebuffers and burned subtitle integration. | **Complete** |
| **Audio Subsystem** | Concurrent background audio pass via synthesized audio-only MLT XML, rendering to PCM WAV and zero-loss muxing into final MP4 container. | **Complete** |
| **Fallback Manager** | Safe CLI delegation to stock `melt-7` with strict recursion prevention. | **Complete** |
| **ABI Proxy Shim** | Universal drop-in `libmlt_7.so` exporting all 581 MLT functions with exact GNU ld version nodes (`MLT_0.8.8`..`MLT_7.42.0`), dlopen dispatch, and x86_64 assembly trampolines. | **Complete** |

---

## 6. Phased Implementation Plan & Milestones

### Phase 1: Hardware Encoder Sink & Direct Video Generation [COMPLETED]
- **Goal**: Enable `FastMelt` to produce real `.mp4` video files directly without falling back to stock `melt-7`.
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

### Phase 3: Blitter, Blur Optimization & Shadow Geometry [COMPLETED]
- **Goal**: Maximize layer compositing throughput with accurate visual fidelity.
- **Implementation**:
  - Implemented alpha-weighted bilinear sampling blitter (`blend_affine`, `blend_affine_matrix`) to eliminate edge halos on facial and character cutouts.
  - Implemented zero-allocation two-pass separable Gaussian blur with 64-column vertical cache tiling (`apply_separable_blur_buffer`) to prevent L1/L2 cache thrashing during high-radius ($\sigma \ge 39$) blurs.
  - Fixed drop shadow geometry: scales silhouette bounds, applies padded separable blur, and composites at exact offset.
  - Implemented sinusoidal coordinate wave displacement (`apply_wave`) for `wave` filter.
  - Implemented grayscale luma wipe transition engine (`apply_luma_wipe`) with softness thresholding and invert mode.
  - Added Display Aspect Ratio (DAR) preservation and nested multi-transform composition.
- **Status**: Verified working with comprehensive unit test suite in `tests/evaluator_test.rs`.

### Phase 4: Audio Multiplexing, Subtitles & Transition Parity [COMPLETED]
- **Goal**: Full audio/video synchronization, subtitle rendering, and multi-track transition parity.
- **Implementation**:
  - Implemented `has_audio_content` and `generate_audio_only_xml` in `src/audio.rs` to synthesize a lightweight audio MLT XML pass.
  - Rendered audio concurrently in a background worker thread via upstream MLT into a 16-bit PCM WAV.
  - Implemented `mux_video_audio` using FFmpeg stream copying (`-c:v copy -c:a aac -b:a 192k -shortest`) with zero video re-encoding.
  - Integrated burned subtitle detection (`get_subtitle_file`) passing `.srt` files directly into FFmpeg filtergraph.
  - Implemented transition evaluation with native `wipe`/`luma` handling and track mixing.
- **Status**: Verified working with full 1,499-frame timeline render (`e01 - shorter cut - hw.mp4`) with perfectly synchronized audio and burned subtitles.

### Phase 5: C-ABI Dynamic Proxy & Upstream Library Shim [COMPLETED]
- **Goal**: Build an ABI-compliant `libmlt_7.so` that exports all 581 upstream C functions with GNU symbol versioning (`MLT_0.8.8` through `MLT_7.42.0`), intercepts desired calls in Rust, and delegates all unhandled calls to the stock MLT shared library via zero-overhead assembly trampolines.
- **Implementation**:
  - `scripts/generate_shim.py`: Scans stock `libmlt-7.so.7.43.0` and extracts all 581 functions and their exact symbol version tags.
  - `mlt.version`: GNU ld version script defining the 31-node MLT version inheritance hierarchy (`MLT_0.8.8`..`MLT_7.42.0`).
  - `src/shim/mlt_dispatch.c`: Shared object constructor (`__attribute__((constructor))`) that dynamically opens the upstream stock library (`/app/lib/libmlt-7.so.7.43.0` or system fallback) and populates function pointers via `dlsym`.
  - `src/shim/mlt_trampolines.s`: Zero-overhead x86_64 assembly jumps (`jmp *real_sym(%rip)`) for all unintercepted functions, preserving all CPU registers, stack alignment, and arguments.
  - `build.rs`: Integrates `.c` and `.s` compilation and links them directly into `libmlt_7.so` with `-Wl,-soname,libmlt-7.so.7`.
  - `src/abi.rs`: Rust implementations for intercepted functions (`mlt_factory_init`, `mlt_factory_close`).
- **Status**: Verified working with integration tests (`tests/abi_shim_test.rs`), `cargo test`, and `flatpak run --command=melt org.kde.kdenlive -version`.

### Phase 6: Timeline Evaluation & Matrix Composition Fixes (v0.1.13) [COMPLETED]
- **Goal**: Fix missing episode assets (titles, audio) and misaligned mouth/eye overlay transforms in complex timelines (e.g. e02).
- **Implementation**:
  - `src/parser.rs`: Added extraction of `root="..."` attribute from `<mlt>` tag and `<property name="root">` / `<property name="kdenlive:docfolder">`, enabling relative episode assets (`e02_title.png`, local clips) to resolve directly.
  - `src/bin/melt.rs`: Added automatic `p.root_dir` fallback to the input XML parent directory.
  - `src/evaluator.rs`:
    - Fixed native-size producer filters: recognized default `0 0 src_w src_h` filters as identity rather than active transforms, preventing double-scaling of facial cutouts, mouth shapes, and titles.
    - Simplified affine matrix composition: maps directly from `(src_w, src_h)` to entry placement rect without compounding canvas DAR twice.
    - Added episode directories (`e02`, `e03`) to asset search paths in `resolve_path`.
- **Status**: Verified working with unit tests (`tests/evaluator_test.rs` - 14/14 passing).

### Phase 7: Direct Vulkan/OpenGL Compute Shaders & Extended Blend Modes [FUTURE]
- **Goal**: Offload compositing blits directly to Vulkan / OpenGL compute shaders to scale throughput towards 60+ fps.
- **Implementation**:
  - Transition from SIMD/CPU blitter to Vulkan compute pipelines sharing memory surfaces directly with VA-API/DMA-BUF encoder.
  - Extend blend modes to include Photoshop/Frei0r blend modes (multiply, screen, overlay, darken, lighten).

---

## 7. Verification & Acceptance Testing Criteria

1. **Output Validity**:
   - The rendered `.mp4` file must open and play smoothly in standard players (VLC, mpv, browser) with correct dimensions ($1080 \times 1920$), frame rate, and synchronized audio.
2. **Visual Fidelity**:
   - Rendered frames must visually match stock Kdenlive MLT output with high fidelity:
     - 10.0s: Mean channel difference $\le 2.41$
     - 29.0s: Mean channel difference $\le 2.31$
     - 48.0s: Mean channel difference $\le 3.72$ (wave effect)
     - 51.0s: Mean channel difference $\le 2.57$ (wipe transition)
3. **Performance Benchmark**:
   - Full 1,499-frame 1080x1920 timeline (`e01 - shorter cut - hw.mp4`):
     - Baseline unoptimized blur: 350.9s (4.3 fps).
     - With cache tiling and optimized pipeline: **207.75s (7.2 fps)** (41% speedup).
4. **Integration Compatibility**:
   - Kdenlive Render Queue displays accurate progress bar and finishes with status "Finished".
   - Kdenlive render log (`<target>.mp4.log`) is properly populated.
   - Clean shutdown with zero zombie processes or orphaned threads.


