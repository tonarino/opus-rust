// Based on libopus/tests/test_opus_padding.c

/* Check for overflow in reading the padding length.
 * http://lists.xiph.org/pipermail/opus/2012-November/001834.html
 */

extern crate opus;

#[test]
fn test_overflow() {
    const PACKETSIZE: usize = 16909318;
    const CHANNELS: opus::Channels = opus::Channels::Stereo;
    const FRAMESIZE: usize = 5760;

    let mut input = vec![0xff; PACKETSIZE];
    let mut output = vec![0i16; FRAMESIZE * 2];

    input[0] = 0xff;
    input[1] = 0x41;
    *input.last_mut().unwrap() = 0x0b;

    let mut decoder = opus::Decoder::new(48000, CHANNELS).unwrap();
    let result = decoder.decode(&input[..], &mut output[..], false);
    drop(decoder);
    drop(input);
    drop(output);

    assert_eq!(result.unwrap_err().code(), opus::ErrorCode::InvalidPacket);
}
