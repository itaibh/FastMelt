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
        is_audio: false,
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

#[test]
fn test_blend_affine_aspect_preservation_and_rotation() {
    let mut fb = FrameBuffer::new(200, 200);
    fb.clear([0, 0, 0, 255]); // Black canvas

    // Create a 100x50 source (2:1 aspect ratio)
    let mut img = RgbaImage::new(100, 50);
    for p in img.pixels_mut() {
        *p = Rgba([255, 255, 255, 255]);
    }

    // Blend into a 100x100 square rect with distort=false
    // Expected: fits to width 100, height scaled to 50, centered vertically at y = 50 + (100-50)/2 = 75
    fb.blend_affine(
        img.as_raw(),
        100,
        50,
        50.0,
        50.0,
        100.0,
        100.0,
        0.0,
        false,
        false, // distort = false
        1.0,
    );

    let raw = fb.raw_bytes();
    // Center of rectangle (100, 100) should be white
    let center_idx = (100 * 200 + 100) * 4;
    assert_eq!(raw[center_idx], 255);

    // Top border of rect (100, 55) should be BLACK because image is centered vertically between 75 and 125
    let top_idx = (55 * 200 + 100) * 4;
    assert_eq!(raw[top_idx], 0, "Aspect preservation should center image vertically leaving top empty");
}

#[test]
fn test_render_bounds_resolution() {
    let mut proj = mlt_7::parser::MltProject::default();
    proj.consumer = Some(mlt_7::parser::Consumer {
        target: "test.mp4".to_string(),
        vcodec: "".to_string(),
        acodec: "".to_string(),
        in_frame: Some(100),
        out_frame: Some(250),
        extra_args: std::collections::HashMap::new(),
    });

    // 1. Without CLI overrides, should use consumer in/out
    let (s, e) = proj.get_render_bounds(None, None);
    assert_eq!((s, e), (100, 250));

    // 2. CLI in override
    let (s, e) = proj.get_render_bounds(Some(120), None);
    assert_eq!((s, e), (120, 250));

    // 3. CLI both overrides
    let (s, e) = proj.get_render_bounds(Some(50), Some(300));
    assert_eq!((s, e), (50, 300));
}

#[test]
fn test_affine_matrix_composition() {
    // Producer matrix M1: raw 100x100 -> placed at (10, 10) with size (50, 50)
    let m1 = FrameBuffer::build_affine_matrix(100.0, 100.0, 10.0, 10.0, 50.0, 50.0, 0.0, false, true);

    // Entry matrix M2: canvas 200x200 -> shifted by (20, 30)
    let m2 = FrameBuffer::build_affine_matrix(200.0, 200.0, 20.0, 30.0, 200.0, 200.0, 0.0, false, true);

    // Composite M = M2 * M1
    let m = m2 * m1;

    // Point (0, 0) in src:
    // Through M1: (10, 10)
    // Through M2: (20 + 10, 30 + 10) = (30, 40)
    let p0 = m.transform_point2(glam::vec2(0.0, 0.0));
    assert!((p0.x - 30.0).abs() < 0.01);
    assert!((p0.y - 40.0).abs() < 0.01);
}

#[test]
fn test_nested_mlt_producer_parsing() {
    let xml = r#"
    <mlt>
      <profile width="1080" height="1920" frame_rate_num="25" frame_rate_den="1" />
      <producer id="prod_title">
        <property name="resource">/path/to/nested_title.mlt</property>
        <property name="mlt_service">xml</property>
      </producer>
      <tractor id="root">
        <filter id="sub1">
          <property name="mlt_service">avfilter.subtitles</property>
          <property name="av.filename">/path/to/subs.ass</property>
        </filter>
        <track producer="prod_title" />
      </tractor>
    </mlt>
    "#;

    let proj = mlt_7::parser::MltProject::parse_xml(xml).expect("XML should parse cleanly");
    let prod = proj.producers.get("prod_title").expect("Producer should exist");
    match &prod.source {
        mlt_7::parser::ProducerSource::MltXml(path) => {
            assert_eq!(path.to_str().unwrap(), "/path/to/nested_title.mlt");
        }
        _ => panic!("Expected ProducerSource::MltXml, got {:?}", prod.source),
    }

    let sub_file = proj.get_subtitle_file(None);
    assert_eq!(sub_file.unwrap().to_str().unwrap(), "/path/to/subs.ass");
}

#[test]
fn test_audio_xml_generation() {
    let xml = r#"
    <mlt>
      <profile width="1080" height="1920" frame_rate_num="25" frame_rate_den="1" />
      <playlist id="playlist_audio">
        <property name="kdenlive:audio_track">1</property>
        <entry producer="audio1" in="0" out="100" />
      </playlist>
      <playlist id="playlist_video">
        <entry producer="video1" in="0" out="100" />
      </playlist>
      <tractor id="root">
        <track producer="playlist_audio" />
        <track producer="playlist_video" />
        <transition id="trans_vid">
          <property name="mlt_service">qtblend</property>
        </transition>
        <filter id="f_vid">
          <property name="mlt_service">qtblend</property>
        </filter>
      </tractor>
    </mlt>
    "#;

    let proj = mlt_7::parser::MltProject::parse_xml(xml).expect("XML should parse cleanly");
    assert!(mlt_7::audio::has_audio_content(&proj));

    let audio_xml = mlt_7::audio::generate_audio_only_xml(xml).expect("Should generate audio XML");
    assert!(audio_xml.contains("width=\"16\""));
    assert!(audio_xml.contains("height=\"16\""));
    assert!(audio_xml.contains("track producer=\"playlist_audio\""));
    assert!(!audio_xml.contains("track producer=\"playlist_video\""));
    assert!(!audio_xml.contains("qtblend"));
}

