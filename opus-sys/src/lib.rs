#![allow(non_camel_case_types)]

extern crate libc;

pub type int8_t = ::libc::c_char;
pub type int16_t = ::libc::c_short;
pub type int32_t = ::libc::c_int;
pub type int64_t = ::libc::c_long;
pub type uint8_t = ::libc::c_uchar;
pub type uint16_t = ::libc::c_ushort;
pub type uint32_t = ::libc::c_uint;
pub type uint64_t = ::libc::c_ulong;
pub type int_least8_t = ::libc::c_char;
pub type int_least16_t = ::libc::c_short;
pub type int_least32_t = ::libc::c_int;
pub type int_least64_t = ::libc::c_long;
pub type uint_least8_t = ::libc::c_uchar;
pub type uint_least16_t = ::libc::c_ushort;
pub type uint_least32_t = ::libc::c_uint;
pub type uint_least64_t = ::libc::c_ulong;
pub type int_fast8_t = ::libc::c_char;
pub type int_fast16_t = ::libc::c_long;
pub type int_fast32_t = ::libc::c_long;
pub type int_fast64_t = ::libc::c_long;
pub type uint_fast8_t = ::libc::c_uchar;
pub type uint_fast16_t = ::libc::c_ulong;
pub type uint_fast32_t = ::libc::c_ulong;
pub type uint_fast64_t = ::libc::c_ulong;
pub type intptr_t = ::libc::c_long;
pub type uintptr_t = ::libc::c_ulong;
pub type intmax_t = ::libc::c_long;
pub type uintmax_t = ::libc::c_ulong;
pub type opus_int16 = int16_t;
pub type opus_uint16 = uint16_t;
pub type opus_int32 = int32_t;
pub type opus_uint32 = uint32_t;

pub enum OpusEncoder { }
pub enum OpusDecoder { }
pub enum OpusRepacketizer { }

// ------ Constants from opus_defines.h ------

pub const OPUS_OK               : i32 =  0;
pub const OPUS_BAD_ARG          : i32 = -1;
pub const OPUS_BUFFER_TOO_SMALL : i32 = -2;
pub const OPUS_INTERNAL_ERROR   : i32 = -3;
pub const OPUS_INVALID_PACKET   : i32 = -4;
pub const OPUS_UNIMPLEMENTED    : i32 = -5;
pub const OPUS_INVALID_STATE    : i32 = -6;
pub const OPUS_ALLOC_FAIL       : i32 = -7;

// -------------------------------------------

