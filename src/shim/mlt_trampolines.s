/* Auto-generated x86_64 assembly trampolines for FastMelt */
.text

.global default_callback
.type default_callback, @function
default_callback:
    jmp *real_default_callback(%rip)
.size default_callback, . - default_callback

.global mlt_animation_close
.type mlt_animation_close, @function
mlt_animation_close:
    jmp *real_mlt_animation_close(%rip)
.size mlt_animation_close, . - mlt_animation_close

.global mlt_animation_get_item
.type mlt_animation_get_item, @function
mlt_animation_get_item:
    jmp *real_mlt_animation_get_item(%rip)
.size mlt_animation_get_item, . - mlt_animation_get_item

.global mlt_animation_get_length
.type mlt_animation_get_length, @function
mlt_animation_get_length:
    jmp *real_mlt_animation_get_length(%rip)
.size mlt_animation_get_length, . - mlt_animation_get_length

.global mlt_animation_get_string
.type mlt_animation_get_string, @function
mlt_animation_get_string:
    jmp *real_mlt_animation_get_string(%rip)
.size mlt_animation_get_string, . - mlt_animation_get_string

.global mlt_animation_insert
.type mlt_animation_insert, @function
mlt_animation_insert:
    jmp *real_mlt_animation_insert(%rip)
.size mlt_animation_insert, . - mlt_animation_insert

.global mlt_animation_interpolate
.type mlt_animation_interpolate, @function
mlt_animation_interpolate:
    jmp *real_mlt_animation_interpolate(%rip)
.size mlt_animation_interpolate, . - mlt_animation_interpolate

.global mlt_animation_key_count
.type mlt_animation_key_count, @function
mlt_animation_key_count:
    jmp *real_mlt_animation_key_count(%rip)
.size mlt_animation_key_count, . - mlt_animation_key_count

.global mlt_animation_key_get
.type mlt_animation_key_get, @function
mlt_animation_key_get:
    jmp *real_mlt_animation_key_get(%rip)
.size mlt_animation_key_get, . - mlt_animation_key_get

.global mlt_animation_key_set_frame
.type mlt_animation_key_set_frame, @function
mlt_animation_key_set_frame:
    jmp *real_mlt_animation_key_set_frame(%rip)
.size mlt_animation_key_set_frame, . - mlt_animation_key_set_frame

.global mlt_animation_key_set_type
.type mlt_animation_key_set_type, @function
mlt_animation_key_set_type:
    jmp *real_mlt_animation_key_set_type(%rip)
.size mlt_animation_key_set_type, . - mlt_animation_key_set_type

.global mlt_animation_new
.type mlt_animation_new, @function
mlt_animation_new:
    jmp *real_mlt_animation_new(%rip)
.size mlt_animation_new, . - mlt_animation_new

.global mlt_animation_next_key
.type mlt_animation_next_key, @function
mlt_animation_next_key:
    jmp *real_mlt_animation_next_key(%rip)
.size mlt_animation_next_key, . - mlt_animation_next_key

.global mlt_animation_parse
.type mlt_animation_parse, @function
mlt_animation_parse:
    jmp *real_mlt_animation_parse(%rip)
.size mlt_animation_parse, . - mlt_animation_parse

.global mlt_animation_parse_item
.type mlt_animation_parse_item, @function
mlt_animation_parse_item:
    jmp *real_mlt_animation_parse_item(%rip)
.size mlt_animation_parse_item, . - mlt_animation_parse_item

.global mlt_animation_prev_key
.type mlt_animation_prev_key, @function
mlt_animation_prev_key:
    jmp *real_mlt_animation_prev_key(%rip)
.size mlt_animation_prev_key, . - mlt_animation_prev_key

.global mlt_animation_refresh
.type mlt_animation_refresh, @function
mlt_animation_refresh:
    jmp *real_mlt_animation_refresh(%rip)
.size mlt_animation_refresh, . - mlt_animation_refresh

.global mlt_animation_remove
.type mlt_animation_remove, @function
mlt_animation_remove:
    jmp *real_mlt_animation_remove(%rip)
.size mlt_animation_remove, . - mlt_animation_remove

.global mlt_animation_serialize
.type mlt_animation_serialize, @function
mlt_animation_serialize:
    jmp *real_mlt_animation_serialize(%rip)
.size mlt_animation_serialize, . - mlt_animation_serialize

.global mlt_animation_serialize_cut
.type mlt_animation_serialize_cut, @function
mlt_animation_serialize_cut:
    jmp *real_mlt_animation_serialize_cut(%rip)
.size mlt_animation_serialize_cut, . - mlt_animation_serialize_cut

.global mlt_animation_serialize_cut_tf
.type mlt_animation_serialize_cut_tf, @function
mlt_animation_serialize_cut_tf:
    jmp *real_mlt_animation_serialize_cut_tf(%rip)
.size mlt_animation_serialize_cut_tf, . - mlt_animation_serialize_cut_tf

.global mlt_animation_serialize_tf
.type mlt_animation_serialize_tf, @function
mlt_animation_serialize_tf:
    jmp *real_mlt_animation_serialize_tf(%rip)
.size mlt_animation_serialize_tf, . - mlt_animation_serialize_tf

.global mlt_animation_set_length
.type mlt_animation_set_length, @function
mlt_animation_set_length:
    jmp *real_mlt_animation_set_length(%rip)
.size mlt_animation_set_length, . - mlt_animation_set_length

.global mlt_animation_shift_frames
.type mlt_animation_shift_frames, @function
mlt_animation_shift_frames:
    jmp *real_mlt_animation_shift_frames(%rip)
.size mlt_animation_shift_frames, . - mlt_animation_shift_frames

.global mlt_audio_alloc_data
.type mlt_audio_alloc_data, @function
mlt_audio_alloc_data:
    jmp *real_mlt_audio_alloc_data(%rip)
.size mlt_audio_alloc_data, . - mlt_audio_alloc_data

.global mlt_audio_calculate_frame_samples
.type mlt_audio_calculate_frame_samples, @function
mlt_audio_calculate_frame_samples:
    jmp *real_mlt_audio_calculate_frame_samples(%rip)
.size mlt_audio_calculate_frame_samples, . - mlt_audio_calculate_frame_samples

.global mlt_audio_calculate_samples_to_position
.type mlt_audio_calculate_samples_to_position, @function
mlt_audio_calculate_samples_to_position:
    jmp *real_mlt_audio_calculate_samples_to_position(%rip)
.size mlt_audio_calculate_samples_to_position, . - mlt_audio_calculate_samples_to_position

.global mlt_audio_calculate_size
.type mlt_audio_calculate_size, @function
mlt_audio_calculate_size:
    jmp *real_mlt_audio_calculate_size(%rip)
.size mlt_audio_calculate_size, . - mlt_audio_calculate_size

.global mlt_audio_channel_layout_channels
.type mlt_audio_channel_layout_channels, @function
mlt_audio_channel_layout_channels:
    jmp *real_mlt_audio_channel_layout_channels(%rip)
.size mlt_audio_channel_layout_channels, . - mlt_audio_channel_layout_channels

.global mlt_audio_channel_layout_default
.type mlt_audio_channel_layout_default, @function
mlt_audio_channel_layout_default:
    jmp *real_mlt_audio_channel_layout_default(%rip)
.size mlt_audio_channel_layout_default, . - mlt_audio_channel_layout_default

.global mlt_audio_channel_layout_id
.type mlt_audio_channel_layout_id, @function
mlt_audio_channel_layout_id:
    jmp *real_mlt_audio_channel_layout_id(%rip)
.size mlt_audio_channel_layout_id, . - mlt_audio_channel_layout_id

.global mlt_audio_channel_layout_name
.type mlt_audio_channel_layout_name, @function
mlt_audio_channel_layout_name:
    jmp *real_mlt_audio_channel_layout_name(%rip)
.size mlt_audio_channel_layout_name, . - mlt_audio_channel_layout_name

.global mlt_audio_close
.type mlt_audio_close, @function
mlt_audio_close:
    jmp *real_mlt_audio_close(%rip)
.size mlt_audio_close, . - mlt_audio_close

.global mlt_audio_copy
.type mlt_audio_copy, @function
mlt_audio_copy:
    jmp *real_mlt_audio_copy(%rip)
.size mlt_audio_copy, . - mlt_audio_copy

.global mlt_audio_format_id
.type mlt_audio_format_id, @function
mlt_audio_format_id:
    jmp *real_mlt_audio_format_id(%rip)
.size mlt_audio_format_id, . - mlt_audio_format_id

.global mlt_audio_format_name
.type mlt_audio_format_name, @function
mlt_audio_format_name:
    jmp *real_mlt_audio_format_name(%rip)
.size mlt_audio_format_name, . - mlt_audio_format_name

.global mlt_audio_format_size
.type mlt_audio_format_size, @function
mlt_audio_format_size:
    jmp *real_mlt_audio_format_size(%rip)
.size mlt_audio_format_size, . - mlt_audio_format_size

.global mlt_audio_free_data
.type mlt_audio_free_data, @function
mlt_audio_free_data:
    jmp *real_mlt_audio_free_data(%rip)
.size mlt_audio_free_data, . - mlt_audio_free_data

.global mlt_audio_get_planes
.type mlt_audio_get_planes, @function
mlt_audio_get_planes:
    jmp *real_mlt_audio_get_planes(%rip)
.size mlt_audio_get_planes, . - mlt_audio_get_planes

.global mlt_audio_get_values
.type mlt_audio_get_values, @function
mlt_audio_get_values:
    jmp *real_mlt_audio_get_values(%rip)
.size mlt_audio_get_values, . - mlt_audio_get_values

.global mlt_audio_new
.type mlt_audio_new, @function
mlt_audio_new:
    jmp *real_mlt_audio_new(%rip)
.size mlt_audio_new, . - mlt_audio_new

.global mlt_audio_plane_count
.type mlt_audio_plane_count, @function
mlt_audio_plane_count:
    jmp *real_mlt_audio_plane_count(%rip)
.size mlt_audio_plane_count, . - mlt_audio_plane_count

.global mlt_audio_plane_size
.type mlt_audio_plane_size, @function
mlt_audio_plane_size:
    jmp *real_mlt_audio_plane_size(%rip)
.size mlt_audio_plane_size, . - mlt_audio_plane_size

.global mlt_audio_reverse
.type mlt_audio_reverse, @function
mlt_audio_reverse:
    jmp *real_mlt_audio_reverse(%rip)
.size mlt_audio_reverse, . - mlt_audio_reverse

.global mlt_audio_set_values
.type mlt_audio_set_values, @function
mlt_audio_set_values:
    jmp *real_mlt_audio_set_values(%rip)
.size mlt_audio_set_values, . - mlt_audio_set_values

.global mlt_audio_shrink
.type mlt_audio_shrink, @function
mlt_audio_shrink:
    jmp *real_mlt_audio_shrink(%rip)
.size mlt_audio_shrink, . - mlt_audio_shrink

.global mlt_audio_silence
.type mlt_audio_silence, @function
mlt_audio_silence:
    jmp *real_mlt_audio_silence(%rip)
.size mlt_audio_silence, . - mlt_audio_silence

.global mlt_cache_close
.type mlt_cache_close, @function
mlt_cache_close:
    jmp *real_mlt_cache_close(%rip)
.size mlt_cache_close, . - mlt_cache_close

.global mlt_cache_get
.type mlt_cache_get, @function
mlt_cache_get:
    jmp *real_mlt_cache_get(%rip)
.size mlt_cache_get, . - mlt_cache_get

.global mlt_cache_get_frame
.type mlt_cache_get_frame, @function
mlt_cache_get_frame:
    jmp *real_mlt_cache_get_frame(%rip)
.size mlt_cache_get_frame, . - mlt_cache_get_frame

.global mlt_cache_get_size
.type mlt_cache_get_size, @function
mlt_cache_get_size:
    jmp *real_mlt_cache_get_size(%rip)
.size mlt_cache_get_size, . - mlt_cache_get_size

.global mlt_cache_init
.type mlt_cache_init, @function
mlt_cache_init:
    jmp *real_mlt_cache_init(%rip)
.size mlt_cache_init, . - mlt_cache_init

.global mlt_cache_item_close
.type mlt_cache_item_close, @function
mlt_cache_item_close:
    jmp *real_mlt_cache_item_close(%rip)
.size mlt_cache_item_close, . - mlt_cache_item_close

.global mlt_cache_item_data
.type mlt_cache_item_data, @function
mlt_cache_item_data:
    jmp *real_mlt_cache_item_data(%rip)
.size mlt_cache_item_data, . - mlt_cache_item_data

.global mlt_cache_purge
.type mlt_cache_purge, @function
mlt_cache_purge:
    jmp *real_mlt_cache_purge(%rip)
.size mlt_cache_purge, . - mlt_cache_purge

.global mlt_cache_put
.type mlt_cache_put, @function
mlt_cache_put:
    jmp *real_mlt_cache_put(%rip)
.size mlt_cache_put, . - mlt_cache_put

.global mlt_cache_put_frame
.type mlt_cache_put_frame, @function
mlt_cache_put_frame:
    jmp *real_mlt_cache_put_frame(%rip)
.size mlt_cache_put_frame, . - mlt_cache_put_frame

.global mlt_cache_put_frame_audio
.type mlt_cache_put_frame_audio, @function
mlt_cache_put_frame_audio:
    jmp *real_mlt_cache_put_frame_audio(%rip)
.size mlt_cache_put_frame_audio, . - mlt_cache_put_frame_audio

.global mlt_cache_put_frame_image
.type mlt_cache_put_frame_image, @function
mlt_cache_put_frame_image:
    jmp *real_mlt_cache_put_frame_image(%rip)
.size mlt_cache_put_frame_image, . - mlt_cache_put_frame_image

.global mlt_cache_set_size
.type mlt_cache_set_size, @function
mlt_cache_set_size:
    jmp *real_mlt_cache_set_size(%rip)
.size mlt_cache_set_size, . - mlt_cache_set_size

.global mlt_chain_attach
.type mlt_chain_attach, @function
mlt_chain_attach:
    jmp *real_mlt_chain_attach(%rip)
.size mlt_chain_attach, . - mlt_chain_attach

.global mlt_chain_attach_normalizers
.type mlt_chain_attach_normalizers, @function
mlt_chain_attach_normalizers:
    jmp *real_mlt_chain_attach_normalizers(%rip)
.size mlt_chain_attach_normalizers, . - mlt_chain_attach_normalizers

.global mlt_chain_close
.type mlt_chain_close, @function
mlt_chain_close:
    jmp *real_mlt_chain_close(%rip)
.size mlt_chain_close, . - mlt_chain_close

.global mlt_chain_detach
.type mlt_chain_detach, @function
mlt_chain_detach:
    jmp *real_mlt_chain_detach(%rip)
.size mlt_chain_detach, . - mlt_chain_detach

.global mlt_chain_get_source
.type mlt_chain_get_source, @function
mlt_chain_get_source:
    jmp *real_mlt_chain_get_source(%rip)
.size mlt_chain_get_source, . - mlt_chain_get_source

.global mlt_chain_init
.type mlt_chain_init, @function
mlt_chain_init:
    jmp *real_mlt_chain_init(%rip)
.size mlt_chain_init, . - mlt_chain_init

.global mlt_chain_link
.type mlt_chain_link, @function
mlt_chain_link:
    jmp *real_mlt_chain_link(%rip)
.size mlt_chain_link, . - mlt_chain_link

.global mlt_chain_link_count
.type mlt_chain_link_count, @function
mlt_chain_link_count:
    jmp *real_mlt_chain_link_count(%rip)
.size mlt_chain_link_count, . - mlt_chain_link_count

.global mlt_chain_move_link
.type mlt_chain_move_link, @function
mlt_chain_move_link:
    jmp *real_mlt_chain_move_link(%rip)
.size mlt_chain_move_link, . - mlt_chain_move_link

.global mlt_chain_set_source
.type mlt_chain_set_source, @function
mlt_chain_set_source:
    jmp *real_mlt_chain_set_source(%rip)
.size mlt_chain_set_source, . - mlt_chain_set_source

.global mlt_color_convert_trc
.type mlt_color_convert_trc, @function
mlt_color_convert_trc:
    jmp *real_mlt_color_convert_trc(%rip)
.size mlt_color_convert_trc, . - mlt_color_convert_trc

.global mlt_consumer_close
.type mlt_consumer_close, @function
mlt_consumer_close:
    jmp *real_mlt_consumer_close(%rip)
.size mlt_consumer_close, . - mlt_consumer_close

