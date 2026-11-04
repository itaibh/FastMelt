# FastMelt

A high-performance GPU-accelerated drop-in replacement for MLT (`melt` and `libmlt.so`) in Kdenlive.

## The Problem It Solves
Kdenlive delegates all rendering and timeline evaluation to the MLT framework. MLT processes compositing, transitions, scaling, and convolution filters (like 20px Gaussian blurs and drop shadows) on the CPU using single-threaded raster algorithms (`qtblend`, `QPainter`). 

On complex multi-track vertical HD timelines (e.g., animated lip-sync projects with 500+ layer blends and drop shadows), this throttles render speed to **~7 fps** while the GPU sits at 1% usage.

`FastMelt` bridges this gap:
1. **Drop-in `melt` Binary**: Replaces `/app/bin/melt`. Parses Kdenlive's `.mlt` XML projects and renders them using GPU shaders and direct VA-API hardware encoding.
2. **GPU Render Graph**:
   - Multitrack compositing (`qtblend` transforms) executed as GPU quad shaders.
   - 20-pixel Drop Shadows (`dropshadow`) and Gaussian Blurs (`avfilter.gblur`) executed as two-pass separable GPU compute shaders with cache tiling.
   - Decoded video and PNG sequences stream directly into GPU textures.
   - Final framebuffers are encoded directly via VA-API (`h264_vaapi`) with zero CPU-RAM round trips.
3. **Transparent Fallback**: Any unsupported legacy MLT filter or transition is dynamically proxied to the system's upstream `libmlt-7.so` via `dlopen` or stock melt.
4. **ABI-Compliant `libmlt.so`**: Exports the C-ABI expected by Kdenlive for timeline playback acceleration.

---

## Architecture

```text
[ Kdenlive ]
     │ (Invokes: melt -progress2 project.mlt)
     ▼
[ FastMelt / melt CLI ]
     │
     ├──► [ MLT XML Parser ] (roxmltree)
     │         Extracts Tractors, Playlists, Rect Keyframes, and Filters
     │
     ├──► [ GPU Render Graph ]
     │         ├── Shaders: Transform (Affine Keyframes), Alpha-Over Blend
     │         ├── Shaders: 2-Pass Separable Gaussian Blur & Drop Shadow
     │         └── Decoders: Hardware VA-API / Image Texture Cache
     │
     ├──► [ Hardware Encoder Pipeline ] ──► Intel Iris Xe (/dev/dri/renderD128)
     │
     └──► [ Fallback Manager (dlopen libmlt-7.so) ] ──► Upstream MLT for legacy plugins
```

---

## Supported Core Services

| MLT Service | FastMelt Handling | Acceleration Method |
| :--- | :--- | :--- |
| `qimage` | PNG / JPG image layers | Uploaded to GPU texture cache once |
| `qtblend` | Timeline transforms / transitions | GPU affine transform vertex/fragment shader |
| `dropshadow` | Drop shadow with radius & offset | 2-pass separable GPU Gaussian blur shader |
| `avfilter.gblur` | Gaussian blur | Fast GPU separable convolution shader |
| `wave` | Sinusoidal wave deformation | Real-time coordinate displacement |
| `wipe` / `luma` | Grayscale wipe transitions | Luma threshold shader |
| `mix` | Audio track mixing | Concurrent background MLT pass + FFmpeg mux |
| `avformat` | Consumer export | Direct VA-API H.264/HEVC hardware encoder |
| *Other* | Any unsupported filter | Dynamically proxied to upstream `libmlt-7.so` |

---

## Project Structure

```text
FastMelt/
├── Cargo.toml          # Rust configuration (produces 'melt' bin and 'mlt-7' cdylib)
├── README.md           # This documentation
└── src/
    ├── lib.rs          # Library root & proxy fallback
    ├── bin/
    │   └── melt.rs     # CLI drop-in replacing /app/bin/melt
    ├── parser.rs       # XML parser for .mlt project files
    ├── evaluator.rs    # Timeline composition & keyframe evaluator
    ├── pipeline.rs     # GPU/parallel compositor (qtblend, dropshadow, blur)
    ├── audio.rs        # Concurrent audio rendering pass and muxer
    ├── fallback.rs     # Upstream libmlt-7.so dynamic loader (dlopen proxy)
    └── abi.rs          # C-ABI export symbols for libmlt-7.so compatibility
```

---

## Building and Usage

### 1. Build
```bash
cargo build --release
```
Outputs:
* `target/release/melt` (The drop-in executable)
* `target/release/libmlt_7.so` (The drop-in shared library)

### 2. Configure in Kdenlive Flatpak

You can switch Kdenlive Flatpak between your project's binaries and the stock MLT using the provided switcher script:

```bash
# Check current configuration status
./switch.sh status

# Switch Flatpak to use FastMelt (melt and libmlt.so)
./switch.sh project

# Switch Flatpak back to original stock MLT binaries
./switch.sh original
```

Or simply run `./switch.sh` for an interactive menu.

Alternatively, individual scripts are available in `scripts/`:
* `scripts/switch-to-project.sh`
* `scripts/switch-to-original.sh`
* `scripts/status.sh`

*(Note: Modifying Flatpak system files in `/var/lib/flatpak` will automatically prompt for `sudo`)*