extern "C" {
    pub fn opus_strerror(error: ::libc::c_int) -> *const ::libc::c_char;
    pub fn opus_get_version_string() -> *const ::libc::c_char;
    pub fn opus_encoder_get_size(channels: ::libc::c_int) -> ::libc::c_int;
    pub fn opus_encoder_create(Fs: opus_int32, channels: ::libc::c_int,
                               application: ::libc::c_int,
                               error: *mut ::libc::c_int) -> *mut OpusEncoder;
    pub fn opus_encoder_init(st: *mut OpusEncoder, Fs: opus_int32,
                             channels: ::libc::c_int,
                             application: ::libc::c_int) -> ::libc::c_int;
    pub fn opus_encode(st: *mut OpusEncoder, pcm: *const opus_int16,
                       frame_size: ::libc::c_int, data: *mut ::libc::c_uchar,
                       max_data_bytes: opus_int32) -> opus_int32;
    pub fn opus_encode_float(st: *mut OpusEncoder,
                             pcm: *const ::libc::c_float,
                             frame_size: ::libc::c_int,
                             data: *mut ::libc::c_uchar,
                             max_data_bytes: opus_int32) -> opus_int32;
    pub fn opus_encoder_destroy(st: *mut OpusEncoder) -> ();
    pub fn opus_encoder_ctl(st: *mut OpusEncoder, request: ::libc::c_int, ...)
     -> ::libc::c_int;
    pub fn opus_decoder_get_size(channels: ::libc::c_int) -> ::libc::c_int;
    pub fn opus_decoder_create(Fs: opus_int32, channels: ::libc::c_int,
                               error: *mut ::libc::c_int) -> *mut OpusDecoder;
    pub fn opus_decoder_init(st: *mut OpusDecoder, Fs: opus_int32,
                             channels: ::libc::c_int) -> ::libc::c_int;
    pub fn opus_decode(st: *mut OpusDecoder, data: *const ::libc::c_uchar,
                       len: opus_int32, pcm: *mut opus_int16,
                       frame_size: ::libc::c_int, decode_fec: ::libc::c_int)
     -> ::libc::c_int;
    pub fn opus_decode_float(st: *mut OpusDecoder,
                             data: *const ::libc::c_uchar, len: opus_int32,
                             pcm: *mut ::libc::c_float,
                             frame_size: ::libc::c_int,
                             decode_fec: ::libc::c_int) -> ::libc::c_int;
    pub fn opus_decoder_ctl(st: *mut OpusDecoder, request: ::libc::c_int, ...)
     -> ::libc::c_int;
    pub fn opus_decoder_destroy(st: *mut OpusDecoder) -> ();
    pub fn opus_packet_parse(data: *const ::libc::c_uchar, len: opus_int32,
                             out_toc: *mut ::libc::c_uchar,
                             frames: *mut *const ::libc::c_uchar,
                             size: *mut opus_int16,
                             payload_offset: *mut ::libc::c_int)
     -> ::libc::c_int;
    pub fn opus_packet_get_bandwidth(data: *const ::libc::c_uchar)
     -> ::libc::c_int;
    pub fn opus_packet_get_samples_per_frame(data: *const ::libc::c_uchar,
                                             Fs: opus_int32) -> ::libc::c_int;
    pub fn opus_packet_get_nb_channels(data: *const ::libc::c_uchar)
     -> ::libc::c_int;
    pub fn opus_packet_get_nb_frames(packet: *const ::libc::c_uchar,
                                     len: opus_int32) -> ::libc::c_int;
    pub fn opus_packet_get_nb_samples(packet: *const ::libc::c_uchar,
                                      len: opus_int32, Fs: opus_int32)
     -> ::libc::c_int;
    pub fn opus_decoder_get_nb_samples(dec: *const OpusDecoder,
                                       packet: *const ::libc::c_uchar,
                                       len: opus_int32) -> ::libc::c_int;
    pub fn opus_pcm_soft_clip(pcm: *mut ::libc::c_float,
                              frame_size: ::libc::c_int,
                              channels: ::libc::c_int,
                              softclip_mem: *mut ::libc::c_float) -> ();
    pub fn opus_repacketizer_get_size() -> ::libc::c_int;
    pub fn opus_repacketizer_init(rp: *mut OpusRepacketizer)
     -> *mut OpusRepacketizer;
    pub fn opus_repacketizer_create() -> *mut OpusRepacketizer;
    pub fn opus_repacketizer_destroy(rp: *mut OpusRepacketizer) -> ();
    pub fn opus_repacketizer_cat(rp: *mut OpusRepacketizer,
                                 data: *const ::libc::c_uchar,
                                 len: opus_int32) -> ::libc::c_int;
    pub fn opus_repacketizer_out_range(rp: *mut OpusRepacketizer,
                                       begin: ::libc::c_int,
                                       end: ::libc::c_int,
                                       data: *mut ::libc::c_uchar,
                                       maxlen: opus_int32) -> opus_int32;
    pub fn opus_repacketizer_get_nb_frames(rp: *mut OpusRepacketizer)
     -> ::libc::c_int;
    pub fn opus_repacketizer_out(rp: *mut OpusRepacketizer,
                                 data: *mut ::libc::c_uchar,
                                 maxlen: opus_int32) -> opus_int32;
    pub fn opus_packet_pad(data: *mut ::libc::c_uchar, len: opus_int32,
                           new_len: opus_int32) -> ::libc::c_int;
    pub fn opus_packet_unpad(data: *mut ::libc::c_uchar, len: opus_int32)
     -> opus_int32;
    pub fn opus_multistream_packet_pad(data: *mut ::libc::c_uchar,
                                       len: opus_int32, new_len: opus_int32,
                                       nb_streams: ::libc::c_int)
     -> ::libc::c_int;
    pub fn opus_multistream_packet_unpad(data: *mut ::libc::c_uchar,
                                         len: opus_int32,
                                         nb_streams: ::libc::c_int)
     -> opus_int32;
}

// ------ From opus_multistream.h ------------

pub enum OpusMSEncoder { }
pub enum OpusMSDecoder { }