.global mlt_consumer_connect
.type mlt_consumer_connect, @function
mlt_consumer_connect:
    jmp *real_mlt_consumer_connect(%rip)
.size mlt_consumer_connect, . - mlt_consumer_connect

.global mlt_consumer_get_frame
.type mlt_consumer_get_frame, @function
mlt_consumer_get_frame:
    jmp *real_mlt_consumer_get_frame(%rip)
.size mlt_consumer_get_frame, . - mlt_consumer_get_frame

.global mlt_consumer_init
.type mlt_consumer_init, @function
mlt_consumer_init:
    jmp *real_mlt_consumer_init(%rip)
.size mlt_consumer_init, . - mlt_consumer_init

.global mlt_consumer_is_stopped
.type mlt_consumer_is_stopped, @function
mlt_consumer_is_stopped:
    jmp *real_mlt_consumer_is_stopped(%rip)
.size mlt_consumer_is_stopped, . - mlt_consumer_is_stopped

.global mlt_consumer_new
.type mlt_consumer_new, @function
mlt_consumer_new:
    jmp *real_mlt_consumer_new(%rip)
.size mlt_consumer_new, . - mlt_consumer_new

.global mlt_consumer_position
.type mlt_consumer_position, @function
mlt_consumer_position:
    jmp *real_mlt_consumer_position(%rip)
.size mlt_consumer_position, . - mlt_consumer_position

.global mlt_consumer_properties
.type mlt_consumer_properties, @function
mlt_consumer_properties:
    jmp *real_mlt_consumer_properties(%rip)
.size mlt_consumer_properties, . - mlt_consumer_properties

.global mlt_consumer_purge
.type mlt_consumer_purge, @function
mlt_consumer_purge:
    jmp *real_mlt_consumer_purge(%rip)
.size mlt_consumer_purge, . - mlt_consumer_purge

.global mlt_consumer_put_frame
.type mlt_consumer_put_frame, @function
mlt_consumer_put_frame:
    jmp *real_mlt_consumer_put_frame(%rip)
.size mlt_consumer_put_frame, . - mlt_consumer_put_frame

.global mlt_consumer_rt_frame
.type mlt_consumer_rt_frame, @function
mlt_consumer_rt_frame:
    jmp *real_mlt_consumer_rt_frame(%rip)
.size mlt_consumer_rt_frame, . - mlt_consumer_rt_frame

.global mlt_consumer_service
.type mlt_consumer_service, @function
mlt_consumer_service:
    jmp *real_mlt_consumer_service(%rip)
.size mlt_consumer_service, . - mlt_consumer_service

.global mlt_consumer_start
.type mlt_consumer_start, @function
mlt_consumer_start:
    jmp rust_mlt_consumer_start
.size mlt_consumer_start, . - mlt_consumer_start

.global mlt_consumer_stop
.type mlt_consumer_stop, @function
mlt_consumer_stop:
    jmp *real_mlt_consumer_stop(%rip)
.size mlt_consumer_stop, . - mlt_consumer_stop

.global mlt_consumer_stopped
.type mlt_consumer_stopped, @function
mlt_consumer_stopped:
    jmp *real_mlt_consumer_stopped(%rip)
.size mlt_consumer_stopped, . - mlt_consumer_stopped

.global mlt_deinterlacer_id
.type mlt_deinterlacer_id, @function
mlt_deinterlacer_id:
    jmp *real_mlt_deinterlacer_id(%rip)
.size mlt_deinterlacer_id, . - mlt_deinterlacer_id

.global mlt_deinterlacer_name
.type mlt_deinterlacer_name, @function
mlt_deinterlacer_name:
    jmp *real_mlt_deinterlacer_name(%rip)
.size mlt_deinterlacer_name, . - mlt_deinterlacer_name

.global mlt_deque_close
.type mlt_deque_close, @function
mlt_deque_close:
    jmp *real_mlt_deque_close(%rip)
.size mlt_deque_close, . - mlt_deque_close

.global mlt_deque_count
.type mlt_deque_count, @function
mlt_deque_count:
    jmp *real_mlt_deque_count(%rip)
.size mlt_deque_count, . - mlt_deque_count

.global mlt_deque_init
.type mlt_deque_init, @function
mlt_deque_init:
    jmp *real_mlt_deque_init(%rip)
.size mlt_deque_init, . - mlt_deque_init

.global mlt_deque_insert
.type mlt_deque_insert, @function
mlt_deque_insert:
    jmp *real_mlt_deque_insert(%rip)
.size mlt_deque_insert, . - mlt_deque_insert

.global mlt_deque_peek
.type mlt_deque_peek, @function
mlt_deque_peek:
    jmp *real_mlt_deque_peek(%rip)
.size mlt_deque_peek, . - mlt_deque_peek

.global mlt_deque_peek_back
.type mlt_deque_peek_back, @function
mlt_deque_peek_back:
    jmp *real_mlt_deque_peek_back(%rip)
.size mlt_deque_peek_back, . - mlt_deque_peek_back

.global mlt_deque_peek_back_double
.type mlt_deque_peek_back_double, @function
mlt_deque_peek_back_double:
    jmp *real_mlt_deque_peek_back_double(%rip)
.size mlt_deque_peek_back_double, . - mlt_deque_peek_back_double

.global mlt_deque_peek_back_int
.type mlt_deque_peek_back_int, @function
mlt_deque_peek_back_int:
    jmp *real_mlt_deque_peek_back_int(%rip)
.size mlt_deque_peek_back_int, . - mlt_deque_peek_back_int

.global mlt_deque_peek_front
.type mlt_deque_peek_front, @function
mlt_deque_peek_front:
    jmp *real_mlt_deque_peek_front(%rip)
.size mlt_deque_peek_front, . - mlt_deque_peek_front

.global mlt_deque_peek_front_double
.type mlt_deque_peek_front_double, @function
mlt_deque_peek_front_double:
    jmp *real_mlt_deque_peek_front_double(%rip)
.size mlt_deque_peek_front_double, . - mlt_deque_peek_front_double

.global mlt_deque_peek_front_int
.type mlt_deque_peek_front_int, @function
mlt_deque_peek_front_int:
    jmp *real_mlt_deque_peek_front_int(%rip)
.size mlt_deque_peek_front_int, . - mlt_deque_peek_front_int

.global mlt_deque_pop_back
.type mlt_deque_pop_back, @function
mlt_deque_pop_back:
    jmp *real_mlt_deque_pop_back(%rip)
.size mlt_deque_pop_back, . - mlt_deque_pop_back

.global mlt_deque_pop_back_double
.type mlt_deque_pop_back_double, @function
mlt_deque_pop_back_double:
    jmp *real_mlt_deque_pop_back_double(%rip)
.size mlt_deque_pop_back_double, . - mlt_deque_pop_back_double

.global mlt_deque_pop_back_int
.type mlt_deque_pop_back_int, @function
mlt_deque_pop_back_int:
    jmp *real_mlt_deque_pop_back_int(%rip)
.size mlt_deque_pop_back_int, . - mlt_deque_pop_back_int

.global mlt_deque_pop_front
.type mlt_deque_pop_front, @function
mlt_deque_pop_front:
    jmp *real_mlt_deque_pop_front(%rip)
.size mlt_deque_pop_front, . - mlt_deque_pop_front

.global mlt_deque_pop_front_double
.type mlt_deque_pop_front_double, @function
mlt_deque_pop_front_double:
    jmp *real_mlt_deque_pop_front_double(%rip)
.size mlt_deque_pop_front_double, . - mlt_deque_pop_front_double

.global mlt_deque_pop_front_int
.type mlt_deque_pop_front_int, @function
mlt_deque_pop_front_int:
    jmp *real_mlt_deque_pop_front_int(%rip)
.size mlt_deque_pop_front_int, . - mlt_deque_pop_front_int

.global mlt_deque_push_back
.type mlt_deque_push_back, @function
mlt_deque_push_back:
    jmp *real_mlt_deque_push_back(%rip)
.size mlt_deque_push_back, . - mlt_deque_push_back

.global mlt_deque_push_back_double
.type mlt_deque_push_back_double, @function
mlt_deque_push_back_double:
    jmp *real_mlt_deque_push_back_double(%rip)
.size mlt_deque_push_back_double, . - mlt_deque_push_back_double

.global mlt_deque_push_back_int
.type mlt_deque_push_back_int, @function
mlt_deque_push_back_int:
    jmp *real_mlt_deque_push_back_int(%rip)
.size mlt_deque_push_back_int, . - mlt_deque_push_back_int

.global mlt_deque_push_front
.type mlt_deque_push_front, @function
mlt_deque_push_front:
    jmp *real_mlt_deque_push_front(%rip)
.size mlt_deque_push_front, . - mlt_deque_push_front

.global mlt_deque_push_front_double
.type mlt_deque_push_front_double, @function
mlt_deque_push_front_double:
    jmp *real_mlt_deque_push_front_double(%rip)
.size mlt_deque_push_front_double, . - mlt_deque_push_front_double

.global mlt_deque_push_front_int
.type mlt_deque_push_front_int, @function
mlt_deque_push_front_int:
    jmp *real_mlt_deque_push_front_int(%rip)
.size mlt_deque_push_front_int, . - mlt_deque_push_front_int

.global mlt_environment
.type mlt_environment, @function
mlt_environment:
    jmp *real_mlt_environment(%rip)
.size mlt_environment, . - mlt_environment

.global mlt_environment_set
.type mlt_environment_set, @function
mlt_environment_set:
    jmp *real_mlt_environment_set(%rip)
.size mlt_environment_set, . - mlt_environment_set

.global mlt_event_block
.type mlt_event_block, @function
mlt_event_block:
    jmp *real_mlt_event_block(%rip)
.size mlt_event_block, . - mlt_event_block

.global mlt_event_close
.type mlt_event_close, @function
mlt_event_close:
    jmp *real_mlt_event_close(%rip)
.size mlt_event_close, . - mlt_event_close

.global mlt_event_data_from_frame
.type mlt_event_data_from_frame, @function
mlt_event_data_from_frame:
    jmp *real_mlt_event_data_from_frame(%rip)
.size mlt_event_data_from_frame, . - mlt_event_data_from_frame

.global mlt_event_data_from_int
.type mlt_event_data_from_int, @function
mlt_event_data_from_int:
    jmp *real_mlt_event_data_from_int(%rip)
.size mlt_event_data_from_int, . - mlt_event_data_from_int

.global mlt_event_data_from_object
.type mlt_event_data_from_object, @function
mlt_event_data_from_object:
    jmp *real_mlt_event_data_from_object(%rip)
.size mlt_event_data_from_object, . - mlt_event_data_from_object

.global mlt_event_data_from_string
.type mlt_event_data_from_string, @function
mlt_event_data_from_string:
    jmp *real_mlt_event_data_from_string(%rip)
.size mlt_event_data_from_string, . - mlt_event_data_from_string

.global mlt_event_data_none
.type mlt_event_data_none, @function
mlt_event_data_none:
    jmp *real_mlt_event_data_none(%rip)
.size mlt_event_data_none, . - mlt_event_data_none

.global mlt_event_data_to_frame
.type mlt_event_data_to_frame, @function
mlt_event_data_to_frame:
    jmp *real_mlt_event_data_to_frame(%rip)
.size mlt_event_data_to_frame, . - mlt_event_data_to_frame

.global mlt_event_data_to_int
.type mlt_event_data_to_int, @function
mlt_event_data_to_int:
    jmp *real_mlt_event_data_to_int(%rip)
.size mlt_event_data_to_int, . - mlt_event_data_to_int

.global mlt_event_data_to_object
.type mlt_event_data_to_object, @function
mlt_event_data_to_object:
    jmp *real_mlt_event_data_to_object(%rip)
.size mlt_event_data_to_object, . - mlt_event_data_to_object

.global mlt_event_data_to_string
.type mlt_event_data_to_string, @function
mlt_event_data_to_string:
    jmp *real_mlt_event_data_to_string(%rip)
.size mlt_event_data_to_string, . - mlt_event_data_to_string

.global mlt_event_inc_ref
.type mlt_event_inc_ref, @function
mlt_event_inc_ref:
    jmp *real_mlt_event_inc_ref(%rip)
.size mlt_event_inc_ref, . - mlt_event_inc_ref

.global mlt_event_unblock
.type mlt_event_unblock, @function
mlt_event_unblock:
    jmp *real_mlt_event_unblock(%rip)
.size mlt_event_unblock, . - mlt_event_unblock

.global mlt_events_block
.type mlt_events_block, @function
mlt_events_block:
    jmp *real_mlt_events_block(%rip)
.size mlt_events_block, . - mlt_events_block

.global mlt_events_close_wait_for
.type mlt_events_close_wait_for, @function
mlt_events_close_wait_for:
    jmp *real_mlt_events_close_wait_for(%rip)
.size mlt_events_close_wait_for, . - mlt_events_close_wait_for

.global mlt_events_disconnect
.type mlt_events_disconnect, @function
mlt_events_disconnect:
    jmp *real_mlt_events_disconnect(%rip)
.size mlt_events_disconnect, . - mlt_events_disconnect

.global mlt_events_fire
.type mlt_events_fire, @function
mlt_events_fire:
    jmp *real_mlt_events_fire(%rip)
.size mlt_events_fire, . - mlt_events_fire

.global mlt_events_init
.type mlt_events_init, @function
mlt_events_init:
    jmp *real_mlt_events_init(%rip)
.size mlt_events_init, . - mlt_events_init

.global mlt_events_listen
.type mlt_events_listen, @function
mlt_events_listen:
    jmp *real_mlt_events_listen(%rip)
.size mlt_events_listen, . - mlt_events_listen

.global mlt_events_register
.type mlt_events_register, @function
mlt_events_register:
    jmp *real_mlt_events_register(%rip)
.size mlt_events_register, . - mlt_events_register

.global mlt_events_setup_wait_for
.type mlt_events_setup_wait_for, @function
mlt_events_setup_wait_for:
    jmp *real_mlt_events_setup_wait_for(%rip)
.size mlt_events_setup_wait_for, . - mlt_events_setup_wait_for

.global mlt_events_unblock
.type mlt_events_unblock, @function
mlt_events_unblock:
    jmp *real_mlt_events_unblock(%rip)
.size mlt_events_unblock, . - mlt_events_unblock

.global mlt_events_wait_for
.type mlt_events_wait_for, @function
mlt_events_wait_for:
    jmp *real_mlt_events_wait_for(%rip)
.size mlt_events_wait_for, . - mlt_events_wait_for

.global mlt_factory_close
.type mlt_factory_close, @function
mlt_factory_close:
    jmp rust_mlt_factory_close
.size mlt_factory_close, . - mlt_factory_close

.global mlt_factory_consumer
.type mlt_factory_consumer, @function
mlt_factory_consumer:
    jmp *real_mlt_factory_consumer(%rip)
.size mlt_factory_consumer, . - mlt_factory_consumer

.global mlt_factory_directory
.type mlt_factory_directory, @function
mlt_factory_directory:
    jmp *real_mlt_factory_directory(%rip)
.size mlt_factory_directory, . - mlt_factory_directory

.global mlt_factory_event_object
.type mlt_factory_event_object, @function
mlt_factory_event_object:
    jmp *real_mlt_factory_event_object(%rip)
.size mlt_factory_event_object, . - mlt_factory_event_object

.global mlt_factory_filter
.type mlt_factory_filter, @function
mlt_factory_filter:
    jmp *real_mlt_factory_filter(%rip)
.size mlt_factory_filter, . - mlt_factory_filter

.global mlt_factory_init
.type mlt_factory_init, @function
mlt_factory_init:
    jmp rust_mlt_factory_init
.size mlt_factory_init, . - mlt_factory_init

.global mlt_factory_link
.type mlt_factory_link, @function
mlt_factory_link:
    jmp *real_mlt_factory_link(%rip)
.size mlt_factory_link, . - mlt_factory_link

.global mlt_factory_producer
.type mlt_factory_producer, @function
mlt_factory_producer:
    jmp *real_mlt_factory_producer(%rip)
.size mlt_factory_producer, . - mlt_factory_producer

.global mlt_factory_register_for_clean_up
.type mlt_factory_register_for_clean_up, @function
mlt_factory_register_for_clean_up:
    jmp *real_mlt_factory_register_for_clean_up(%rip)
.size mlt_factory_register_for_clean_up, . - mlt_factory_register_for_clean_up

.global mlt_factory_repository
.type mlt_factory_repository, @function
mlt_factory_repository:
    jmp *real_mlt_factory_repository(%rip)