#[test]
fn test_wave_filter_distortion() {
    let mut fb = FrameBuffer::new(100, 100);
    // Draw horizontal line at y=50
    for x in 0..100 {
        let idx = (50 * 100 + x) * 4;
        fb.data[idx] = 255;
        fb.data[idx + 1] = 255;
        fb.data[idx + 2] = 255;
        fb.data[idx + 3] = 255;
    }

    // Apply vertical wave distortion
    fb.apply_wave(0, 10.0, 5.0, false, true);

    // The line at y=50 should now be distorted (not strictly at y=50 across all x)
    let mut found_non_50 = false;
    for y in 0..100 {
        if y != 50 && fb.data[(y * 100 + 25) * 4] == 255 {
            found_non_50 = true;
            break;
        }
    }
    assert!(found_non_50, "Wave filter should have displaced horizontal line");
}

#[test]
fn test_luma_wipe_transition() {
    let mut fb = FrameBuffer::new(10, 10);
    fb.clear([255, 255, 255, 255]);

    // Create a 10x10 gradient luma map from 0 to 255
    let mut luma = vec![0u8; 100];
    for y in 0..10 {
        for x in 0..10 {
            luma[y * 10 + x] = ((x as f32 / 9.0) * 255.0) as u8;
        }
    }

    // At progress 0.0 with low softness, pixels should be mostly 0 alpha
    let mut fb_0 = fb.clone();
    fb_0.apply_luma_wipe(&luma, 10, 10, 0.0, 0.05, false);
    assert_eq!(fb_0.data[(5 * 10 + 9) * 4 + 3], 0);

    // At progress 1.0, pixels should be full alpha
    let mut fb_1 = fb.clone();
    fb_1.apply_luma_wipe(&luma, 10, 10, 1.0, 0.05, false);
    assert_eq!(fb_1.data[(5 * 10 + 0) * 4 + 3], 255);
}

#[test]
fn test_mlt_root_dir_parsing() {
    let xml = r#"<?xml version="1.0" encoding="utf-8"?>
<mlt root="/home/user/my_project" version="7.43.0">
  <profile width="1920" height="1080"/>
</mlt>"#;
    let proj = mlt_7::parser::MltProject::parse(xml).expect("Parse failed");
    assert_eq!(
        proj.root_dir,
        Some(std::path::PathBuf::from("/home/user/my_project"))
    );
}

#[test]
fn test_native_size_producer_transform_handling() {
    // Verify that when a producer has a native-size filter (e.g. 0 0 452 598 for a 452x598 image),
    // and entry specifies placement (417 480 219 374),
    // the matrix places the corners at exactly (417, 480) and (417+219, 480+374) without compounding.
    let src_w = 452.0f32;
    let src_h = 598.0f32;
    let rect_x = 417.0f32;
    let rect_y = 480.0f32;
    let rect_w = 219.0f32;
    let rect_h = 374.0f32;

    let m_place = FrameBuffer::build_affine_matrix_with_dar(
        src_w,
        src_h,
        rect_x,
        rect_y,
        rect_w,
        rect_h,
        0.0,
        false,
        true, // distort to exact rect
        None,
    );

    let top_left = m_place.transform_point2(glam::vec2(0.0, 0.0));
    let bottom_right = m_place.transform_point2(glam::vec2(src_w, src_h));

    assert!((top_left.x - rect_x).abs() < 0.01);
    assert!((top_left.y - rect_y).abs() < 0.01);
    assert!((bottom_right.x - (rect_x + rect_w)).abs() < 0.01);
    assert!((bottom_right.y - (rect_y + rect_h)).abs() < 0.01);
}

#[test]
fn test_audio_only_export_detection() {
    use mlt_7::parser::Consumer;

    // Test audio target extension checks
    assert!(Consumer::is_audio_target("output.wav"));
    assert!(Consumer::is_audio_target("/path/to/render.MP3"));
    assert!(Consumer::is_audio_target("audio.flac"));
    assert!(Consumer::is_audio_target("track.m4a"));
    assert!(Consumer::is_audio_target("stream.opus"));
    assert!(!Consumer::is_audio_target("video.mp4"));
    assert!(!Consumer::is_audio_target("clip.mkv"));
    assert!(!Consumer::is_audio_target("animation.webm"));

    // Test consumer is_audio_only properties
    let audio_consumer = Consumer {
        target: "/renders/final.wav".to_string(),
        vcodec: "".to_string(),
        acodec: "pcm_s16le".to_string(),
        in_frame: None,
        out_frame: None,
        extra_args: std::collections::HashMap::new(),
    };
    assert!(audio_consumer.is_audio_only());

    // Video extension but vn=1 or vcodec=none
    let mut vn_consumer = Consumer {
        target: "/renders/final.mp4".to_string(),
        vcodec: "none".to_string(),
        acodec: "aac".to_string(),
        in_frame: None,
        out_frame: None,
        extra_args: std::collections::HashMap::new(),
    };
    assert!(vn_consumer.is_audio_only());

    vn_consumer.vcodec = "h264".to_string();
    assert!(!vn_consumer.is_audio_only());

    vn_consumer.extra_args.insert("vn".to_string(), "1".to_string());
    assert!(vn_consumer.is_audio_only());
}
