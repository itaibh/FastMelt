use mlt_7::parser::{KeyframeRect, Playlist, PlaylistEntry, PlaylistItem};
use mlt_7::pipeline::FrameBuffer;
use image::{Rgba, RgbaImage};

#[test]
fn test_keyframe_interpolation() {
    let keyframes = vec![
        KeyframeRect { frame: 0, x: 0.0, y: 0.0, w: 100.0, h: 200.0, opacity: 0.0 },
        KeyframeRect { frame: 10, x: 100.0, y: 200.0, w: 200.0, h: 400.0, opacity: 1.0 },
        KeyframeRect { frame: 20, x: 100.0, y: 200.0, w: 200.0, h: 400.0, opacity: 1.0 },
    ];

    // Frame before start
    let (x, y, w, h, opacity) = KeyframeRect::interpolate(&keyframes, 0);
    assert_eq!((x, y, w, h, opacity), (0.0, 0.0, 100.0, 200.0, 0.0));

    // Midpoint at frame 5
    let (x, y, w, h, opacity) = KeyframeRect::interpolate(&keyframes, 5);
    assert_eq!(x, 50.0);
    assert_eq!(y, 100.0);
    assert_eq!(w, 150.0);
    assert_eq!(h, 300.0);
    assert_eq!(opacity, 0.5);

    // End boundary
    let (x, y, w, h, opacity) = KeyframeRect::interpolate(&keyframes, 10);
    assert_eq!((x, y, w, h, opacity), (100.0, 200.0, 200.0, 400.0, 1.0));

    // Beyond last frame
    let (x, y, w, h, opacity) = KeyframeRect::interpolate(&keyframes, 25);
    assert_eq!((x, y, w, h, opacity), (100.0, 200.0, 200.0, 400.0, 1.0));
}

#[test]
fn test_playlist_active_clip_resolution() {
    let playlist = Playlist {
        id: "test_pl".to_string(),
        items: vec![
            PlaylistItem::Blank(10), // Frames 0..10
            PlaylistItem::Entry(PlaylistEntry {
                producer_id: "clip1".to_string(),
                in_frame: 0,
                out_frame: 19, // 20 frames long (10..30)
                filters: vec![],
            }),
            PlaylistItem::Blank(5), // Frames 30..35
            PlaylistItem::Entry(PlaylistEntry {
                producer_id: "clip2".to_string(),
                in_frame: 50,
                out_frame: 59, // 10 frames long (35..45)
                filters: vec![],
            }),
        ],
    };

    // Frame 5: Inside blank
    assert!(playlist.get_active_clip_at(5).is_none());

    // Frame 10: First frame of clip1
    let (entry, clip_frame) = playlist.get_active_clip_at(10).unwrap();
    assert_eq!(entry.producer_id, "clip1");
    assert_eq!(clip_frame, 0);

    // Frame 15: 5 frames into clip1
    let (entry, clip_frame) = playlist.get_active_clip_at(15).unwrap();
    assert_eq!(entry.producer_id, "clip1");
    assert_eq!(clip_frame, 5);

    // Frame 29: Last frame of clip1
    let (entry, clip_frame) = playlist.get_active_clip_at(29).unwrap();
    assert_eq!(entry.producer_id, "clip1");
    assert_eq!(clip_frame, 19);

    // Frame 32: Inside second blank
    assert!(playlist.get_active_clip_at(32).is_none());

    // Frame 35: First frame of clip2
    let (entry, clip_frame) = playlist.get_active_clip_at(35).unwrap();
    assert_eq!(entry.producer_id, "clip2");
    assert_eq!(clip_frame, 50);

    // Frame 50: Past end of playlist
    assert!(playlist.get_active_clip_at(50).is_none());
}

#[test]
fn test_blitter_and_alpha_composite() {
    let mut fb = FrameBuffer::new(100, 100);
    fb.clear([0, 0, 0, 255]); // Black background

    // Create a 20x20 red square with 100% opacity
    let mut red_square = RgbaImage::new(20, 20);
    for p in red_square.pixels_mut() {
        *p = Rgba([255, 0, 0, 255]);
    }

    // Blend onto fb at (10, 10)
    fb.blend_layer(&red_square, 10, 10, 20, 20, 1.0);

    let raw = fb.raw_bytes();
    // Pixel inside square (15, 15) should be red
    let idx = (15 * 100 + 15) * 4;
    assert_eq!(raw[idx], 255);
    assert_eq!(raw[idx + 1], 0);
    assert_eq!(raw[idx + 2], 0);
    assert_eq!(raw[idx + 3], 255);

    // Pixel outside square (5, 5) should remain black
    let out_idx = (5 * 100 + 5) * 4;
    assert_eq!(raw[out_idx], 0);
    assert_eq!(raw[out_idx + 1], 0);
    assert_eq!(raw[out_idx + 2], 0);
    assert_eq!(raw[out_idx + 3], 255);
}

#[test]
fn test_separable_blur_energy_conservation() {
    let mut fb = FrameBuffer::new(50, 50);
    fb.clear([0, 0, 0, 0]);

    // Draw single point of high energy at center
    let center_idx = (25 * 50 + 25) * 4;
    fb.data[center_idx] = 200;
    fb.data[center_idx + 3] = 255;

    fb.apply_separable_blur(3.0);

    // After blur, center should have spread to neighboring pixels
    let raw = fb.raw_bytes();
    let neighbor_idx = (25 * 50 + 26) * 4;
    assert!(raw[neighbor_idx] > 0, "Neighbor pixel should receive energy from blur");
}

#[test]
fn test_drop_shadow_placement() {
    let mut fb = FrameBuffer::new(200, 200);
    fb.clear([255, 255, 255, 255]); // White canvas

    let mut dot = RgbaImage::new(10, 10);
    for p in dot.pixels_mut() {
        *p = Rgba([0, 0, 0, 255]);
    }

    // Render drop shadow at x=50, y=50 with offset 10, 10 -> shadow at 60, 60
    fb.render_drop_shadow(&dot, 60, 60, 10, 10, 2.0, 0x000000FF, 1.0);

    let raw = fb.raw_bytes();
    // Pixel around (65, 65) should have darkened (shadow presence)
    let shadow_idx = (65 * 200 + 65) * 4;
    assert!(raw[shadow_idx] < 255, "Shadow should darken background around (65, 65)");
}