.size mlt_factory_repository, . - mlt_factory_repository

.global mlt_factory_transition
.type mlt_factory_transition, @function
mlt_factory_transition:
    jmp *real_mlt_factory_transition(%rip)
.size mlt_factory_transition, . - mlt_factory_transition

.global mlt_field_close
.type mlt_field_close, @function
mlt_field_close:
    jmp *real_mlt_field_close(%rip)
.size mlt_field_close, . - mlt_field_close

.global mlt_field_disconnect_service
.type mlt_field_disconnect_service, @function
mlt_field_disconnect_service:
    jmp *real_mlt_field_disconnect_service(%rip)
.size mlt_field_disconnect_service, . - mlt_field_disconnect_service

.global mlt_field_init
.type mlt_field_init, @function
mlt_field_init:
    jmp *real_mlt_field_init(%rip)
.size mlt_field_init, . - mlt_field_init

.global mlt_field_multitrack
.type mlt_field_multitrack, @function
mlt_field_multitrack:
    jmp *real_mlt_field_multitrack(%rip)
.size mlt_field_multitrack, . - mlt_field_multitrack

.global mlt_field_new
.type mlt_field_new, @function
mlt_field_new:
    jmp *real_mlt_field_new(%rip)
.size mlt_field_new, . - mlt_field_new

.global mlt_field_plant_filter
.type mlt_field_plant_filter, @function
mlt_field_plant_filter:
    jmp *real_mlt_field_plant_filter(%rip)
.size mlt_field_plant_filter, . - mlt_field_plant_filter

.global mlt_field_plant_transition
.type mlt_field_plant_transition, @function
mlt_field_plant_transition:
    jmp *real_mlt_field_plant_transition(%rip)
.size mlt_field_plant_transition, . - mlt_field_plant_transition

.global mlt_field_properties
.type mlt_field_properties, @function
mlt_field_properties:
    jmp *real_mlt_field_properties(%rip)
.size mlt_field_properties, . - mlt_field_properties

.global mlt_field_service
.type mlt_field_service, @function
mlt_field_service:
    jmp *real_mlt_field_service(%rip)
.size mlt_field_service, . - mlt_field_service

.global mlt_field_tractor
.type mlt_field_tractor, @function
mlt_field_tractor:
    jmp *real_mlt_field_tractor(%rip)
.size mlt_field_tractor, . - mlt_field_tractor

.global mlt_filter_close
.type mlt_filter_close, @function
mlt_filter_close:
    jmp *real_mlt_filter_close(%rip)
.size mlt_filter_close, . - mlt_filter_close

.global mlt_filter_connect
.type mlt_filter_connect, @function
mlt_filter_connect:
    jmp *real_mlt_filter_connect(%rip)
.size mlt_filter_connect, . - mlt_filter_connect

.global mlt_filter_get_in
.type mlt_filter_get_in, @function
mlt_filter_get_in:
    jmp *real_mlt_filter_get_in(%rip)
.size mlt_filter_get_in, . - mlt_filter_get_in

.global mlt_filter_get_length
.type mlt_filter_get_length, @function
mlt_filter_get_length:
    jmp *real_mlt_filter_get_length(%rip)
.size mlt_filter_get_length, . - mlt_filter_get_length

.global mlt_filter_get_length2
.type mlt_filter_get_length2, @function
mlt_filter_get_length2:
    jmp *real_mlt_filter_get_length2(%rip)
.size mlt_filter_get_length2, . - mlt_filter_get_length2

.global mlt_filter_get_out
.type mlt_filter_get_out, @function
mlt_filter_get_out:
    jmp *real_mlt_filter_get_out(%rip)
.size mlt_filter_get_out, . - mlt_filter_get_out

.global mlt_filter_get_position
.type mlt_filter_get_position, @function
mlt_filter_get_position:
    jmp *real_mlt_filter_get_position(%rip)
.size mlt_filter_get_position, . - mlt_filter_get_position

.global mlt_filter_get_progress
.type mlt_filter_get_progress, @function
mlt_filter_get_progress:
    jmp *real_mlt_filter_get_progress(%rip)
.size mlt_filter_get_progress, . - mlt_filter_get_progress

.global mlt_filter_get_track
.type mlt_filter_get_track, @function
mlt_filter_get_track:
    jmp *real_mlt_filter_get_track(%rip)
.size mlt_filter_get_track, . - mlt_filter_get_track

.global mlt_filter_init
.type mlt_filter_init, @function
mlt_filter_init:
    jmp *real_mlt_filter_init(%rip)
.size mlt_filter_init, . - mlt_filter_init

.global mlt_filter_new
.type mlt_filter_new, @function
mlt_filter_new:
    jmp *real_mlt_filter_new(%rip)
.size mlt_filter_new, . - mlt_filter_new

.global mlt_filter_process
.type mlt_filter_process, @function
mlt_filter_process:
    jmp *real_mlt_filter_process(%rip)
.size mlt_filter_process, . - mlt_filter_process

.global mlt_filter_properties
.type mlt_filter_properties, @function
mlt_filter_properties:
    jmp *real_mlt_filter_properties(%rip)
.size mlt_filter_properties, . - mlt_filter_properties

.global mlt_filter_service
.type mlt_filter_service, @function
mlt_filter_service:
    jmp *real_mlt_filter_service(%rip)
.size mlt_filter_service, . - mlt_filter_service

.global mlt_filter_set_in_and_out
.type mlt_filter_set_in_and_out, @function
mlt_filter_set_in_and_out:
    jmp *real_mlt_filter_set_in_and_out(%rip)
.size mlt_filter_set_in_and_out, . - mlt_filter_set_in_and_out

.global mlt_frame_append_convert_image
.type mlt_frame_append_convert_image, @function
mlt_frame_append_convert_image:
    jmp *real_mlt_frame_append_convert_image(%rip)
.size mlt_frame_append_convert_image, . - mlt_frame_append_convert_image

.global mlt_frame_clone
.type mlt_frame_clone, @function
mlt_frame_clone:
    jmp *real_mlt_frame_clone(%rip)
.size mlt_frame_clone, . - mlt_frame_clone

.global mlt_frame_clone_audio
.type mlt_frame_clone_audio, @function
mlt_frame_clone_audio:
    jmp *real_mlt_frame_clone_audio(%rip)
.size mlt_frame_clone_audio, . - mlt_frame_clone_audio

.global mlt_frame_clone_image
.type mlt_frame_clone_image, @function
mlt_frame_clone_image:
    jmp *real_mlt_frame_clone_image(%rip)
.size mlt_frame_clone_image, . - mlt_frame_clone_image

.global mlt_frame_close
.type mlt_frame_close, @function
mlt_frame_close:
    jmp *real_mlt_frame_close(%rip)
.size mlt_frame_close, . - mlt_frame_close

.global mlt_frame_convert_image
.type mlt_frame_convert_image, @function
mlt_frame_convert_image:
    jmp *real_mlt_frame_convert_image(%rip)
.size mlt_frame_convert_image, . - mlt_frame_convert_image

.global mlt_frame_copy_convert_image
.type mlt_frame_copy_convert_image, @function
mlt_frame_copy_convert_image:
    jmp *real_mlt_frame_copy_convert_image(%rip)
.size mlt_frame_copy_convert_image, . - mlt_frame_copy_convert_image

.global mlt_frame_get_alpha
.type mlt_frame_get_alpha, @function
mlt_frame_get_alpha:
    jmp *real_mlt_frame_get_alpha(%rip)
.size mlt_frame_get_alpha, . - mlt_frame_get_alpha

.global mlt_frame_get_alpha_size
.type mlt_frame_get_alpha_size, @function
mlt_frame_get_alpha_size:
    jmp *real_mlt_frame_get_alpha_size(%rip)
.size mlt_frame_get_alpha_size, . - mlt_frame_get_alpha_size

.global mlt_frame_get_aspect_ratio
.type mlt_frame_get_aspect_ratio, @function
mlt_frame_get_aspect_ratio:
    jmp *real_mlt_frame_get_aspect_ratio(%rip)
.size mlt_frame_get_aspect_ratio, . - mlt_frame_get_aspect_ratio

.global mlt_frame_get_audio
.type mlt_frame_get_audio, @function
mlt_frame_get_audio:
    jmp *real_mlt_frame_get_audio(%rip)
.size mlt_frame_get_audio, . - mlt_frame_get_audio

.global mlt_frame_get_image
.type mlt_frame_get_image, @function
mlt_frame_get_image:
    jmp *real_mlt_frame_get_image(%rip)
.size mlt_frame_get_image, . - mlt_frame_get_image

.global mlt_frame_get_original_producer
.type mlt_frame_get_original_producer, @function
mlt_frame_get_original_producer:
    jmp *real_mlt_frame_get_original_producer(%rip)
.size mlt_frame_get_original_producer, . - mlt_frame_get_original_producer

.global mlt_frame_get_position
.type mlt_frame_get_position, @function
mlt_frame_get_position:
    jmp *real_mlt_frame_get_position(%rip)
.size mlt_frame_get_position, . - mlt_frame_get_position

.global mlt_frame_get_unique_properties
.type mlt_frame_get_unique_properties, @function
mlt_frame_get_unique_properties:
    jmp *real_mlt_frame_get_unique_properties(%rip)
.size mlt_frame_get_unique_properties, . - mlt_frame_get_unique_properties

.global mlt_frame_get_waveform
.type mlt_frame_get_waveform, @function
mlt_frame_get_waveform:
    jmp *real_mlt_frame_get_waveform(%rip)
.size mlt_frame_get_waveform, . - mlt_frame_get_waveform

.global mlt_frame_has_convert_image
.type mlt_frame_has_convert_image, @function
mlt_frame_has_convert_image:
    jmp *real_mlt_frame_has_convert_image(%rip)
.size mlt_frame_has_convert_image, . - mlt_frame_has_convert_image

.global mlt_frame_init
.type mlt_frame_init, @function
mlt_frame_init:
    jmp *real_mlt_frame_init(%rip)
.size mlt_frame_init, . - mlt_frame_init

.global mlt_frame_is_test_audio
.type mlt_frame_is_test_audio, @function
mlt_frame_is_test_audio:
    jmp *real_mlt_frame_is_test_audio(%rip)
.size mlt_frame_is_test_audio, . - mlt_frame_is_test_audio

.global mlt_frame_is_test_card
.type mlt_frame_is_test_card, @function
mlt_frame_is_test_card:
    jmp *real_mlt_frame_is_test_card(%rip)
.size mlt_frame_is_test_card, . - mlt_frame_is_test_card

.global mlt_frame_next_convert_image
.type mlt_frame_next_convert_image, @function
mlt_frame_next_convert_image:
    jmp *real_mlt_frame_next_convert_image(%rip)
.size mlt_frame_next_convert_image, . - mlt_frame_next_convert_image

.global mlt_frame_original_position
.type mlt_frame_original_position, @function
mlt_frame_original_position:
    jmp *real_mlt_frame_original_position(%rip)
.size mlt_frame_original_position, . - mlt_frame_original_position

.global mlt_frame_pop_audio
.type mlt_frame_pop_audio, @function
mlt_frame_pop_audio:
    jmp *real_mlt_frame_pop_audio(%rip)
.size mlt_frame_pop_audio, . - mlt_frame_pop_audio

.global mlt_frame_pop_frame
.type mlt_frame_pop_frame, @function
mlt_frame_pop_frame:
    jmp *real_mlt_frame_pop_frame(%rip)
.size mlt_frame_pop_frame, . - mlt_frame_pop_frame

.global mlt_frame_pop_get_image
.type mlt_frame_pop_get_image, @function
mlt_frame_pop_get_image:
    jmp *real_mlt_frame_pop_get_image(%rip)
.size mlt_frame_pop_get_image, . - mlt_frame_pop_get_image

.global mlt_frame_pop_service
.type mlt_frame_pop_service, @function
mlt_frame_pop_service:
    jmp *real_mlt_frame_pop_service(%rip)
.size mlt_frame_pop_service, . - mlt_frame_pop_service

.global mlt_frame_pop_service_int
.type mlt_frame_pop_service_int, @function
mlt_frame_pop_service_int:
    jmp *real_mlt_frame_pop_service_int(%rip)
.size mlt_frame_pop_service_int, . - mlt_frame_pop_service_int

.global mlt_frame_prepend_convert_image
.type mlt_frame_prepend_convert_image, @function
mlt_frame_prepend_convert_image:
    jmp *real_mlt_frame_prepend_convert_image(%rip)
.size mlt_frame_prepend_convert_image, . - mlt_frame_prepend_convert_image

.global mlt_frame_prepend_image_from_service
.type mlt_frame_prepend_image_from_service, @function
mlt_frame_prepend_image_from_service:
    jmp *real_mlt_frame_prepend_image_from_service(%rip)
.size mlt_frame_prepend_image_from_service, . - mlt_frame_prepend_image_from_service

.global mlt_frame_properties
.type mlt_frame_properties, @function
mlt_frame_properties:
    jmp *real_mlt_frame_properties(%rip)
.size mlt_frame_properties, . - mlt_frame_properties

.global mlt_frame_push_audio
.type mlt_frame_push_audio, @function
mlt_frame_push_audio:
    jmp *real_mlt_frame_push_audio(%rip)
.size mlt_frame_push_audio, . - mlt_frame_push_audio

.global mlt_frame_push_frame
.type mlt_frame_push_frame, @function
mlt_frame_push_frame:
    jmp *real_mlt_frame_push_frame(%rip)
.size mlt_frame_push_frame, . - mlt_frame_push_frame

.global mlt_frame_push_get_image
.type mlt_frame_push_get_image, @function
mlt_frame_push_get_image:
    jmp *real_mlt_frame_push_get_image(%rip)
.size mlt_frame_push_get_image, . - mlt_frame_push_get_image

.global mlt_frame_push_image_with_fx_cut
.type mlt_frame_push_image_with_fx_cut, @function
mlt_frame_push_image_with_fx_cut:
    jmp *real_mlt_frame_push_image_with_fx_cut(%rip)
.size mlt_frame_push_image_with_fx_cut, . - mlt_frame_push_image_with_fx_cut

.global mlt_frame_push_service
.type mlt_frame_push_service, @function
mlt_frame_push_service:
    jmp *real_mlt_frame_push_service(%rip)
.size mlt_frame_push_service, . - mlt_frame_push_service

.global mlt_frame_push_service_int
.type mlt_frame_push_service_int, @function
mlt_frame_push_service_int:
    jmp *real_mlt_frame_push_service_int(%rip)
.size mlt_frame_push_service_int, . - mlt_frame_push_service_int

.global mlt_frame_replace_image
.type mlt_frame_replace_image, @function
mlt_frame_replace_image:
    jmp *real_mlt_frame_replace_image(%rip)
.size mlt_frame_replace_image, . - mlt_frame_replace_image

.global mlt_frame_service_stack
.type mlt_frame_service_stack, @function
mlt_frame_service_stack:
    jmp *real_mlt_frame_service_stack(%rip)
.size mlt_frame_service_stack, . - mlt_frame_service_stack

.global mlt_frame_set_alpha
.type mlt_frame_set_alpha, @function
mlt_frame_set_alpha:
    jmp *real_mlt_frame_set_alpha(%rip)
.size mlt_frame_set_alpha, . - mlt_frame_set_alpha

.global mlt_frame_set_aspect_ratio
.type mlt_frame_set_aspect_ratio, @function
mlt_frame_set_aspect_ratio:
    jmp *real_mlt_frame_set_aspect_ratio(%rip)
.size mlt_frame_set_aspect_ratio, . - mlt_frame_set_aspect_ratio

.global mlt_frame_set_audio
.type mlt_frame_set_audio, @function
mlt_frame_set_audio:
    jmp *real_mlt_frame_set_audio(%rip)
.size mlt_frame_set_audio, . - mlt_frame_set_audio

.global mlt_frame_set_image
.type mlt_frame_set_image, @function
mlt_frame_set_image:
    jmp *real_mlt_frame_set_image(%rip)
.size mlt_frame_set_image, . - mlt_frame_set_image

.global mlt_frame_set_position
.type mlt_frame_set_position, @function
mlt_frame_set_position:
    jmp *real_mlt_frame_set_position(%rip)
.size mlt_frame_set_position, . - mlt_frame_set_position

.global mlt_frame_unique_properties
.type mlt_frame_unique_properties, @function
mlt_frame_unique_properties:
    jmp *real_mlt_frame_unique_properties(%rip)
.size mlt_frame_unique_properties, . - mlt_frame_unique_properties

