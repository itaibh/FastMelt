use mlt_7::parser::{KeyframeRect, Playlist, PlaylistEntry, PlaylistItem};

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