extern "C" {
    pub fn opus_multistream_encoder_get_size(streams: ::libc::c_int,
                                             coupled_streams: ::libc::c_int)
     -> opus_int32;
    pub fn opus_multistream_surround_encoder_get_size(channels: ::libc::c_int,
                                                      mapping_family:
                                                          ::libc::c_int)
     -> opus_int32;
    pub fn opus_multistream_encoder_create(Fs: opus_int32,
                                           channels: ::libc::c_int,
                                           streams: ::libc::c_int,
                                           coupled_streams: ::libc::c_int,
                                           mapping: *const ::libc::c_uchar,
                                           application: ::libc::c_int,
                                           error: *mut ::libc::c_int)
     -> *mut OpusMSEncoder;
    pub fn opus_multistream_surround_encoder_create(Fs: opus_int32,
                                                    channels: ::libc::c_int,
                                                    mapping_family:
                                                        ::libc::c_int,
                                                    streams:
                                                        *mut ::libc::c_int,
                                                    coupled_streams:
                                                        *mut ::libc::c_int,
                                                    mapping:
                                                        *mut ::libc::c_uchar,
                                                    application:
                                                        ::libc::c_int,
                                                    error: *mut ::libc::c_int)
     -> *mut OpusMSEncoder;
    pub fn opus_multistream_encoder_init(st: *mut OpusMSEncoder,
                                         Fs: opus_int32,
                                         channels: ::libc::c_int,
                                         streams: ::libc::c_int,
                                         coupled_streams: ::libc::c_int,
                                         mapping: *const ::libc::c_uchar,
                                         application: ::libc::c_int)
     -> ::libc::c_int;
    pub fn opus_multistream_surround_encoder_init(st: *mut OpusMSEncoder,
                                                  Fs: opus_int32,
                                                  channels: ::libc::c_int,
                                                  mapping_family:
                                                      ::libc::c_int,
                                                  streams: *mut ::libc::c_int,
                                                  coupled_streams:
                                                      *mut ::libc::c_int,
                                                  mapping:
                                                      *mut ::libc::c_uchar,
                                                  application: ::libc::c_int)
     -> ::libc::c_int;
    pub fn opus_multistream_encode(st: *mut OpusMSEncoder,
                                   pcm: *const opus_int16,
                                   frame_size: ::libc::c_int,
                                   data: *mut ::libc::c_uchar,
                                   max_data_bytes: opus_int32)
     -> ::libc::c_int;
    pub fn opus_multistream_encode_float(st: *mut OpusMSEncoder,
                                         pcm: *const ::libc::c_float,
                                         frame_size: ::libc::c_int,
                                         data: *mut ::libc::c_uchar,
                                         max_data_bytes: opus_int32)
     -> ::libc::c_int;
    pub fn opus_multistream_encoder_destroy(st: *mut OpusMSEncoder) -> ();
    pub fn opus_multistream_encoder_ctl(st: *mut OpusMSEncoder,
                                        request: ::libc::c_int, ...)
     -> ::libc::c_int;
    pub fn opus_multistream_decoder_get_size(streams: ::libc::c_int,
                                             coupled_streams: ::libc::c_int)
     -> opus_int32;
    pub fn opus_multistream_decoder_create(Fs: opus_int32,
                                           channels: ::libc::c_int,
                                           streams: ::libc::c_int,
                                           coupled_streams: ::libc::c_int,
                                           mapping: *const ::libc::c_uchar,
                                           error: *mut ::libc::c_int)
     -> *mut OpusMSDecoder;
    pub fn opus_multistream_decoder_init(st: *mut OpusMSDecoder,
                                         Fs: opus_int32,
                                         channels: ::libc::c_int,
                                         streams: ::libc::c_int,
                                         coupled_streams: ::libc::c_int,
                                         mapping: *const ::libc::c_uchar)
     -> ::libc::c_int;
    pub fn opus_multistream_decode(st: *mut OpusMSDecoder,
                                   data: *const ::libc::c_uchar,
                                   len: opus_int32, pcm: *mut opus_int16,
                                   frame_size: ::libc::c_int,
                                   decode_fec: ::libc::c_int)
     -> ::libc::c_int;
    pub fn opus_multistream_decode_float(st: *mut OpusMSDecoder,
                                         data: *const ::libc::c_uchar,
                                         len: opus_int32,
                                         pcm: *mut ::libc::c_float,
                                         frame_size: ::libc::c_int,
                                         decode_fec: ::libc::c_int)
     -> ::libc::c_int;
    pub fn opus_multistream_decoder_ctl(st: *mut OpusMSDecoder,
                                        request: ::libc::c_int, ...)
     -> ::libc::c_int;
    pub fn opus_multistream_decoder_destroy(st: *mut OpusMSDecoder) -> ();
}