.global mlt_frame_write_ppm
.type mlt_frame_write_ppm, @function
mlt_frame_write_ppm:
    jmp *real_mlt_frame_write_ppm(%rip)
.size mlt_frame_write_ppm, . - mlt_frame_write_ppm

.global mlt_global_properties
.type mlt_global_properties, @function
mlt_global_properties:
    jmp *real_mlt_global_properties(%rip)
.size mlt_global_properties, . - mlt_global_properties

.global mlt_image_alloc_alpha
.type mlt_image_alloc_alpha, @function
mlt_image_alloc_alpha:
    jmp *real_mlt_image_alloc_alpha(%rip)
.size mlt_image_alloc_alpha, . - mlt_image_alloc_alpha

.global mlt_image_alloc_data
.type mlt_image_alloc_data, @function
mlt_image_alloc_data:
    jmp *real_mlt_image_alloc_data(%rip)
.size mlt_image_alloc_data, . - mlt_image_alloc_data

.global mlt_image_calculate_size
.type mlt_image_calculate_size, @function
mlt_image_calculate_size:
    jmp *real_mlt_image_calculate_size(%rip)
.size mlt_image_calculate_size, . - mlt_image_calculate_size

.global mlt_image_close
.type mlt_image_close, @function
mlt_image_close:
    jmp *real_mlt_image_close(%rip)
.size mlt_image_close, . - mlt_image_close

.global mlt_image_color_pri_id
.type mlt_image_color_pri_id, @function
mlt_image_color_pri_id:
    jmp *real_mlt_image_color_pri_id(%rip)
.size mlt_image_color_pri_id, . - mlt_image_color_pri_id

.global mlt_image_color_pri_name
.type mlt_image_color_pri_name, @function
mlt_image_color_pri_name:
    jmp *real_mlt_image_color_pri_name(%rip)
.size mlt_image_color_pri_name, . - mlt_image_color_pri_name

.global mlt_image_color_trc_id
.type mlt_image_color_trc_id, @function
mlt_image_color_trc_id:
    jmp *real_mlt_image_color_trc_id(%rip)
.size mlt_image_color_trc_id, . - mlt_image_color_trc_id

.global mlt_image_color_trc_name
.type mlt_image_color_trc_name, @function
mlt_image_color_trc_name:
    jmp *real_mlt_image_color_trc_name(%rip)
.size mlt_image_color_trc_name, . - mlt_image_color_trc_name

.global mlt_image_colorspace_id
.type mlt_image_colorspace_id, @function
mlt_image_colorspace_id:
    jmp *real_mlt_image_colorspace_id(%rip)
.size mlt_image_colorspace_id, . - mlt_image_colorspace_id

.global mlt_image_colorspace_name
.type mlt_image_colorspace_name, @function
mlt_image_colorspace_name:
    jmp *real_mlt_image_colorspace_name(%rip)
.size mlt_image_colorspace_name, . - mlt_image_colorspace_name

.global mlt_image_default_colorspace
.type mlt_image_default_colorspace, @function
mlt_image_default_colorspace:
    jmp *real_mlt_image_default_colorspace(%rip)
.size mlt_image_default_colorspace, . - mlt_image_default_colorspace

.global mlt_image_default_primaries
.type mlt_image_default_primaries, @function
mlt_image_default_primaries:
    jmp *real_mlt_image_default_primaries(%rip)
.size mlt_image_default_primaries, . - mlt_image_default_primaries

.global mlt_image_default_trc
.type mlt_image_default_trc, @function
mlt_image_default_trc:
    jmp *real_mlt_image_default_trc(%rip)
.size mlt_image_default_trc, . - mlt_image_default_trc

.global mlt_image_fill_black
.type mlt_image_fill_black, @function
mlt_image_fill_black:
    jmp *real_mlt_image_fill_black(%rip)
.size mlt_image_fill_black, . - mlt_image_fill_black

.global mlt_image_fill_checkerboard
.type mlt_image_fill_checkerboard, @function
mlt_image_fill_checkerboard:
    jmp *real_mlt_image_fill_checkerboard(%rip)
.size mlt_image_fill_checkerboard, . - mlt_image_fill_checkerboard

.global mlt_image_fill_opaque
.type mlt_image_fill_opaque, @function
mlt_image_fill_opaque:
    jmp *real_mlt_image_fill_opaque(%rip)
.size mlt_image_fill_opaque, . - mlt_image_fill_opaque

.global mlt_image_fill_white
.type mlt_image_fill_white, @function
mlt_image_fill_white:
    jmp *real_mlt_image_fill_white(%rip)
.size mlt_image_fill_white, . - mlt_image_fill_white

.global mlt_image_format_id
.type mlt_image_format_id, @function
mlt_image_format_id:
    jmp *real_mlt_image_format_id(%rip)
.size mlt_image_format_id, . - mlt_image_format_id

.global mlt_image_format_name
.type mlt_image_format_name, @function
mlt_image_format_name:
    jmp *real_mlt_image_format_name(%rip)
.size mlt_image_format_name, . - mlt_image_format_name

.global mlt_image_format_planes
.type mlt_image_format_planes, @function
mlt_image_format_planes:
    jmp *real_mlt_image_format_planes(%rip)
.size mlt_image_format_planes, . - mlt_image_format_planes

.global mlt_image_format_size
.type mlt_image_format_size, @function
mlt_image_format_size:
    jmp *real_mlt_image_format_size(%rip)
.size mlt_image_format_size, . - mlt_image_format_size

.global mlt_image_full_range
.type mlt_image_full_range, @function
mlt_image_full_range:
    jmp *real_mlt_image_full_range(%rip)
.size mlt_image_full_range, . - mlt_image_full_range

.global mlt_image_get_values
.type mlt_image_get_values, @function
mlt_image_get_values:
    jmp *real_mlt_image_get_values(%rip)
.size mlt_image_get_values, . - mlt_image_get_values

.global mlt_image_is_opaque
.type mlt_image_is_opaque, @function
mlt_image_is_opaque:
    jmp *real_mlt_image_is_opaque(%rip)
.size mlt_image_is_opaque, . - mlt_image_is_opaque

.global mlt_image_new
.type mlt_image_new, @function
mlt_image_new:
    jmp *real_mlt_image_new(%rip)
.size mlt_image_new, . - mlt_image_new

.global mlt_image_rgba_opaque
.type mlt_image_rgba_opaque, @function
mlt_image_rgba_opaque:
    jmp *real_mlt_image_rgba_opaque(%rip)
.size mlt_image_rgba_opaque, . - mlt_image_rgba_opaque

.global mlt_image_set_values
.type mlt_image_set_values, @function
mlt_image_set_values:
    jmp *real_mlt_image_set_values(%rip)
.size mlt_image_set_values, . - mlt_image_set_values

.global mlt_link_close
.type mlt_link_close, @function
mlt_link_close:
    jmp *real_mlt_link_close(%rip)
.size mlt_link_close, . - mlt_link_close

.global mlt_link_connect_next
.type mlt_link_connect_next, @function
mlt_link_connect_next:
    jmp *real_mlt_link_connect_next(%rip)
.size mlt_link_connect_next, . - mlt_link_connect_next

.global mlt_link_filter_init
.type mlt_link_filter_init, @function
mlt_link_filter_init:
    jmp *real_mlt_link_filter_init(%rip)
.size mlt_link_filter_init, . - mlt_link_filter_init

.global mlt_link_filter_metadata
.type mlt_link_filter_metadata, @function
mlt_link_filter_metadata:
    jmp *real_mlt_link_filter_metadata(%rip)
.size mlt_link_filter_metadata, . - mlt_link_filter_metadata

.global mlt_link_init
.type mlt_link_init, @function
mlt_link_init:
    jmp *real_mlt_link_init(%rip)
.size mlt_link_init, . - mlt_link_init

.global mlt_log
.type mlt_log, @function
mlt_log:
    jmp *real_mlt_log(%rip)
.size mlt_log, . - mlt_log

.global mlt_log_get_level
.type mlt_log_get_level, @function
mlt_log_get_level:
    jmp *real_mlt_log_get_level(%rip)
.size mlt_log_get_level, . - mlt_log_get_level

.global mlt_log_set_callback
.type mlt_log_set_callback, @function
mlt_log_set_callback:
    jmp *real_mlt_log_set_callback(%rip)
.size mlt_log_set_callback, . - mlt_log_set_callback

.global mlt_log_set_level
.type mlt_log_set_level, @function
mlt_log_set_level:
    jmp *real_mlt_log_set_level(%rip)
.size mlt_log_set_level, . - mlt_log_set_level

.global mlt_log_timings_now
.type mlt_log_timings_now, @function
mlt_log_timings_now:
    jmp *real_mlt_log_timings_now(%rip)
.size mlt_log_timings_now, . - mlt_log_timings_now

.global mlt_luma_map_from_pgm
.type mlt_luma_map_from_pgm, @function
mlt_luma_map_from_pgm:
    jmp *real_mlt_luma_map_from_pgm(%rip)
.size mlt_luma_map_from_pgm, . - mlt_luma_map_from_pgm

.global mlt_luma_map_from_yuv422
.type mlt_luma_map_from_yuv422, @function
mlt_luma_map_from_yuv422:
    jmp *real_mlt_luma_map_from_yuv422(%rip)
.size mlt_luma_map_from_yuv422, . - mlt_luma_map_from_yuv422

.global mlt_luma_map_init
.type mlt_luma_map_init, @function
mlt_luma_map_init:
    jmp *real_mlt_luma_map_init(%rip)
.size mlt_luma_map_init, . - mlt_luma_map_init

.global mlt_luma_map_new
.type mlt_luma_map_new, @function
mlt_luma_map_new:
    jmp *real_mlt_luma_map_new(%rip)
.size mlt_luma_map_new, . - mlt_luma_map_new

.global mlt_luma_map_render
.type mlt_luma_map_render, @function
mlt_luma_map_render:
    jmp *real_mlt_luma_map_render(%rip)
.size mlt_luma_map_render, . - mlt_luma_map_render

.global mlt_multitrack_clip
.type mlt_multitrack_clip, @function
mlt_multitrack_clip:
    jmp *real_mlt_multitrack_clip(%rip)
.size mlt_multitrack_clip, . - mlt_multitrack_clip

.global mlt_multitrack_close
.type mlt_multitrack_close, @function
mlt_multitrack_close:
    jmp *real_mlt_multitrack_close(%rip)
.size mlt_multitrack_close, . - mlt_multitrack_close

.global mlt_multitrack_connect
.type mlt_multitrack_connect, @function
mlt_multitrack_connect:
    jmp *real_mlt_multitrack_connect(%rip)
.size mlt_multitrack_connect, . - mlt_multitrack_connect

.global mlt_multitrack_count
.type mlt_multitrack_count, @function
mlt_multitrack_count:
    jmp *real_mlt_multitrack_count(%rip)
.size mlt_multitrack_count, . - mlt_multitrack_count

.global mlt_multitrack_disconnect
.type mlt_multitrack_disconnect, @function
mlt_multitrack_disconnect:
    jmp *real_mlt_multitrack_disconnect(%rip)
.size mlt_multitrack_disconnect, . - mlt_multitrack_disconnect

.global mlt_multitrack_init
.type mlt_multitrack_init, @function
mlt_multitrack_init:
    jmp *real_mlt_multitrack_init(%rip)
.size mlt_multitrack_init, . - mlt_multitrack_init

.global mlt_multitrack_insert
.type mlt_multitrack_insert, @function
mlt_multitrack_insert:
    jmp *real_mlt_multitrack_insert(%rip)
.size mlt_multitrack_insert, . - mlt_multitrack_insert

.global mlt_multitrack_producer
.type mlt_multitrack_producer, @function
mlt_multitrack_producer:
    jmp *real_mlt_multitrack_producer(%rip)
.size mlt_multitrack_producer, . - mlt_multitrack_producer

.global mlt_multitrack_properties
.type mlt_multitrack_properties, @function
mlt_multitrack_properties:
    jmp *real_mlt_multitrack_properties(%rip)
.size mlt_multitrack_properties, . - mlt_multitrack_properties

.global mlt_multitrack_refresh
.type mlt_multitrack_refresh, @function
mlt_multitrack_refresh:
    jmp *real_mlt_multitrack_refresh(%rip)
.size mlt_multitrack_refresh, . - mlt_multitrack_refresh

.global mlt_multitrack_service
.type mlt_multitrack_service, @function
mlt_multitrack_service:
    jmp *real_mlt_multitrack_service(%rip)
.size mlt_multitrack_service, . - mlt_multitrack_service

.global mlt_multitrack_track
.type mlt_multitrack_track, @function
mlt_multitrack_track:
    jmp *real_mlt_multitrack_track(%rip)
.size mlt_multitrack_track, . - mlt_multitrack_track

.global mlt_parser_close
.type mlt_parser_close, @function
mlt_parser_close:
    jmp *real_mlt_parser_close(%rip)
.size mlt_parser_close, . - mlt_parser_close

.global mlt_parser_new
.type mlt_parser_new, @function
mlt_parser_new:
    jmp *real_mlt_parser_new(%rip)
.size mlt_parser_new, . - mlt_parser_new

.global mlt_parser_properties
.type mlt_parser_properties, @function
mlt_parser_properties:
    jmp *real_mlt_parser_properties(%rip)
.size mlt_parser_properties, . - mlt_parser_properties

.global mlt_parser_start
.type mlt_parser_start, @function
mlt_parser_start:
    jmp *real_mlt_parser_start(%rip)
.size mlt_parser_start, . - mlt_parser_start

.global mlt_playlist_append
.type mlt_playlist_append, @function
mlt_playlist_append:
    jmp *real_mlt_playlist_append(%rip)
.size mlt_playlist_append, . - mlt_playlist_append

.global mlt_playlist_append_io
.type mlt_playlist_append_io, @function
mlt_playlist_append_io:
    jmp *real_mlt_playlist_append_io(%rip)
.size mlt_playlist_append_io, . - mlt_playlist_append_io

.global mlt_playlist_blank
.type mlt_playlist_blank, @function
mlt_playlist_blank:
    jmp *real_mlt_playlist_blank(%rip)
.size mlt_playlist_blank, . - mlt_playlist_blank

.global mlt_playlist_blank_time
.type mlt_playlist_blank_time, @function
mlt_playlist_blank_time:
    jmp *real_mlt_playlist_blank_time(%rip)
.size mlt_playlist_blank_time, . - mlt_playlist_blank_time

.global mlt_playlist_blanks_from
.type mlt_playlist_blanks_from, @function
mlt_playlist_blanks_from:
    jmp *real_mlt_playlist_blanks_from(%rip)
.size mlt_playlist_blanks_from, . - mlt_playlist_blanks_from

.global mlt_playlist_clear
.type mlt_playlist_clear, @function
mlt_playlist_clear:
    jmp *real_mlt_playlist_clear(%rip)
.size mlt_playlist_clear, . - mlt_playlist_clear

.global mlt_playlist_clip
.type mlt_playlist_clip, @function
mlt_playlist_clip:
    jmp *real_mlt_playlist_clip(%rip)
.size mlt_playlist_clip, . - mlt_playlist_clip

.global mlt_playlist_clip_is_mix
.type mlt_playlist_clip_is_mix, @function
mlt_playlist_clip_is_mix:
    jmp *real_mlt_playlist_clip_is_mix(%rip)
.size mlt_playlist_clip_is_mix, . - mlt_playlist_clip_is_mix

.global mlt_playlist_clip_length
.type mlt_playlist_clip_length, @function
mlt_playlist_clip_length:
    jmp *real_mlt_playlist_clip_length(%rip)
.size mlt_playlist_clip_length, . - mlt_playlist_clip_length

.global mlt_playlist_clip_start
.type mlt_playlist_clip_start, @function
mlt_playlist_clip_start:
    jmp *real_mlt_playlist_clip_start(%rip)
.size mlt_playlist_clip_start, . - mlt_playlist_clip_start

.global mlt_playlist_close
.type mlt_playlist_close, @function
mlt_playlist_close:
    jmp *real_mlt_playlist_close(%rip)
.size mlt_playlist_close, . - mlt_playlist_close

.global mlt_playlist_consolidate_blanks
.type mlt_playlist_consolidate_blanks, @function
mlt_playlist_consolidate_blanks:
    jmp *real_mlt_playlist_consolidate_blanks(%rip)
.size mlt_playlist_consolidate_blanks, . - mlt_playlist_consolidate_blanks

.global mlt_playlist_count
.type mlt_playlist_count, @function
mlt_playlist_count:
    jmp *real_mlt_playlist_count(%rip)
.size mlt_playlist_count, . - mlt_playlist_count

.global mlt_playlist_current
.type mlt_playlist_current, @function
mlt_playlist_current:
    jmp *real_mlt_playlist_current(%rip)
.size mlt_playlist_current, . - mlt_playlist_current

.global mlt_playlist_current_clip
.type mlt_playlist_current_clip, @function
mlt_playlist_current_clip:
    jmp *real_mlt_playlist_current_clip(%rip)
.size mlt_playlist_current_clip, . - mlt_playlist_current_clip

.global mlt_playlist_get_clip
.type mlt_playlist_get_clip, @function
mlt_playlist_get_clip:
    jmp *real_mlt_playlist_get_clip(%rip)
.size mlt_playlist_get_clip, . - mlt_playlist_get_clip

.global mlt_playlist_get_clip_at
.type mlt_playlist_get_clip_at, @function
mlt_playlist_get_clip_at:
    jmp *real_mlt_playlist_get_clip_at(%rip)
.size mlt_playlist_get_clip_at, . - mlt_playlist_get_clip_at

.global mlt_playlist_get_clip_index_at
.type mlt_playlist_get_clip_index_at, @function
mlt_playlist_get_clip_index_at:
    jmp *real_mlt_playlist_get_clip_index_at(%rip)
.size mlt_playlist_get_clip_index_at, . - mlt_playlist_get_clip_index_at

.global mlt_playlist_get_clip_info
.type mlt_playlist_get_clip_info, @function
mlt_playlist_get_clip_info:
    jmp *real_mlt_playlist_get_clip_info(%rip)
.size mlt_playlist_get_clip_info, . - mlt_playlist_get_clip_info

.global mlt_playlist_init
.type mlt_playlist_init, @function
mlt_playlist_init:
    jmp *real_mlt_playlist_init(%rip)
.size mlt_playlist_init, . - mlt_playlist_init

.global mlt_playlist_insert
.type mlt_playlist_insert, @function
mlt_playlist_insert:
    jmp *real_mlt_playlist_insert(%rip)
.size mlt_playlist_insert, . - mlt_playlist_insert

.global mlt_playlist_insert_at
.type mlt_playlist_insert_at, @function
mlt_playlist_insert_at:
    jmp *real_mlt_playlist_insert_at(%rip)
.size mlt_playlist_insert_at, . - mlt_playlist_insert_at

.global mlt_playlist_insert_blank
.type mlt_playlist_insert_blank, @function
mlt_playlist_insert_blank:
    jmp *real_mlt_playlist_insert_blank(%rip)
.size mlt_playlist_insert_blank, . - mlt_playlist_insert_blank

.global mlt_playlist_is_blank
.type mlt_playlist_is_blank, @function
mlt_playlist_is_blank:
    jmp *real_mlt_playlist_is_blank(%rip)
.size mlt_playlist_is_blank, . - mlt_playlist_is_blank

.global mlt_playlist_is_blank_at
.type mlt_playlist_is_blank_at, @function
mlt_playlist_is_blank_at:
    jmp *real_mlt_playlist_is_blank_at(%rip)
.size mlt_playlist_is_blank_at, . - mlt_playlist_is_blank_at

.global mlt_playlist_join
.type mlt_playlist_join, @function
mlt_playlist_join:
    jmp *real_mlt_playlist_join(%rip)
.size mlt_playlist_join, . - mlt_playlist_join

.global mlt_playlist_mix
.type mlt_playlist_mix, @function
mlt_playlist_mix:
    jmp *real_mlt_playlist_mix(%rip)
.size mlt_playlist_mix, . - mlt_playlist_mix

.global mlt_playlist_mix_add
.type mlt_playlist_mix_add, @function
mlt_playlist_mix_add:
    jmp *real_mlt_playlist_mix_add(%rip)
.size mlt_playlist_mix_add, . - mlt_playlist_mix_add

.global mlt_playlist_mix_in
.type mlt_playlist_mix_in, @function
mlt_playlist_mix_in:
    jmp *real_mlt_playlist_mix_in(%rip)
.size mlt_playlist_mix_in, . - mlt_playlist_mix_in

.global mlt_playlist_mix_out
.type mlt_playlist_mix_out, @function
mlt_playlist_mix_out:
    jmp *real_mlt_playlist_mix_out(%rip)
.size mlt_playlist_mix_out, . - mlt_playlist_mix_out

.global mlt_playlist_move
.type mlt_playlist_move, @function
mlt_playlist_move:
    jmp *real_mlt_playlist_move(%rip)
.size mlt_playlist_move, . - mlt_playlist_move

.global mlt_playlist_new
.type mlt_playlist_new, @function
mlt_playlist_new:
    jmp *real_mlt_playlist_new(%rip)
.size mlt_playlist_new, . - mlt_playlist_new

.global mlt_playlist_pad_blanks
.type mlt_playlist_pad_blanks, @function
mlt_playlist_pad_blanks:
    jmp *real_mlt_playlist_pad_blanks(%rip)
.size mlt_playlist_pad_blanks, . - mlt_playlist_pad_blanks

.global mlt_playlist_producer
.type mlt_playlist_producer, @function
mlt_playlist_producer:
    jmp *real_mlt_playlist_producer(%rip)
.size mlt_playlist_producer, . - mlt_playlist_producer

.global mlt_playlist_properties
.type mlt_playlist_properties, @function
mlt_playlist_properties:
    jmp *real_mlt_playlist_properties(%rip)
.size mlt_playlist_properties, . - mlt_playlist_properties

.global mlt_playlist_remove
.type mlt_playlist_remove, @function
mlt_playlist_remove:
    jmp *real_mlt_playlist_remove(%rip)
.size mlt_playlist_remove, . - mlt_playlist_remove

.global mlt_playlist_remove_region
.type mlt_playlist_remove_region, @function
mlt_playlist_remove_region:
    jmp *real_mlt_playlist_remove_region(%rip)
.size mlt_playlist_remove_region, . - mlt_playlist_remove_region

.global mlt_playlist_reorder
.type mlt_playlist_reorder, @function
mlt_playlist_reorder:
    jmp *real_mlt_playlist_reorder(%rip)
.size mlt_playlist_reorder, . - mlt_playlist_reorder

.global mlt_playlist_repeat_clip
.type mlt_playlist_repeat_clip, @function
mlt_playlist_repeat_clip:
    jmp *real_mlt_playlist_repeat_clip(%rip)
.size mlt_playlist_repeat_clip, . - mlt_playlist_repeat_clip

.global mlt_playlist_replace_with_blank
.type mlt_playlist_replace_with_blank, @function
mlt_playlist_replace_with_blank:
    jmp *real_mlt_playlist_replace_with_blank(%rip)
.size mlt_playlist_replace_with_blank, . - mlt_playlist_replace_with_blank

.global mlt_playlist_resize_clip
.type mlt_playlist_resize_clip, @function
mlt_playlist_resize_clip:
    jmp *real_mlt_playlist_resize_clip(%rip)
.size mlt_playlist_resize_clip, . - mlt_playlist_resize_clip

.global mlt_playlist_service
.type mlt_playlist_service, @function
mlt_playlist_service:
    jmp *real_mlt_playlist_service(%rip)
.size mlt_playlist_service, . - mlt_playlist_service

.global mlt_playlist_split
.type mlt_playlist_split, @function
mlt_playlist_split:
    jmp *real_mlt_playlist_split(%rip)
.size mlt_playlist_split, . - mlt_playlist_split

.global mlt_playlist_split_at
.type mlt_playlist_split_at, @function
mlt_playlist_split_at:
    jmp *real_mlt_playlist_split_at(%rip)
.size mlt_playlist_split_at, . - mlt_playlist_split_at

.global mlt_pool_alloc
.type mlt_pool_alloc, @function
mlt_pool_alloc:
    jmp *real_mlt_pool_alloc(%rip)
.size mlt_pool_alloc, . - mlt_pool_alloc

.global mlt_pool_close
.type mlt_pool_close, @function
mlt_pool_close:
    jmp *real_mlt_pool_close(%rip)
.size mlt_pool_close, . - mlt_pool_close

.global mlt_pool_init
.type mlt_pool_init, @function
mlt_pool_init:
    jmp *real_mlt_pool_init(%rip)
.size mlt_pool_init, . - mlt_pool_init

.global mlt_pool_purge
.type mlt_pool_purge, @function
mlt_pool_purge:
    jmp *real_mlt_pool_purge(%rip)
.size mlt_pool_purge, . - mlt_pool_purge

.global mlt_pool_realloc
.type mlt_pool_realloc, @function
mlt_pool_realloc:
    jmp *real_mlt_pool_realloc(%rip)
.size mlt_pool_realloc, . - mlt_pool_realloc

.global mlt_pool_release
.type mlt_pool_release, @function
mlt_pool_release:
    jmp *real_mlt_pool_release(%rip)
.size mlt_pool_release, . - mlt_pool_release

.global mlt_pool_stat
.type mlt_pool_stat, @function
mlt_pool_stat:
    jmp *real_mlt_pool_stat(%rip)
.size mlt_pool_stat, . - mlt_pool_stat

.global mlt_producer_attach
.type mlt_producer_attach, @function
mlt_producer_attach:
    jmp *real_mlt_producer_attach(%rip)
.size mlt_producer_attach, . - mlt_producer_attach

.global mlt_producer_clear
.type mlt_producer_clear, @function
mlt_producer_clear:
    jmp *real_mlt_producer_clear(%rip)
.size mlt_producer_clear, . - mlt_producer_clear

.global mlt_producer_close
.type mlt_producer_close, @function
mlt_producer_close:
    jmp *real_mlt_producer_close(%rip)
.size mlt_producer_close, . - mlt_producer_close

.global mlt_producer_cut
.type mlt_producer_cut, @function
mlt_producer_cut:
    jmp *real_mlt_producer_cut(%rip)
.size mlt_producer_cut, . - mlt_producer_cut

.global mlt_producer_cut_parent
.type mlt_producer_cut_parent, @function
mlt_producer_cut_parent:
    jmp *real_mlt_producer_cut_parent(%rip)
.size mlt_producer_cut_parent, . - mlt_producer_cut_parent

.global mlt_producer_detach
.type mlt_producer_detach, @function
mlt_producer_detach:
    jmp *real_mlt_producer_detach(%rip)
.size mlt_producer_detach, . - mlt_producer_detach

.global mlt_producer_filter
.type mlt_producer_filter, @function
mlt_producer_filter:
    jmp *real_mlt_producer_filter(%rip)
.size mlt_producer_filter, . - mlt_producer_filter

.global mlt_producer_frame
.type mlt_producer_frame, @function
mlt_producer_frame:
    jmp *real_mlt_producer_frame(%rip)
.size mlt_producer_frame, . - mlt_producer_frame

.global mlt_producer_frame_time
.type mlt_producer_frame_time, @function
mlt_producer_frame_time:
    jmp *real_mlt_producer_frame_time(%rip)
.size mlt_producer_frame_time, . - mlt_producer_frame_time

.global mlt_producer_get_creation_time
.type mlt_producer_get_creation_time, @function
mlt_producer_get_creation_time:
    jmp *real_mlt_producer_get_creation_time(%rip)
.size mlt_producer_get_creation_time, . - mlt_producer_get_creation_time

.global mlt_producer_get_fps
.type mlt_producer_get_fps, @function
mlt_producer_get_fps:
    jmp *real_mlt_producer_get_fps(%rip)
.size mlt_producer_get_fps, . - mlt_producer_get_fps

.global mlt_producer_get_in
.type mlt_producer_get_in, @function
mlt_producer_get_in:
    jmp *real_mlt_producer_get_in(%rip)
.size mlt_producer_get_in, . - mlt_producer_get_in

.global mlt_producer_get_length
.type mlt_producer_get_length, @function
mlt_producer_get_length:
    jmp *real_mlt_producer_get_length(%rip)
.size mlt_producer_get_length, . - mlt_producer_get_length

.global mlt_producer_get_length_time
.type mlt_producer_get_length_time, @function
mlt_producer_get_length_time:
    jmp *real_mlt_producer_get_length_time(%rip)
.size mlt_producer_get_length_time, . - mlt_producer_get_length_time

.global mlt_producer_get_out
.type mlt_producer_get_out, @function
mlt_producer_get_out:
    jmp *real_mlt_producer_get_out(%rip)
.size mlt_producer_get_out, . - mlt_producer_get_out

.global mlt_producer_get_playtime
.type mlt_producer_get_playtime, @function
mlt_producer_get_playtime:
    jmp *real_mlt_producer_get_playtime(%rip)
.size mlt_producer_get_playtime, . - mlt_producer_get_playtime

.global mlt_producer_get_speed
.type mlt_producer_get_speed, @function
mlt_producer_get_speed:
    jmp *real_mlt_producer_get_speed(%rip)
.size mlt_producer_get_speed, . - mlt_producer_get_speed

.global mlt_producer_init
.type mlt_producer_init, @function
mlt_producer_init:
    jmp *real_mlt_producer_init(%rip)
.size mlt_producer_init, . - mlt_producer_init

.global mlt_producer_is_blank
.type mlt_producer_is_blank, @function
mlt_producer_is_blank:
    jmp *real_mlt_producer_is_blank(%rip)
.size mlt_producer_is_blank, . - mlt_producer_is_blank

.global mlt_producer_is_cut
.type mlt_producer_is_cut, @function
mlt_producer_is_cut:
    jmp *real_mlt_producer_is_cut(%rip)
.size mlt_producer_is_cut, . - mlt_producer_is_cut

.global mlt_producer_is_mix
.type mlt_producer_is_mix, @function
mlt_producer_is_mix:
    jmp *real_mlt_producer_is_mix(%rip)
.size mlt_producer_is_mix, . - mlt_producer_is_mix

.global mlt_producer_new
.type mlt_producer_new, @function
mlt_producer_new:
    jmp *real_mlt_producer_new(%rip)
.size mlt_producer_new, . - mlt_producer_new

.global mlt_producer_optimise
.type mlt_producer_optimise, @function
mlt_producer_optimise:
    jmp *real_mlt_producer_optimise(%rip)
.size mlt_producer_optimise, . - mlt_producer_optimise

.global mlt_producer_pass_frame_properties
.type mlt_producer_pass_frame_properties, @function
mlt_producer_pass_frame_properties:
    jmp *real_mlt_producer_pass_frame_properties(%rip)
.size mlt_producer_pass_frame_properties, . - mlt_producer_pass_frame_properties

.global mlt_producer_position
.type mlt_producer_position, @function
mlt_producer_position:
    jmp *real_mlt_producer_position(%rip)
.size mlt_producer_position, . - mlt_producer_position

.global mlt_producer_prepare_next
.type mlt_producer_prepare_next, @function
mlt_producer_prepare_next:
    jmp *real_mlt_producer_prepare_next(%rip)
.size mlt_producer_prepare_next, . - mlt_producer_prepare_next

.global mlt_producer_probe
.type mlt_producer_probe, @function
mlt_producer_probe:
    jmp *real_mlt_producer_probe(%rip)
.size mlt_producer_probe, . - mlt_producer_probe

.global mlt_producer_properties
.type mlt_producer_properties, @function
mlt_producer_properties:
    jmp *real_mlt_producer_properties(%rip)
.size mlt_producer_properties, . - mlt_producer_properties

.global mlt_producer_seek
.type mlt_producer_seek, @function
mlt_producer_seek:
    jmp *real_mlt_producer_seek(%rip)
.size mlt_producer_seek, . - mlt_producer_seek

.global mlt_producer_seek_time
.type mlt_producer_seek_time, @function
mlt_producer_seek_time:
    jmp *real_mlt_producer_seek_time(%rip)
.size mlt_producer_seek_time, . - mlt_producer_seek_time

.global mlt_producer_service
.type mlt_producer_service, @function
mlt_producer_service:
    jmp *real_mlt_producer_service(%rip)
.size mlt_producer_service, . - mlt_producer_service

.global mlt_producer_set_creation_time
.type mlt_producer_set_creation_time, @function
mlt_producer_set_creation_time:
    jmp *real_mlt_producer_set_creation_time(%rip)
.size mlt_producer_set_creation_time, . - mlt_producer_set_creation_time

.global mlt_producer_set_in_and_out
.type mlt_producer_set_in_and_out, @function
mlt_producer_set_in_and_out:
    jmp *real_mlt_producer_set_in_and_out(%rip)
.size mlt_producer_set_in_and_out, . - mlt_producer_set_in_and_out

.global mlt_producer_set_speed
.type mlt_producer_set_speed, @function
mlt_producer_set_speed:
    jmp *real_mlt_producer_set_speed(%rip)
.size mlt_producer_set_speed, . - mlt_producer_set_speed

.global mlt_profile_clone
.type mlt_profile_clone, @function
mlt_profile_clone:
    jmp *real_mlt_profile_clone(%rip)
.size mlt_profile_clone, . - mlt_profile_clone

.global mlt_profile_close
.type mlt_profile_close, @function
mlt_profile_close:
    jmp *real_mlt_profile_close(%rip)
.size mlt_profile_close, . - mlt_profile_close

.global mlt_profile_dar
.type mlt_profile_dar, @function
mlt_profile_dar:
    jmp *real_mlt_profile_dar(%rip)
.size mlt_profile_dar, . - mlt_profile_dar

.global mlt_profile_fps
.type mlt_profile_fps, @function
mlt_profile_fps:
    jmp *real_mlt_profile_fps(%rip)
.size mlt_profile_fps, . - mlt_profile_fps

.global mlt_profile_from_producer
.type mlt_profile_from_producer, @function
mlt_profile_from_producer:
    jmp *real_mlt_profile_from_producer(%rip)
.size mlt_profile_from_producer, . - mlt_profile_from_producer

.global mlt_profile_init
.type mlt_profile_init, @function
mlt_profile_init:
    jmp rust_mlt_profile_init
.size mlt_profile_init, . - mlt_profile_init

.global mlt_profile_is_valid
.type mlt_profile_is_valid, @function
mlt_profile_is_valid:
    jmp *real_mlt_profile_is_valid(%rip)
.size mlt_profile_is_valid, . - mlt_profile_is_valid

.global mlt_profile_list
.type mlt_profile_list, @function
mlt_profile_list:
    jmp *real_mlt_profile_list(%rip)
.size mlt_profile_list, . - mlt_profile_list

.global mlt_profile_load_file
.type mlt_profile_load_file, @function
mlt_profile_load_file:
    jmp *real_mlt_profile_load_file(%rip)
.size mlt_profile_load_file, . - mlt_profile_load_file

.global mlt_profile_load_properties
.type mlt_profile_load_properties, @function
mlt_profile_load_properties:
    jmp *real_mlt_profile_load_properties(%rip)
.size mlt_profile_load_properties, . - mlt_profile_load_properties

.global mlt_profile_load_string
.type mlt_profile_load_string, @function
mlt_profile_load_string:
    jmp *real_mlt_profile_load_string(%rip)
.size mlt_profile_load_string, . - mlt_profile_load_string

.global mlt_profile_lumas_dir
.type mlt_profile_lumas_dir, @function
mlt_profile_lumas_dir:
    jmp *real_mlt_profile_lumas_dir(%rip)
.size mlt_profile_lumas_dir, . - mlt_profile_lumas_dir

.global mlt_profile_sar
.type mlt_profile_sar, @function
mlt_profile_sar:
    jmp *real_mlt_profile_sar(%rip)
.size mlt_profile_sar, . - mlt_profile_sar

.global mlt_profile_scale_height
.type mlt_profile_scale_height, @function
mlt_profile_scale_height:
    jmp *real_mlt_profile_scale_height(%rip)
.size mlt_profile_scale_height, . - mlt_profile_scale_height

.global mlt_profile_scale_width
.type mlt_profile_scale_width, @function
mlt_profile_scale_width:
    jmp *real_mlt_profile_scale_width(%rip)
.size mlt_profile_scale_width, . - mlt_profile_scale_width

.global mlt_properties_anim_get
.type mlt_properties_anim_get, @function
mlt_properties_anim_get:
    jmp *real_mlt_properties_anim_get(%rip)
.size mlt_properties_anim_get, . - mlt_properties_anim_get

.global mlt_properties_anim_get_color
.type mlt_properties_anim_get_color, @function
mlt_properties_anim_get_color:
    jmp *real_mlt_properties_anim_get_color(%rip)
.size mlt_properties_anim_get_color, . - mlt_properties_anim_get_color

.global mlt_properties_anim_get_double
.type mlt_properties_anim_get_double, @function
mlt_properties_anim_get_double:
    jmp *real_mlt_properties_anim_get_double(%rip)
.size mlt_properties_anim_get_double, . - mlt_properties_anim_get_double

.global mlt_properties_anim_get_int
.type mlt_properties_anim_get_int, @function
mlt_properties_anim_get_int:
    jmp *real_mlt_properties_anim_get_int(%rip)
.size mlt_properties_anim_get_int, . - mlt_properties_anim_get_int

.global mlt_properties_anim_get_rect
.type mlt_properties_anim_get_rect, @function
mlt_properties_anim_get_rect:
    jmp *real_mlt_properties_anim_get_rect(%rip)
.size mlt_properties_anim_get_rect, . - mlt_properties_anim_get_rect

.global mlt_properties_anim_set
.type mlt_properties_anim_set, @function
mlt_properties_anim_set:
    jmp *real_mlt_properties_anim_set(%rip)
.size mlt_properties_anim_set, . - mlt_properties_anim_set

.global mlt_properties_anim_set_color
.type mlt_properties_anim_set_color, @function
mlt_properties_anim_set_color:
    jmp *real_mlt_properties_anim_set_color(%rip)
.size mlt_properties_anim_set_color, . - mlt_properties_anim_set_color

.global mlt_properties_anim_set_double
.type mlt_properties_anim_set_double, @function
mlt_properties_anim_set_double:
    jmp *real_mlt_properties_anim_set_double(%rip)
.size mlt_properties_anim_set_double, . - mlt_properties_anim_set_double

.global mlt_properties_anim_set_int
.type mlt_properties_anim_set_int, @function
mlt_properties_anim_set_int:
    jmp *real_mlt_properties_anim_set_int(%rip)
.size mlt_properties_anim_set_int, . - mlt_properties_anim_set_int

.global mlt_properties_anim_set_rect
.type mlt_properties_anim_set_rect, @function
mlt_properties_anim_set_rect:
    jmp *real_mlt_properties_anim_set_rect(%rip)
.size mlt_properties_anim_set_rect, . - mlt_properties_anim_set_rect

.global mlt_properties_clear
.type mlt_properties_clear, @function
mlt_properties_clear:
    jmp *real_mlt_properties_clear(%rip)
.size mlt_properties_clear, . - mlt_properties_clear

.global mlt_properties_close
.type mlt_properties_close, @function
mlt_properties_close:
    jmp *real_mlt_properties_close(%rip)
.size mlt_properties_close, . - mlt_properties_close

.global mlt_properties_copy
.type mlt_properties_copy, @function
mlt_properties_copy:
    jmp *real_mlt_properties_copy(%rip)
.size mlt_properties_copy, . - mlt_properties_copy

.global mlt_properties_count
.type mlt_properties_count, @function
mlt_properties_count:
    jmp *real_mlt_properties_count(%rip)
.size mlt_properties_count, . - mlt_properties_count

.global mlt_properties_debug
.type mlt_properties_debug, @function
mlt_properties_debug:
    jmp *real_mlt_properties_debug(%rip)
.size mlt_properties_debug, . - mlt_properties_debug

.global mlt_properties_dec_ref
.type mlt_properties_dec_ref, @function
mlt_properties_dec_ref:
    jmp *real_mlt_properties_dec_ref(%rip)
.size mlt_properties_dec_ref, . - mlt_properties_dec_ref

.global mlt_properties_dir_list
.type mlt_properties_dir_list, @function
mlt_properties_dir_list:
    jmp *real_mlt_properties_dir_list(%rip)
.size mlt_properties_dir_list, . - mlt_properties_dir_list

.global mlt_properties_dump
.type mlt_properties_dump, @function
mlt_properties_dump:
    jmp *real_mlt_properties_dump(%rip)
.size mlt_properties_dump, . - mlt_properties_dump

.global mlt_properties_exists
.type mlt_properties_exists, @function
mlt_properties_exists:
    jmp *real_mlt_properties_exists(%rip)
.size mlt_properties_exists, . - mlt_properties_exists

.global mlt_properties_frames_to_time
.type mlt_properties_frames_to_time, @function
mlt_properties_frames_to_time:
    jmp *real_mlt_properties_frames_to_time(%rip)
.size mlt_properties_frames_to_time, . - mlt_properties_frames_to_time

.global mlt_properties_from_utf8
.type mlt_properties_from_utf8, @function
mlt_properties_from_utf8:
    jmp *real_mlt_properties_from_utf8(%rip)
.size mlt_properties_from_utf8, . - mlt_properties_from_utf8

.global mlt_properties_get
.type mlt_properties_get, @function
mlt_properties_get:
    jmp *real_mlt_properties_get(%rip)
.size mlt_properties_get, . - mlt_properties_get

.global mlt_properties_get_animation
.type mlt_properties_get_animation, @function
mlt_properties_get_animation:
    jmp *real_mlt_properties_get_animation(%rip)
.size mlt_properties_get_animation, . - mlt_properties_get_animation

.global mlt_properties_get_color
.type mlt_properties_get_color, @function
mlt_properties_get_color:
    jmp *real_mlt_properties_get_color(%rip)
.size mlt_properties_get_color, . - mlt_properties_get_color

.global mlt_properties_get_data
.type mlt_properties_get_data, @function
mlt_properties_get_data:
    jmp *real_mlt_properties_get_data(%rip)
.size mlt_properties_get_data, . - mlt_properties_get_data

.global mlt_properties_get_data_at
.type mlt_properties_get_data_at, @function
mlt_properties_get_data_at:
    jmp *real_mlt_properties_get_data_at(%rip)
.size mlt_properties_get_data_at, . - mlt_properties_get_data_at

.global mlt_properties_get_double
.type mlt_properties_get_double, @function
mlt_properties_get_double:
    jmp *real_mlt_properties_get_double(%rip)
.size mlt_properties_get_double, . - mlt_properties_get_double

.global mlt_properties_get_int
.type mlt_properties_get_int, @function
mlt_properties_get_int:
    jmp *real_mlt_properties_get_int(%rip)
.size mlt_properties_get_int, . - mlt_properties_get_int

.global mlt_properties_get_int64
.type mlt_properties_get_int64, @function
mlt_properties_get_int64:
    jmp *real_mlt_properties_get_int64(%rip)
.size mlt_properties_get_int64, . - mlt_properties_get_int64

.global mlt_properties_get_lcnumeric
.type mlt_properties_get_lcnumeric, @function
mlt_properties_get_lcnumeric:
    jmp *real_mlt_properties_get_lcnumeric(%rip)
.size mlt_properties_get_lcnumeric, . - mlt_properties_get_lcnumeric

.global mlt_properties_get_name
.type mlt_properties_get_name, @function
mlt_properties_get_name:
    jmp *real_mlt_properties_get_name(%rip)
.size mlt_properties_get_name, . - mlt_properties_get_name

.global mlt_properties_get_position
.type mlt_properties_get_position, @function
mlt_properties_get_position:
    jmp *real_mlt_properties_get_position(%rip)
.size mlt_properties_get_position, . - mlt_properties_get_position

.global mlt_properties_get_properties
.type mlt_properties_get_properties, @function
mlt_properties_get_properties:
    jmp *real_mlt_properties_get_properties(%rip)
.size mlt_properties_get_properties, . - mlt_properties_get_properties

.global mlt_properties_get_properties_at
.type mlt_properties_get_properties_at, @function
mlt_properties_get_properties_at:
    jmp *real_mlt_properties_get_properties_at(%rip)
.size mlt_properties_get_properties_at, . - mlt_properties_get_properties_at

.global mlt_properties_get_rect
.type mlt_properties_get_rect, @function
mlt_properties_get_rect:
    jmp *real_mlt_properties_get_rect(%rip)
.size mlt_properties_get_rect, . - mlt_properties_get_rect

.global mlt_properties_get_time
.type mlt_properties_get_time, @function
mlt_properties_get_time:
    jmp *real_mlt_properties_get_time(%rip)
.size mlt_properties_get_time, . - mlt_properties_get_time

.global mlt_properties_get_value
.type mlt_properties_get_value, @function
mlt_properties_get_value:
    jmp *real_mlt_properties_get_value(%rip)
.size mlt_properties_get_value, . - mlt_properties_get_value

.global mlt_properties_get_value_tf
.type mlt_properties_get_value_tf, @function
mlt_properties_get_value_tf:
    jmp *real_mlt_properties_get_value_tf(%rip)
.size mlt_properties_get_value_tf, . - mlt_properties_get_value_tf

.global mlt_properties_inc_ref
.type mlt_properties_inc_ref, @function
mlt_properties_inc_ref:
    jmp *real_mlt_properties_inc_ref(%rip)
.size mlt_properties_inc_ref, . - mlt_properties_inc_ref

.global mlt_properties_inherit
.type mlt_properties_inherit, @function
mlt_properties_inherit:
    jmp *real_mlt_properties_inherit(%rip)
.size mlt_properties_inherit, . - mlt_properties_inherit

.global mlt_properties_init
.type mlt_properties_init, @function
mlt_properties_init:
    jmp *real_mlt_properties_init(%rip)
.size mlt_properties_init, . - mlt_properties_init

.global mlt_properties_is_anim
.type mlt_properties_is_anim, @function
mlt_properties_is_anim:
    jmp *real_mlt_properties_is_anim(%rip)
.size mlt_properties_is_anim, . - mlt_properties_is_anim

.global mlt_properties_is_sequence
.type mlt_properties_is_sequence, @function
mlt_properties_is_sequence:
    jmp *real_mlt_properties_is_sequence(%rip)
.size mlt_properties_is_sequence, . - mlt_properties_is_sequence

.global mlt_properties_load
.type mlt_properties_load, @function
mlt_properties_load:
    jmp *real_mlt_properties_load(%rip)
.size mlt_properties_load, . - mlt_properties_load

.global mlt_properties_lock
.type mlt_properties_lock, @function
mlt_properties_lock:
    jmp *real_mlt_properties_lock(%rip)
.size mlt_properties_lock, . - mlt_properties_lock

.global mlt_properties_mirror
.type mlt_properties_mirror, @function
mlt_properties_mirror:
    jmp *real_mlt_properties_mirror(%rip)
.size mlt_properties_mirror, . - mlt_properties_mirror

.global mlt_properties_new
.type mlt_properties_new, @function
mlt_properties_new:
    jmp *real_mlt_properties_new(%rip)
.size mlt_properties_new, . - mlt_properties_new

.global mlt_properties_parse
.type mlt_properties_parse, @function
mlt_properties_parse:
    jmp *real_mlt_properties_parse(%rip)
.size mlt_properties_parse, . - mlt_properties_parse

.global mlt_properties_parse_yaml
.type mlt_properties_parse_yaml, @function
mlt_properties_parse_yaml:
    jmp *real_mlt_properties_parse_yaml(%rip)
.size mlt_properties_parse_yaml, . - mlt_properties_parse_yaml

.global mlt_properties_pass
.type mlt_properties_pass, @function
mlt_properties_pass:
    jmp *real_mlt_properties_pass(%rip)
.size mlt_properties_pass, . - mlt_properties_pass

.global mlt_properties_pass_list
.type mlt_properties_pass_list, @function
mlt_properties_pass_list:
    jmp *real_mlt_properties_pass_list(%rip)
.size mlt_properties_pass_list, . - mlt_properties_pass_list

.global mlt_properties_pass_property
.type mlt_properties_pass_property, @function
mlt_properties_pass_property:
    jmp *real_mlt_properties_pass_property(%rip)
.size mlt_properties_pass_property, . - mlt_properties_pass_property

.global mlt_properties_preset
.type mlt_properties_preset, @function
mlt_properties_preset:
    jmp *real_mlt_properties_preset(%rip)
.size mlt_properties_preset, . - mlt_properties_preset

.global mlt_properties_ref_count
.type mlt_properties_ref_count, @function
mlt_properties_ref_count:
    jmp *real_mlt_properties_ref_count(%rip)
.size mlt_properties_ref_count, . - mlt_properties_ref_count

.global mlt_properties_rename
.type mlt_properties_rename, @function
mlt_properties_rename:
    jmp *real_mlt_properties_rename(%rip)
.size mlt_properties_rename, . - mlt_properties_rename

.global mlt_properties_save
.type mlt_properties_save, @function
mlt_properties_save:
    jmp *real_mlt_properties_save(%rip)
.size mlt_properties_save, . - mlt_properties_save

.global mlt_properties_serialise_yaml
.type mlt_properties_serialise_yaml, @function
mlt_properties_serialise_yaml:
    jmp *real_mlt_properties_serialise_yaml(%rip)
.size mlt_properties_serialise_yaml, . - mlt_properties_serialise_yaml

.global mlt_properties_set
.type mlt_properties_set, @function
mlt_properties_set:
    jmp *real_mlt_properties_set(%rip)
.size mlt_properties_set, . - mlt_properties_set

.global mlt_properties_set_color
.type mlt_properties_set_color, @function
mlt_properties_set_color:
    jmp *real_mlt_properties_set_color(%rip)
.size mlt_properties_set_color, . - mlt_properties_set_color

.global mlt_properties_set_data
.type mlt_properties_set_data, @function
mlt_properties_set_data:
    jmp *real_mlt_properties_set_data(%rip)
.size mlt_properties_set_data, . - mlt_properties_set_data

.global mlt_properties_set_double
.type mlt_properties_set_double, @function
mlt_properties_set_double:
    jmp *real_mlt_properties_set_double(%rip)
.size mlt_properties_set_double, . - mlt_properties_set_double

.global mlt_properties_set_int
.type mlt_properties_set_int, @function
mlt_properties_set_int:
    jmp *real_mlt_properties_set_int(%rip)
.size mlt_properties_set_int, . - mlt_properties_set_int

.global mlt_properties_set_int64
.type mlt_properties_set_int64, @function
mlt_properties_set_int64:
    jmp *real_mlt_properties_set_int64(%rip)
.size mlt_properties_set_int64, . - mlt_properties_set_int64

.global mlt_properties_set_lcnumeric
.type mlt_properties_set_lcnumeric, @function
mlt_properties_set_lcnumeric:
    jmp *real_mlt_properties_set_lcnumeric(%rip)
.size mlt_properties_set_lcnumeric, . - mlt_properties_set_lcnumeric

.global mlt_properties_set_or_default
.type mlt_properties_set_or_default, @function
mlt_properties_set_or_default:
    jmp *real_mlt_properties_set_or_default(%rip)
.size mlt_properties_set_or_default, . - mlt_properties_set_or_default

.global mlt_properties_set_position
.type mlt_properties_set_position, @function
mlt_properties_set_position:
    jmp *real_mlt_properties_set_position(%rip)
.size mlt_properties_set_position, . - mlt_properties_set_position

.global mlt_properties_set_properties
.type mlt_properties_set_properties, @function
mlt_properties_set_properties:
    jmp *real_mlt_properties_set_properties(%rip)
.size mlt_properties_set_properties, . - mlt_properties_set_properties

.global mlt_properties_set_rect
.type mlt_properties_set_rect, @function
mlt_properties_set_rect:
    jmp *real_mlt_properties_set_rect(%rip)
.size mlt_properties_set_rect, . - mlt_properties_set_rect

.global mlt_properties_set_string
.type mlt_properties_set_string, @function
mlt_properties_set_string:
    jmp *real_mlt_properties_set_string(%rip)
.size mlt_properties_set_string, . - mlt_properties_set_string

.global mlt_properties_time_to_frames
.type mlt_properties_time_to_frames, @function
mlt_properties_time_to_frames:
    jmp *real_mlt_properties_time_to_frames(%rip)
.size mlt_properties_time_to_frames, . - mlt_properties_time_to_frames

.global mlt_properties_unlock
.type mlt_properties_unlock, @function
mlt_properties_unlock:
    jmp *real_mlt_properties_unlock(%rip)
.size mlt_properties_unlock, . - mlt_properties_unlock

.global mlt_property_anim_get_color
.type mlt_property_anim_get_color, @function
mlt_property_anim_get_color:
    jmp *real_mlt_property_anim_get_color(%rip)
.size mlt_property_anim_get_color, . - mlt_property_anim_get_color

.global mlt_property_anim_get_double
.type mlt_property_anim_get_double, @function
mlt_property_anim_get_double:
    jmp *real_mlt_property_anim_get_double(%rip)
.size mlt_property_anim_get_double, . - mlt_property_anim_get_double

.global mlt_property_anim_get_int
.type mlt_property_anim_get_int, @function
mlt_property_anim_get_int:
    jmp *real_mlt_property_anim_get_int(%rip)
.size mlt_property_anim_get_int, . - mlt_property_anim_get_int

.global mlt_property_anim_get_rect
.type mlt_property_anim_get_rect, @function
mlt_property_anim_get_rect:
    jmp *real_mlt_property_anim_get_rect(%rip)
.size mlt_property_anim_get_rect, . - mlt_property_anim_get_rect

.global mlt_property_anim_get_string
.type mlt_property_anim_get_string, @function
mlt_property_anim_get_string:
    jmp *real_mlt_property_anim_get_string(%rip)
.size mlt_property_anim_get_string, . - mlt_property_anim_get_string

.global mlt_property_anim_set_color
.type mlt_property_anim_set_color, @function
mlt_property_anim_set_color:
    jmp *real_mlt_property_anim_set_color(%rip)
.size mlt_property_anim_set_color, . - mlt_property_anim_set_color

.global mlt_property_anim_set_double
.type mlt_property_anim_set_double, @function
mlt_property_anim_set_double:
    jmp *real_mlt_property_anim_set_double(%rip)
.size mlt_property_anim_set_double, . - mlt_property_anim_set_double

.global mlt_property_anim_set_int
.type mlt_property_anim_set_int, @function
mlt_property_anim_set_int:
    jmp *real_mlt_property_anim_set_int(%rip)
.size mlt_property_anim_set_int, . - mlt_property_anim_set_int

.global mlt_property_anim_set_rect
.type mlt_property_anim_set_rect, @function
mlt_property_anim_set_rect:
    jmp *real_mlt_property_anim_set_rect(%rip)
.size mlt_property_anim_set_rect, . - mlt_property_anim_set_rect

.global mlt_property_anim_set_string
.type mlt_property_anim_set_string, @function
mlt_property_anim_set_string:
    jmp *real_mlt_property_anim_set_string(%rip)
.size mlt_property_anim_set_string, . - mlt_property_anim_set_string

.global mlt_property_clear
.type mlt_property_clear, @function
mlt_property_clear:
    jmp *real_mlt_property_clear(%rip)
.size mlt_property_clear, . - mlt_property_clear

.global mlt_property_close
.type mlt_property_close, @function
mlt_property_close:
    jmp *real_mlt_property_close(%rip)
.size mlt_property_close, . - mlt_property_close

.global mlt_property_get_animation
.type mlt_property_get_animation, @function
mlt_property_get_animation:
    jmp *real_mlt_property_get_animation(%rip)
.size mlt_property_get_animation, . - mlt_property_get_animation

.global mlt_property_get_color
.type mlt_property_get_color, @function
mlt_property_get_color:
    jmp *real_mlt_property_get_color(%rip)
.size mlt_property_get_color, . - mlt_property_get_color

.global mlt_property_get_data
.type mlt_property_get_data, @function
mlt_property_get_data:
    jmp *real_mlt_property_get_data(%rip)
.size mlt_property_get_data, . - mlt_property_get_data

.global mlt_property_get_double
.type mlt_property_get_double, @function
mlt_property_get_double:
    jmp *real_mlt_property_get_double(%rip)
.size mlt_property_get_double, . - mlt_property_get_double

.global mlt_property_get_int
.type mlt_property_get_int, @function
mlt_property_get_int:
    jmp *real_mlt_property_get_int(%rip)
.size mlt_property_get_int, . - mlt_property_get_int

.global mlt_property_get_int64
.type mlt_property_get_int64, @function
mlt_property_get_int64:
    jmp *real_mlt_property_get_int64(%rip)
.size mlt_property_get_int64, . - mlt_property_get_int64

.global mlt_property_get_position
.type mlt_property_get_position, @function
mlt_property_get_position:
    jmp *real_mlt_property_get_position(%rip)
.size mlt_property_get_position, . - mlt_property_get_position

.global mlt_property_get_properties
.type mlt_property_get_properties, @function
mlt_property_get_properties:
    jmp *real_mlt_property_get_properties(%rip)
.size mlt_property_get_properties, . - mlt_property_get_properties

.global mlt_property_get_rect
.type mlt_property_get_rect, @function
mlt_property_get_rect:
    jmp *real_mlt_property_get_rect(%rip)
.size mlt_property_get_rect, . - mlt_property_get_rect

.global mlt_property_get_string
.type mlt_property_get_string, @function
mlt_property_get_string:
    jmp *real_mlt_property_get_string(%rip)
.size mlt_property_get_string, . - mlt_property_get_string

.global mlt_property_get_string_l
.type mlt_property_get_string_l, @function
mlt_property_get_string_l:
    jmp *real_mlt_property_get_string_l(%rip)
.size mlt_property_get_string_l, . - mlt_property_get_string_l

.global mlt_property_get_string_l_tf
.type mlt_property_get_string_l_tf, @function
mlt_property_get_string_l_tf:
    jmp *real_mlt_property_get_string_l_tf(%rip)
.size mlt_property_get_string_l_tf, . - mlt_property_get_string_l_tf

.global mlt_property_get_string_tf
.type mlt_property_get_string_tf, @function
mlt_property_get_string_tf:
    jmp *real_mlt_property_get_string_tf(%rip)
.size mlt_property_get_string_tf, . - mlt_property_get_string_tf

.global mlt_property_get_time
.type mlt_property_get_time, @function
mlt_property_get_time:
    jmp *real_mlt_property_get_time(%rip)
.size mlt_property_get_time, . - mlt_property_get_time

.global mlt_property_init
.type mlt_property_init, @function
mlt_property_init:
    jmp *real_mlt_property_init(%rip)
.size mlt_property_init, . - mlt_property_init

.global mlt_property_interpolate
.type mlt_property_interpolate, @function
mlt_property_interpolate:
    jmp *real_mlt_property_interpolate(%rip)
.size mlt_property_interpolate, . - mlt_property_interpolate

.global mlt_property_is_anim
.type mlt_property_is_anim, @function
mlt_property_is_anim:
    jmp *real_mlt_property_is_anim(%rip)
.size mlt_property_is_anim, . - mlt_property_is_anim

.global mlt_property_is_clear
.type mlt_property_is_clear, @function
mlt_property_is_clear:
    jmp *real_mlt_property_is_clear(%rip)
.size mlt_property_is_clear, . - mlt_property_is_clear

.global mlt_property_is_color
.type mlt_property_is_color, @function
mlt_property_is_color:
    jmp *real_mlt_property_is_color(%rip)
.size mlt_property_is_color, . - mlt_property_is_color

.global mlt_property_is_numeric
.type mlt_property_is_numeric, @function
mlt_property_is_numeric:
    jmp *real_mlt_property_is_numeric(%rip)
.size mlt_property_is_numeric, . - mlt_property_is_numeric

.global mlt_property_is_rect
.type mlt_property_is_rect, @function
mlt_property_is_rect:
    jmp *real_mlt_property_is_rect(%rip)
.size mlt_property_is_rect, . - mlt_property_is_rect

.global mlt_property_pass
.type mlt_property_pass, @function
mlt_property_pass:
    jmp *real_mlt_property_pass(%rip)
.size mlt_property_pass, . - mlt_property_pass

.global mlt_property_set_color
.type mlt_property_set_color, @function
mlt_property_set_color:
    jmp *real_mlt_property_set_color(%rip)
.size mlt_property_set_color, . - mlt_property_set_color

.global mlt_property_set_data
.type mlt_property_set_data, @function
mlt_property_set_data:
    jmp *real_mlt_property_set_data(%rip)
.size mlt_property_set_data, . - mlt_property_set_data

.global mlt_property_set_double
.type mlt_property_set_double, @function
mlt_property_set_double:
    jmp *real_mlt_property_set_double(%rip)
.size mlt_property_set_double, . - mlt_property_set_double

.global mlt_property_set_int
.type mlt_property_set_int, @function
mlt_property_set_int:
    jmp *real_mlt_property_set_int(%rip)
.size mlt_property_set_int, . - mlt_property_set_int

.global mlt_property_set_int64
.type mlt_property_set_int64, @function
mlt_property_set_int64:
    jmp *real_mlt_property_set_int64(%rip)
.size mlt_property_set_int64, . - mlt_property_set_int64

.global mlt_property_set_position
.type mlt_property_set_position, @function
mlt_property_set_position:
    jmp *real_mlt_property_set_position(%rip)
.size mlt_property_set_position, . - mlt_property_set_position

.global mlt_property_set_properties
.type mlt_property_set_properties, @function
mlt_property_set_properties:
    jmp *real_mlt_property_set_properties(%rip)
.size mlt_property_set_properties, . - mlt_property_set_properties

.global mlt_property_set_rect
.type mlt_property_set_rect, @function
mlt_property_set_rect:
    jmp *real_mlt_property_set_rect(%rip)
.size mlt_property_set_rect, . - mlt_property_set_rect

.global mlt_property_set_string
.type mlt_property_set_string, @function
mlt_property_set_string:
    jmp *real_mlt_property_set_string(%rip)
.size mlt_property_set_string, . - mlt_property_set_string

.global mlt_repository_close
.type mlt_repository_close, @function
mlt_repository_close:
    jmp *real_mlt_repository_close(%rip)
.size mlt_repository_close, . - mlt_repository_close

.global mlt_repository_consumers
.type mlt_repository_consumers, @function
mlt_repository_consumers:
    jmp *real_mlt_repository_consumers(%rip)
.size mlt_repository_consumers, . - mlt_repository_consumers

.global mlt_repository_create
.type mlt_repository_create, @function
mlt_repository_create:
    jmp *real_mlt_repository_create(%rip)
.size mlt_repository_create, . - mlt_repository_create

.global mlt_repository_filters
.type mlt_repository_filters, @function
mlt_repository_filters:
    jmp *real_mlt_repository_filters(%rip)
.size mlt_repository_filters, . - mlt_repository_filters

.global mlt_repository_init
.type mlt_repository_init, @function
mlt_repository_init:
    jmp *real_mlt_repository_init(%rip)
.size mlt_repository_init, . - mlt_repository_init

.global mlt_repository_languages
.type mlt_repository_languages, @function
mlt_repository_languages:
    jmp *real_mlt_repository_languages(%rip)
.size mlt_repository_languages, . - mlt_repository_languages

.global mlt_repository_links
.type mlt_repository_links, @function
mlt_repository_links:
    jmp *real_mlt_repository_links(%rip)
.size mlt_repository_links, . - mlt_repository_links

.global mlt_repository_metadata
.type mlt_repository_metadata, @function
mlt_repository_metadata:
    jmp *real_mlt_repository_metadata(%rip)
.size mlt_repository_metadata, . - mlt_repository_metadata

.global mlt_repository_presets
.type mlt_repository_presets, @function
mlt_repository_presets:
    jmp *real_mlt_repository_presets(%rip)
.size mlt_repository_presets, . - mlt_repository_presets

.global mlt_repository_producers
.type mlt_repository_producers, @function
mlt_repository_producers:
    jmp *real_mlt_repository_producers(%rip)
.size mlt_repository_producers, . - mlt_repository_producers

.global mlt_repository_register
.type mlt_repository_register, @function
mlt_repository_register:
    jmp *real_mlt_repository_register(%rip)
.size mlt_repository_register, . - mlt_repository_register

.global mlt_repository_register_metadata
.type mlt_repository_register_metadata, @function
mlt_repository_register_metadata:
    jmp *real_mlt_repository_register_metadata(%rip)
.size mlt_repository_register_metadata, . - mlt_repository_register_metadata

.global mlt_repository_transitions
.type mlt_repository_transitions, @function
mlt_repository_transitions:
    jmp *real_mlt_repository_transitions(%rip)
.size mlt_repository_transitions, . - mlt_repository_transitions

.global mlt_service_apply_filters
.type mlt_service_apply_filters, @function
mlt_service_apply_filters:
    jmp *real_mlt_service_apply_filters(%rip)
.size mlt_service_apply_filters, . - mlt_service_apply_filters

.global mlt_service_attach
.type mlt_service_attach, @function
mlt_service_attach:
    jmp *real_mlt_service_attach(%rip)
.size mlt_service_attach, . - mlt_service_attach

.global mlt_service_cache_get
.type mlt_service_cache_get, @function
mlt_service_cache_get:
    jmp *real_mlt_service_cache_get(%rip)
.size mlt_service_cache_get, . - mlt_service_cache_get

.global mlt_service_cache_get_size
.type mlt_service_cache_get_size, @function
mlt_service_cache_get_size:
    jmp *real_mlt_service_cache_get_size(%rip)
.size mlt_service_cache_get_size, . - mlt_service_cache_get_size

.global mlt_service_cache_purge
.type mlt_service_cache_purge, @function
mlt_service_cache_purge:
    jmp *real_mlt_service_cache_purge(%rip)
.size mlt_service_cache_purge, . - mlt_service_cache_purge

.global mlt_service_cache_put
.type mlt_service_cache_put, @function
mlt_service_cache_put:
    jmp *real_mlt_service_cache_put(%rip)
.size mlt_service_cache_put, . - mlt_service_cache_put

.global mlt_service_cache_set_size
.type mlt_service_cache_set_size, @function
mlt_service_cache_set_size:
    jmp *real_mlt_service_cache_set_size(%rip)
.size mlt_service_cache_set_size, . - mlt_service_cache_set_size

.global mlt_service_close
.type mlt_service_close, @function
mlt_service_close:
    jmp *real_mlt_service_close(%rip)
.size mlt_service_close, . - mlt_service_close

.global mlt_service_connect_producer
.type mlt_service_connect_producer, @function
mlt_service_connect_producer:
    jmp *real_mlt_service_connect_producer(%rip)
.size mlt_service_connect_producer, . - mlt_service_connect_producer

.global mlt_service_consumer
.type mlt_service_consumer, @function
mlt_service_consumer:
    jmp *real_mlt_service_consumer(%rip)
.size mlt_service_consumer, . - mlt_service_consumer

.global mlt_service_detach
.type mlt_service_detach, @function
mlt_service_detach:
    jmp *real_mlt_service_detach(%rip)
.size mlt_service_detach, . - mlt_service_detach

.global mlt_service_disconnect_all_producers
.type mlt_service_disconnect_all_producers, @function
mlt_service_disconnect_all_producers:
    jmp *real_mlt_service_disconnect_all_producers(%rip)
.size mlt_service_disconnect_all_producers, . - mlt_service_disconnect_all_producers

.global mlt_service_disconnect_producer
.type mlt_service_disconnect_producer, @function
mlt_service_disconnect_producer:
    jmp *real_mlt_service_disconnect_producer(%rip)
.size mlt_service_disconnect_producer, . - mlt_service_disconnect_producer

.global mlt_service_filter
.type mlt_service_filter, @function
mlt_service_filter:
    jmp *real_mlt_service_filter(%rip)
.size mlt_service_filter, . - mlt_service_filter

.global mlt_service_filter_count
.type mlt_service_filter_count, @function
mlt_service_filter_count:
    jmp *real_mlt_service_filter_count(%rip)
.size mlt_service_filter_count, . - mlt_service_filter_count

.global mlt_service_get_frame
.type mlt_service_get_frame, @function
mlt_service_get_frame:
    jmp *real_mlt_service_get_frame(%rip)
.size mlt_service_get_frame, . - mlt_service_get_frame

.global mlt_service_get_producer
.type mlt_service_get_producer, @function
mlt_service_get_producer:
    jmp *real_mlt_service_get_producer(%rip)
.size mlt_service_get_producer, . - mlt_service_get_producer

.global mlt_service_identify
.type mlt_service_identify, @function
mlt_service_identify:
    jmp *real_mlt_service_identify(%rip)
.size mlt_service_identify, . - mlt_service_identify

.global mlt_service_init
.type mlt_service_init, @function
mlt_service_init:
    jmp *real_mlt_service_init(%rip)
.size mlt_service_init, . - mlt_service_init

.global mlt_service_insert_producer
.type mlt_service_insert_producer, @function
mlt_service_insert_producer:
    jmp *real_mlt_service_insert_producer(%rip)
.size mlt_service_insert_producer, . - mlt_service_insert_producer

.global mlt_service_lock
.type mlt_service_lock, @function
mlt_service_lock:
    jmp *real_mlt_service_lock(%rip)
.size mlt_service_lock, . - mlt_service_lock

.global mlt_service_move_filter
.type mlt_service_move_filter, @function
mlt_service_move_filter:
    jmp *real_mlt_service_move_filter(%rip)
.size mlt_service_move_filter, . - mlt_service_move_filter

.global mlt_service_producer
.type mlt_service_producer, @function
mlt_service_producer:
    jmp *real_mlt_service_producer(%rip)
.size mlt_service_producer, . - mlt_service_producer

.global mlt_service_profile
.type mlt_service_profile, @function
mlt_service_profile:
    jmp *real_mlt_service_profile(%rip)
.size mlt_service_profile, . - mlt_service_profile

.global mlt_service_properties
.type mlt_service_properties, @function
mlt_service_properties:
    jmp *real_mlt_service_properties(%rip)
.size mlt_service_properties, . - mlt_service_properties

.global mlt_service_set_consumer
.type mlt_service_set_consumer, @function
mlt_service_set_consumer:
    jmp *real_mlt_service_set_consumer(%rip)
.size mlt_service_set_consumer, . - mlt_service_set_consumer

.global mlt_service_set_profile
.type mlt_service_set_profile, @function
mlt_service_set_profile:
    jmp *real_mlt_service_set_profile(%rip)
.size mlt_service_set_profile, . - mlt_service_set_profile

.global mlt_service_unlock
.type mlt_service_unlock, @function
mlt_service_unlock:
    jmp *real_mlt_service_unlock(%rip)
.size mlt_service_unlock, . - mlt_service_unlock

.global mlt_slices_count_fifo
.type mlt_slices_count_fifo, @function
mlt_slices_count_fifo:
    jmp *real_mlt_slices_count_fifo(%rip)
.size mlt_slices_count_fifo, . - mlt_slices_count_fifo

.global mlt_slices_count_normal
.type mlt_slices_count_normal, @function
mlt_slices_count_normal:
    jmp *real_mlt_slices_count_normal(%rip)
.size mlt_slices_count_normal, . - mlt_slices_count_normal

.global mlt_slices_count_rr
.type mlt_slices_count_rr, @function
mlt_slices_count_rr:
    jmp *real_mlt_slices_count_rr(%rip)
.size mlt_slices_count_rr, . - mlt_slices_count_rr

.global mlt_slices_run_fifo
.type mlt_slices_run_fifo, @function
mlt_slices_run_fifo:
    jmp *real_mlt_slices_run_fifo(%rip)
.size mlt_slices_run_fifo, . - mlt_slices_run_fifo

.global mlt_slices_run_normal
.type mlt_slices_run_normal, @function
mlt_slices_run_normal:
    jmp *real_mlt_slices_run_normal(%rip)
.size mlt_slices_run_normal, . - mlt_slices_run_normal

.global mlt_slices_run_rr
.type mlt_slices_run_rr, @function
mlt_slices_run_rr:
    jmp *real_mlt_slices_run_rr(%rip)
.size mlt_slices_run_rr, . - mlt_slices_run_rr

.global mlt_slices_size_slice
.type mlt_slices_size_slice, @function
mlt_slices_size_slice:
    jmp *real_mlt_slices_size_slice(%rip)
.size mlt_slices_size_slice, . - mlt_slices_size_slice

.global mlt_tokeniser_close
.type mlt_tokeniser_close, @function
mlt_tokeniser_close:
    jmp *real_mlt_tokeniser_close(%rip)
.size mlt_tokeniser_close, . - mlt_tokeniser_close

.global mlt_tokeniser_count
.type mlt_tokeniser_count, @function
mlt_tokeniser_count:
    jmp *real_mlt_tokeniser_count(%rip)
.size mlt_tokeniser_count, . - mlt_tokeniser_count

.global mlt_tokeniser_get_input
.type mlt_tokeniser_get_input, @function
mlt_tokeniser_get_input:
    jmp *real_mlt_tokeniser_get_input(%rip)
.size mlt_tokeniser_get_input, . - mlt_tokeniser_get_input

.global mlt_tokeniser_get_string
.type mlt_tokeniser_get_string, @function
mlt_tokeniser_get_string:
    jmp *real_mlt_tokeniser_get_string(%rip)
.size mlt_tokeniser_get_string, . - mlt_tokeniser_get_string

.global mlt_tokeniser_init
.type mlt_tokeniser_init, @function
mlt_tokeniser_init:
    jmp *real_mlt_tokeniser_init(%rip)
.size mlt_tokeniser_init, . - mlt_tokeniser_init

.global mlt_tokeniser_parse_new
.type mlt_tokeniser_parse_new, @function
mlt_tokeniser_parse_new:
    jmp *real_mlt_tokeniser_parse_new(%rip)
.size mlt_tokeniser_parse_new, . - mlt_tokeniser_parse_new

.global mlt_tractor_close
.type mlt_tractor_close, @function
mlt_tractor_close:
    jmp *real_mlt_tractor_close(%rip)
.size mlt_tractor_close, . - mlt_tractor_close

.global mlt_tractor_connect
.type mlt_tractor_connect, @function
mlt_tractor_connect:
    jmp *real_mlt_tractor_connect(%rip)
.size mlt_tractor_connect, . - mlt_tractor_connect

.global mlt_tractor_field
.type mlt_tractor_field, @function
mlt_tractor_field:
    jmp *real_mlt_tractor_field(%rip)
.size mlt_tractor_field, . - mlt_tractor_field

.global mlt_tractor_get_track
.type mlt_tractor_get_track, @function
mlt_tractor_get_track:
    jmp *real_mlt_tractor_get_track(%rip)
.size mlt_tractor_get_track, . - mlt_tractor_get_track

.global mlt_tractor_init
.type mlt_tractor_init, @function
mlt_tractor_init:
    jmp *real_mlt_tractor_init(%rip)
.size mlt_tractor_init, . - mlt_tractor_init

.global mlt_tractor_insert_track
.type mlt_tractor_insert_track, @function
mlt_tractor_insert_track:
    jmp *real_mlt_tractor_insert_track(%rip)
.size mlt_tractor_insert_track, . - mlt_tractor_insert_track

.global mlt_tractor_multitrack
.type mlt_tractor_multitrack, @function
mlt_tractor_multitrack:
    jmp *real_mlt_tractor_multitrack(%rip)
.size mlt_tractor_multitrack, . - mlt_tractor_multitrack

.global mlt_tractor_new
.type mlt_tractor_new, @function
mlt_tractor_new:
    jmp *real_mlt_tractor_new(%rip)
.size mlt_tractor_new, . - mlt_tractor_new

.global mlt_tractor_producer
.type mlt_tractor_producer, @function
mlt_tractor_producer:
    jmp *real_mlt_tractor_producer(%rip)
.size mlt_tractor_producer, . - mlt_tractor_producer

.global mlt_tractor_properties
.type mlt_tractor_properties, @function
mlt_tractor_properties:
    jmp *real_mlt_tractor_properties(%rip)
.size mlt_tractor_properties, . - mlt_tractor_properties

.global mlt_tractor_refresh
.type mlt_tractor_refresh, @function
mlt_tractor_refresh:
    jmp *real_mlt_tractor_refresh(%rip)
.size mlt_tractor_refresh, . - mlt_tractor_refresh

.global mlt_tractor_remove_track
.type mlt_tractor_remove_track, @function
mlt_tractor_remove_track:
    jmp *real_mlt_tractor_remove_track(%rip)
.size mlt_tractor_remove_track, . - mlt_tractor_remove_track

.global mlt_tractor_service
.type mlt_tractor_service, @function
mlt_tractor_service:
    jmp *real_mlt_tractor_service(%rip)
.size mlt_tractor_service, . - mlt_tractor_service

.global mlt_tractor_set_track
.type mlt_tractor_set_track, @function
mlt_tractor_set_track:
    jmp *real_mlt_tractor_set_track(%rip)
.size mlt_tractor_set_track, . - mlt_tractor_set_track

.global mlt_transition_close
.type mlt_transition_close, @function
mlt_transition_close:
    jmp *real_mlt_transition_close(%rip)
.size mlt_transition_close, . - mlt_transition_close

.global mlt_transition_connect
.type mlt_transition_connect, @function
mlt_transition_connect:
    jmp *real_mlt_transition_connect(%rip)
.size mlt_transition_connect, . - mlt_transition_connect

.global mlt_transition_get_a_track
.type mlt_transition_get_a_track, @function
mlt_transition_get_a_track:
    jmp *real_mlt_transition_get_a_track(%rip)
.size mlt_transition_get_a_track, . - mlt_transition_get_a_track

.global mlt_transition_get_b_track
.type mlt_transition_get_b_track, @function
mlt_transition_get_b_track:
    jmp *real_mlt_transition_get_b_track(%rip)
.size mlt_transition_get_b_track, . - mlt_transition_get_b_track

.global mlt_transition_get_in
.type mlt_transition_get_in, @function
mlt_transition_get_in:
    jmp *real_mlt_transition_get_in(%rip)
.size mlt_transition_get_in, . - mlt_transition_get_in

.global mlt_transition_get_length
.type mlt_transition_get_length, @function
mlt_transition_get_length:
    jmp *real_mlt_transition_get_length(%rip)
.size mlt_transition_get_length, . - mlt_transition_get_length

.global mlt_transition_get_out
.type mlt_transition_get_out, @function
mlt_transition_get_out:
    jmp *real_mlt_transition_get_out(%rip)
.size mlt_transition_get_out, . - mlt_transition_get_out

.global mlt_transition_get_position
.type mlt_transition_get_position, @function
mlt_transition_get_position:
    jmp *real_mlt_transition_get_position(%rip)
.size mlt_transition_get_position, . - mlt_transition_get_position

.global mlt_transition_get_progress
.type mlt_transition_get_progress, @function
mlt_transition_get_progress:
    jmp *real_mlt_transition_get_progress(%rip)
.size mlt_transition_get_progress, . - mlt_transition_get_progress

.global mlt_transition_get_progress_delta
.type mlt_transition_get_progress_delta, @function
mlt_transition_get_progress_delta:
    jmp *real_mlt_transition_get_progress_delta(%rip)
.size mlt_transition_get_progress_delta, . - mlt_transition_get_progress_delta

.global mlt_transition_init
.type mlt_transition_init, @function
mlt_transition_init:
    jmp *real_mlt_transition_init(%rip)
.size mlt_transition_init, . - mlt_transition_init

.global mlt_transition_new
.type mlt_transition_new, @function
mlt_transition_new:
    jmp *real_mlt_transition_new(%rip)
.size mlt_transition_new, . - mlt_transition_new

.global mlt_transition_process
.type mlt_transition_process, @function
mlt_transition_process:
    jmp *real_mlt_transition_process(%rip)
.size mlt_transition_process, . - mlt_transition_process

.global mlt_transition_properties
.type mlt_transition_properties, @function
mlt_transition_properties:
    jmp *real_mlt_transition_properties(%rip)
.size mlt_transition_properties, . - mlt_transition_properties

.global mlt_transition_service
.type mlt_transition_service, @function
mlt_transition_service:
    jmp *real_mlt_transition_service(%rip)
.size mlt_transition_service, . - mlt_transition_service

.global mlt_transition_set_in_and_out
.type mlt_transition_set_in_and_out, @function
mlt_transition_set_in_and_out:
    jmp *real_mlt_transition_set_in_and_out(%rip)
.size mlt_transition_set_in_and_out, . - mlt_transition_set_in_and_out

.global mlt_transition_set_tracks
.type mlt_transition_set_tracks, @function
mlt_transition_set_tracks:
    jmp *real_mlt_transition_set_tracks(%rip)
.size mlt_transition_set_tracks, . - mlt_transition_set_tracks

.global mlt_version_get_int
.type mlt_version_get_int, @function
mlt_version_get_int:
    jmp *real_mlt_version_get_int(%rip)
.size mlt_version_get_int, . - mlt_version_get_int

.global mlt_version_get_major
.type mlt_version_get_major, @function
mlt_version_get_major:
    jmp *real_mlt_version_get_major(%rip)
.size mlt_version_get_major, . - mlt_version_get_major

.global mlt_version_get_minor
.type mlt_version_get_minor, @function
mlt_version_get_minor:
    jmp *real_mlt_version_get_minor(%rip)
.size mlt_version_get_minor, . - mlt_version_get_minor

.global mlt_version_get_revision
.type mlt_version_get_revision, @function
mlt_version_get_revision:
    jmp *real_mlt_version_get_revision(%rip)
.size mlt_version_get_revision, . - mlt_version_get_revision

.global mlt_version_get_string
.type mlt_version_get_string, @function
mlt_version_get_string:
    jmp *real_mlt_version_get_string(%rip)
.size mlt_version_get_string, . - mlt_version_get_string

.global mlt_vlog
.type mlt_vlog, @function
mlt_vlog:
    jmp *real_mlt_vlog(%rip)
.size mlt_vlog, . - mlt_vlog

