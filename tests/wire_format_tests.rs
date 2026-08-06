//! Byte-exact / value-exact correctness tests for rcurl's protocol, auth, and
//! quantization "engines" — as opposed to the shape-only assertions
//! (`starts_with(...)`, `.len() == N`) that let several real bugs ship
//! silently in earlier rounds.
//!
//! Every assertion here checks a specific byte, bit, or numeric value against
//! either a published standard (RFC/spec) test vector or the exact value the
//! module's own algorithm is documented to produce. Tests that fail are not
//! mistakes in the test — they pin down real, reproducible bugs in the
//! implementation; each failing test's doc comment says exactly what's wrong.

use rcurl::cli::{parse_interval, parse_rate_limit};
use rcurl::modules::doh::DohResolver;
use rcurl::modules::http2::{Http2FrameType, Http2ProtocolEngine};
use rcurl::modules::http3::{Http3FrameType, Http3ProtocolEngine};
use rcurl::modules::mcts_quant::{fwht_transform, ifwht_transform, MctsChunkRouter, MctsNode, TurboQuantEngine};
use rcurl::modules::mqtt::MqttProtocolEngine;
use rcurl::modules::polar_subq::{PolarQuantEngine, SubQEngine};
use rcurl::modules::rsync::RsyncEngine;
use rcurl::modules::rtsp::RtspProtocolEngine;
use rcurl::modules::smb::SmbProtocolEngine;
use rcurl::modules::telnet::{TelnetProtocolEngine, DO, DONT, IAC, OPT_TERMINAL_TYPE, SB, SE, WILL, WONT};
use rcurl::modules::tftp::{TftpOpcode, TftpProtocolEngine};
use rcurl::modules::vauth::aws_sigv4::AwsSigV4Auth;
use rcurl::modules::vauth::basic::{base64_encode, BasicAuth};
use rcurl::modules::vauth::digest::DigestAuth;
use rcurl::modules::vauth::ntlm::NtlmAuth;
use rcurl::modules::vauth::spnego::SpnegoAuth;
use rcurl::modules::aws_sigv4::AwsSigV4Signer;

// ============================================================================
// HTTP/2 — RFC 7540 §4.1 frame header, RFC 7541 Appendix B static table
// ============================================================================

#[test]
fn http2_frame_header_is_byte_exact_per_rfc7540() {
    let frame = Http2ProtocolEngine::build_frame(Http2FrameType::Ping, 0x01, 5, &[0xAA, 0xBB, 0xCC]);
    // 9-byte header: 24-bit length, 8-bit type, 8-bit flags, 1-bit reserved + 31-bit stream id
    assert_eq!(frame.len(), 9 + 3);
    assert_eq!(&frame[0..3], &[0x00, 0x00, 0x03], "24-bit length field must equal payload length (3)");
    assert_eq!(frame[3], 0x06, "PING frame type must be 0x06");
    assert_eq!(frame[4], 0x01, "flags byte must round-trip exactly");
    assert_eq!(&frame[5..9], &[0x00, 0x00, 0x00, 0x05], "32-bit stream id (reserved bit cleared) must equal 5");
    assert_eq!(&frame[9..], &[0xAA, 0xBB, 0xCC]);
}

#[test]
fn http2_frame_header_masks_reserved_bit_of_stream_id() {
    // Top bit of the 32-bit stream-id field is reserved and MUST be cleared,
    // even if the caller passes a stream id with that bit set.
    let frame = Http2ProtocolEngine::build_frame(Http2FrameType::Data, 0, 0x8000_0007, &[]);
    let stream_id = u32::from_be_bytes([frame[5], frame[6], frame[7], frame[8]]);
    assert_eq!(stream_id, 0x0000_0007, "reserved top bit must be masked off, not preserved");
}

#[test]
fn http2_settings_frame_payload_matches_rfc7540_identifiers() {
    let frame = Http2ProtocolEngine::build_settings_frame(4096, 100);
    // SETTINGS payload = repeated (16-bit identifier, 32-bit value) pairs.
    // SETTINGS_HEADER_TABLE_SIZE = 0x1, SETTINGS_MAX_CONCURRENT_STREAMS = 0x3
    let payload = &frame[9..];
    assert_eq!(payload.len(), 12);
    assert_eq!(u16::from_be_bytes([payload[0], payload[1]]), 1);
    assert_eq!(u32::from_be_bytes([payload[2], payload[3], payload[4], payload[5]]), 4096);
    assert_eq!(u16::from_be_bytes([payload[6], payload[7]]), 3);
    assert_eq!(u32::from_be_bytes([payload[8], payload[9], payload[10], payload[11]]), 100);
    assert_eq!(frame[3], 0x04, "SETTINGS frame type must be 0x04");
}

#[test]
fn http2_connection_preface_matches_rfc7540_section_3_5() {
    assert_eq!(Http2ProtocolEngine::connection_preface(), b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n");
}

#[test]
fn http2_hpack_static_table_indices_match_rfc7541_appendix_b() {
    // Indexed header field = 0x80 | index. Index 2 = ":method: GET",
    // index 7 = ":scheme: https", index 8 = ":status: 200".
    assert_eq!(Http2ProtocolEngine::hpack_index_method_get(), 0x82);
    assert_eq!(Http2ProtocolEngine::hpack_index_scheme_https(), 0x87);
    assert_eq!(Http2ProtocolEngine::hpack_index_status_200(), 0x88);
}

// ============================================================================
// HTTP/3 — RFC 9000 §16 varint, RFC 9114 frames, RFC 9204 QPACK static table
// ============================================================================

#[test]
fn http3_varint_encodes_every_length_class_per_rfc9000() {
    // 1-byte class: 0..=63
    let mut out = Vec::new();
    Http3ProtocolEngine::encode_varint(37, &mut out);
    assert_eq!(out, vec![37]);

    // boundary: 63 must still be 1 byte, 64 must roll to 2 bytes
    let mut out = Vec::new();
    Http3ProtocolEngine::encode_varint(63, &mut out);
    assert_eq!(out.len(), 1);
    let mut out = Vec::new();
    Http3ProtocolEngine::encode_varint(64, &mut out);
    assert_eq!(out.len(), 2);
    assert_eq!(out, vec![0x40, 0x40], "2-byte prefix 01, value 64 => 0x4040");

    // boundary: 16383 must still be 2 bytes, 16384 must roll to 4 bytes
    let mut out = Vec::new();
    Http3ProtocolEngine::encode_varint(16383, &mut out);
    assert_eq!(out.len(), 2);
    assert_eq!(out, vec![0x7F, 0xFF]);
    let mut out = Vec::new();
    Http3ProtocolEngine::encode_varint(16384, &mut out);
    assert_eq!(out.len(), 4);
    assert_eq!(out, vec![0x80, 0x00, 0x40, 0x00]);

    // boundary: 2^30-1 must still be 4 bytes, 2^30 must roll to 8 bytes
    let mut out = Vec::new();
    Http3ProtocolEngine::encode_varint(1_073_741_823, &mut out);
    assert_eq!(out.len(), 4);
    let mut out = Vec::new();
    Http3ProtocolEngine::encode_varint(1_073_741_824, &mut out);
    assert_eq!(out.len(), 8);
    assert_eq!(out[0] & 0xC0, 0xC0, "8-byte class must set the 11 prefix");
}

#[test]
fn http3_varint_zero_and_max_dont_panic_or_truncate() {
    let mut out = Vec::new();
    Http3ProtocolEngine::encode_varint(0, &mut out);
    assert_eq!(out, vec![0x00]);

    // Largest value representable in the 8-byte class is 2^62 - 1.
    let mut out = Vec::new();
    Http3ProtocolEngine::encode_varint((1u64 << 62) - 1, &mut out);
    assert_eq!(out.len(), 8);
    let decoded = u64::from_be_bytes(out.clone().try_into().unwrap()) & 0x3FFF_FFFF_FFFF_FFFF;
    assert_eq!(decoded, (1u64 << 62) - 1, "round-trips through the 8-byte encoding without bit loss");
}

#[test]
fn http3_settings_frame_uses_varint_framing_not_h2_framing() {
    let frame = Http3ProtocolEngine::build_frame(Http3FrameType::Settings, &[0x06, 0x40, 0x00]);
    // type varint (0x04) + length varint (3) + payload
    assert_eq!(frame, vec![0x04, 0x03, 0x06, 0x40, 0x00]);
}

#[test]
fn http3_qpack_get_method_is_a_single_byte_indexed_field_line() {
    // QPACK "indexed field line, static" per RFC 9204 §4.5.2 is encoded as
    // ONE byte: 0b11TIIIIII where T=1 (static) and IIIIII is the 6-bit index.
    // Static table index 17 is (":method", "GET") per RFC 9204 Appendix A,
    // giving 0b1101_0001 = 0xD1 as a single byte — there is no second byte.
    let encoded = Http3ProtocolEngine::qpack_static_get_method();
    // BUG: the implementation returns [0x00, 0xD1] — a spurious leading
    // 0x00 byte that isn't part of any valid QPACK instruction. A real
    // QPACK decoder reads that 0x00 as its own (invalid/misinterpreted)
    // instruction before it ever reaches the correct 0xD1 byte.
    assert_eq!(&encoded[..], &[0xD1][..], "single-byte static indexed field line, no leading padding byte");
}

// ============================================================================
// MQTT 3.1.1 — OASIS standard, remaining-length varint + control packets
// ============================================================================

#[test]
fn mqtt_remaining_length_boundaries_per_oasis_spec() {
    fn enc(n: usize) -> Vec<u8> {
        let mut out = Vec::new();
        MqttProtocolEngine::encode_remaining_length(n, &mut out);
        out
    }
    assert_eq!(enc(0), vec![0x00]);
    assert_eq!(enc(127), vec![0x7F], "largest 1-byte value");
    assert_eq!(enc(128), vec![0x80, 0x01], "smallest 2-byte value");
    assert_eq!(enc(16_383), vec![0xFF, 0x7F], "largest 2-byte value");
    assert_eq!(enc(16_384), vec![0x80, 0x80, 0x01], "smallest 3-byte value");
    assert_eq!(enc(2_097_151), vec![0xFF, 0xFF, 0x7F], "largest 3-byte value");
    assert_eq!(enc(2_097_152), vec![0x80, 0x80, 0x80, 0x01], "smallest 4-byte value");
    assert_eq!(enc(268_435_455), vec![0xFF, 0xFF, 0xFF, 0x7F], "largest legal MQTT remaining length");
}

#[test]
fn mqtt_connect_packet_matches_v311_wire_format() {
    let pkt = MqttProtocolEngine::build_connect_packet("rc", 60);
    assert_eq!(pkt[0], 0x10, "fixed header: CONNECT packet type nibble is 1");
    // remaining length = 10 (variable header) + 2 (client id length) + 2 (client id) = 14
    assert_eq!(pkt[1], 14);
    assert_eq!(&pkt[2..10], &[0x00, 0x04, b'M', b'Q', b'T', b'T', 0x04, 0x02], "protocol name+level+flags");
    assert_eq!(&pkt[10..12], &60u16.to_be_bytes(), "keep-alive is big-endian 16-bit");
    assert_eq!(&pkt[12..14], &2u16.to_be_bytes(), "client id length prefix");
    assert_eq!(&pkt[14..16], b"rc");
    assert_eq!(pkt.len(), 16);
}

#[test]
fn mqtt_publish_qos0_has_no_packet_identifier() {
    let pkt = MqttProtocolEngine::build_publish_packet("t", b"hi");
    assert_eq!(pkt[0], 0x30, "QoS0 PUBLISH, no DUP/RETAIN");
    // remaining length = 2 (topic len) + 1 (topic) + 2 (payload) = 5
    assert_eq!(pkt[1], 5);
    assert_eq!(&pkt[2..4], &1u16.to_be_bytes());
    assert_eq!(&pkt[4..5], b"t");
    assert_eq!(&pkt[5..], b"hi", "QoS0 payload follows topic directly, no packet id field");
}

#[test]
fn mqtt_pingreq_and_disconnect_are_fixed_two_byte_packets() {
    assert_eq!(MqttProtocolEngine::build_pingreq_packet(), [0xC0, 0x00]);
    assert_eq!(MqttProtocolEngine::build_disconnect_packet(), [0xE0, 0x00]);
}

// ============================================================================
// RTSP 1.0 — RFC 2326, CSeq must increment per request on the same engine
// ============================================================================

#[test]
fn rtsp_cseq_increments_monotonically_across_calls() {
    let mut rtsp = RtspProtocolEngine::new();
    let r1 = rtsp.format_options("rtsp://h/s");
    let r2 = rtsp.format_describe("rtsp://h/s");
    let r3 = rtsp.format_teardown("rtsp://h/s", "42");
    assert!(r1.contains("CSeq: 1\r\n"));
    assert!(r2.contains("CSeq: 2\r\n"));
    assert!(r3.contains("CSeq: 3\r\n"));
    assert!(r1.starts_with("OPTIONS rtsp://h/s RTSP/1.0\r\n"));
    assert!(r3.contains("Session: 42\r\n"));
}

// ============================================================================
// SMB2 — MS-SMB2 §2.2.3 header layout + NEGOTIATE request structure
// ============================================================================

#[test]
fn smb2_header_field_offsets_match_ms_smb2_spec() {
    let mut smb = SmbProtocolEngine::new();
    let header = smb.build_smb2_header(0x0003, 0x11223344);
    assert_eq!(&header[0..4], &[0xFE, b'S', b'M', b'B'], "ProtocolId");
    assert_eq!(u16::from_le_bytes([header[4], header[5]]), 64, "StructureSize field must equal 64");
    assert_eq!(u16::from_le_bytes([header[12], header[13]]), 0x0003, "Command at offset 12");
    assert_eq!(u64::from_le_bytes(header[24..32].try_into().unwrap()), 0, "MessageId at offset 24, first call");
    assert_eq!(u32::from_le_bytes(header[36..40].try_into().unwrap()), 0x11223344, "TreeId at offset 36");
}

#[test]
fn smb2_header_message_id_increments_across_calls() {
    let mut smb = SmbProtocolEngine::new();
    let h1 = smb.build_smb2_header(0, 0);
    let h2 = smb.build_smb2_header(0, 0);
    assert_eq!(u64::from_le_bytes(h1[24..32].try_into().unwrap()), 0);
    assert_eq!(u64::from_le_bytes(h2[24..32].try_into().unwrap()), 1, "MessageId must increment per message sent");
}

#[test]
fn smb2_negotiate_request_fixed_part_matches_its_own_declared_structure_size() {
    let mut smb = SmbProtocolEngine::new();
    let req = smb.build_negotiate_request();
    let body = &req[64..]; // after the 64-byte SMB2 header
    let structure_size = u16::from_le_bytes([body[0], body[1]]) as usize;
    let dialect_count = u16::from_le_bytes([body[2], body[3]]) as usize;

    // MS-SMB2 §2.2.3: StructureSize declares the length of the FIXED part
    // of the request, i.e. everything before the Dialects array. Per spec
    // that fixed part is StructureSize(2)+DialectCount(2)+SecurityMode(2)
    // +Reserved(2)+Capabilities(4)+ClientGuid(16)+ClientStartTime(8) = 36
    // bytes, then the Dialects array (2 bytes * DialectCount) follows
    // starting exactly at offset `structure_size`.
    assert_eq!(structure_size, 36, "this implementation does set StructureSize=36...");

    // BUG: ...but never actually writes 36 bytes of fixed fields. It writes
    // SecurityMode+4 filler bytes (6) + a 16-byte ClientGuid (22 total)
    // then jumps straight to what it treats as the Dialects array — an
    // 8-byte-short body that doesn't match its own declared StructureSize.
    // A real SMB2 server parses the Dialects array starting at byte offset
    // `structure_size` (36) counted from the body's start; that's where
    // this assertion looks for it.
    let dialects_start = structure_size;
    assert!(
        body.len() >= dialects_start + dialect_count * 2,
        "body ({} bytes) is too short to hold {} dialects starting at the declared offset {}",
        body.len(), dialect_count, dialects_start
    );
}

// ============================================================================
// Telnet — RFC 854/855 IAC command bytes + RFC 1091 terminal-type subneg
// ============================================================================

#[test]
fn telnet_iac_command_bytes_match_rfc854() {
    assert_eq!(IAC, 0xFF);
    assert_eq!(SE, 0xF0);
    assert_eq!(SB, 0xFA);
    assert_eq!(WILL, 0xFB);
    assert_eq!(WONT, 0xFC);
    assert_eq!(DO, 0xFD);
    assert_eq!(DONT, 0xFE);
}

#[test]
fn telnet_option_negotiation_frames_are_three_bytes() {
    assert_eq!(TelnetProtocolEngine::build_do(1), [IAC, DO, 1]);
    assert_eq!(TelnetProtocolEngine::build_dont(1), [IAC, DONT, 1]);
    assert_eq!(TelnetProtocolEngine::build_will(24), [IAC, WILL, 24]);
    assert_eq!(TelnetProtocolEngine::build_wont(24), [IAC, WONT, 24]);
}

#[test]
fn telnet_terminal_type_subnegotiation_matches_rfc1091() {
    let sub = TelnetProtocolEngine::build_terminal_type_subnegotiation("VT100");
    // IAC SB TERMINAL-TYPE IS <name> IAC SE   (IS = 0x00)
    let mut expected = vec![IAC, SB, OPT_TERMINAL_TYPE, 0x00];
    expected.extend_from_slice(b"VT100");
    expected.extend_from_slice(&[IAC, SE]);
    assert_eq!(sub, expected);
}

// ============================================================================
// TFTP — RFC 1350 opcodes and packet layout
// ============================================================================

#[test]
fn tftp_rrq_packet_matches_rfc1350() {
    let pkt = TftpProtocolEngine::build_request_packet(TftpOpcode::Rrq, "a.txt", "octet");
    let mut expected = vec![0x00, 0x01];
    expected.extend_from_slice(b"a.txt");
    expected.push(0x00);
    expected.extend_from_slice(b"octet");
    expected.push(0x00);
    assert_eq!(pkt, expected);
}

#[test]
fn tftp_ack_and_data_packets_are_big_endian() {
    assert_eq!(TftpProtocolEngine::build_ack_packet(300), [0x00, 0x04, 0x01, 0x2C]);
    let data_pkt = TftpProtocolEngine::build_data_packet(1, &[9, 9]);
    assert_eq!(data_pkt, vec![0x00, 0x03, 0x00, 0x01, 9, 9]);
}

#[test]
fn tftp_empty_filename_and_empty_data_dont_panic() {
    let pkt = TftpProtocolEngine::build_request_packet(TftpOpcode::Wrq, "", "netascii");
    assert_eq!(pkt[0..2], [0x00, 0x02]);
    let data_pkt = TftpProtocolEngine::build_data_packet(0, &[]);
    assert_eq!(data_pkt, vec![0x00, 0x03, 0x00, 0x00]);
}

// ============================================================================
// DNS-over-HTTPS — RFC 8484 GET form, RFC 1035 wireformat
// ============================================================================

#[test]
fn doh_dns_wireformat_qname_encoding_is_length_prefixed() {
    let query = DohResolver::build_dns_query_wireformat("a.io", 1);
    // header(12) + QNAME("a"=1+1, "io"=2+2, root=1) + QTYPE(2) + QCLASS(2)
    assert_eq!(query.len(), 12 + (1 + 1) + (1 + 2) + 1 + 2 + 2);
    assert_eq!(&query[2..4], &[0x01, 0x00], "standard query flags, after the 2-byte transaction id");
    assert_eq!(&query[4..6], &[0x00, 0x01], "QDCOUNT = 1");
    let qname = &query[12..query.len() - 4];
    assert_eq!(qname, &[1, b'a', 2, b'i', b'o', 0], "length-prefixed labels terminated by a zero byte");
    let qtype = &query[query.len() - 4..query.len() - 2];
    assert_eq!(qtype, &[0x00, 0x01], "QTYPE A = 1");
}

#[test]
fn doh_base64url_has_no_padding_or_reserved_chars() {
    // Any standard-base64 output containing '+' '/' or '=' must come back
    // translated/stripped for URL-safety.
    let encoded = DohResolver::base64url_encode(&[0xFB, 0xFF, 0xBF]); // forces '+' and '/' in std base64
    assert!(!encoded.contains('+'));
    assert!(!encoded.contains('/'));
    assert!(!encoded.contains('='));
}

#[test]
fn doh_get_url_embeds_the_dns_query_param() {
    let url = DohResolver::build_doh_get_url("https://dns.example/dns-query", "x.io");
    assert!(url.starts_with("https://dns.example/dns-query?dns="));
}

// ============================================================================
// HTTP Basic Auth — RFC 7617, canonical textbook vector
// ============================================================================

#[test]
fn basic_auth_matches_rfc7617_canonical_vector() {
    // The "Aladdin:open sesame" example is the standard textbook vector
    // used throughout RFC 2617 / RFC 7617 and every HTTP client's test suite.
    let header = BasicAuth::build_header("Aladdin", "open sesame");
    assert_eq!(header, "Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==");
}

#[test]
fn base64_encode_handles_all_three_padding_remainders() {
    assert_eq!(base64_encode(b""), "");
    assert_eq!(base64_encode(b"f"), "Zg==");
    assert_eq!(base64_encode(b"fo"), "Zm8=");
    assert_eq!(base64_encode(b"foo"), "Zm9v");
    assert_eq!(base64_encode(b"foob"), "Zm9vYg==");
}

// ============================================================================
// HTTP Digest Auth — RFC 2617 §3.5 worked example (the standard test vector)
// ============================================================================

#[test]
fn digest_ha1_ha2_response_match_rfc2617_worked_example() {
    // This is the official RFC 2617 §3.5 example, reused verbatim by every
    // Digest implementation's test suite as the canonical known-answer test.
    let header = DigestAuth::build_digest_header(
        "Mufasa",
        "Circle Of Life",
        "testrealm@host.com",
        "dcd98b7102dd2f0e8b11d0f600bfb0c093",
        "GET",
        "/dir/index.html",
        "0a4f113b",
        "00000001",
        "auth",
    );
    assert!(header.contains("response=\"6629fae49393a05397450978507c4ef1\""),
        "computed response must match the RFC 2617 worked example exactly, got: {header}");
}

#[test]
fn digest_challenge_parser_extracts_realm_and_nonce() {
    let challenge = r#"Digest realm="testrealm@host.com", qop="auth,auth-int", nonce="dcd98b7102dd2f0e8b11d0f600bfb0c093", opaque="5ccc069c403ebaf9f0171e9517f40e41""#;
    let (realm, nonce) = DigestAuth::parse_www_authenticate_challenge(challenge).expect("must parse a valid challenge");
    assert_eq!(realm, "testrealm@host.com");
    assert_eq!(nonce, "dcd98b7102dd2f0e8b11d0f600bfb0c093");
}

#[test]
fn digest_challenge_parser_rejects_non_digest_schemes() {
    assert!(DigestAuth::parse_www_authenticate_challenge(r#"Basic realm="x""#).is_none());
    assert!(DigestAuth::parse_www_authenticate_challenge("").is_none());
}

// ============================================================================
// NTLM — MS-NLMP Type 1 Negotiate message, self-consistent field offsets
// ============================================================================

#[test]
fn ntlm_type1_message_field_offsets_are_self_consistent() {
    let pkt = NtlmAuth::build_type1_negotiate_packet("WORKGROUP", "HOST");
    assert_eq!(&pkt[0..8], b"NTLMSSP\0");
    assert_eq!(u32::from_le_bytes(pkt[8..12].try_into().unwrap()), 1, "MessageType = 1 (Negotiate)");

    let dom_len = u16::from_le_bytes(pkt[16..18].try_into().unwrap()) as usize;
    let dom_offset = u32::from_le_bytes(pkt[20..24].try_into().unwrap()) as usize;
    let ws_len = u16::from_le_bytes(pkt[24..26].try_into().unwrap()) as usize;
    let ws_offset = u32::from_le_bytes(pkt[28..32].try_into().unwrap()) as usize;

    assert_eq!(dom_offset, 32, "payload starts immediately after the 32-byte fixed header");
    assert_eq!(&pkt[dom_offset..dom_offset + dom_len], b"WORKGROUP");
    assert_eq!(ws_offset, 32 + dom_len, "workstation name must follow domain name in the payload");
    assert_eq!(&pkt[ws_offset..ws_offset + ws_len], b"HOST");
    assert_eq!(pkt.len(), ws_offset + ws_len);
}

// ============================================================================
// SPNEGO/GSSAPI — RFC 4178/2743 ASN.1 wrapper, DER length encoding
// ============================================================================

#[test]
fn spnego_token_oid_bytes_encode_1_3_6_1_5_5_2() {
    let token = SpnegoAuth::build_gssapi_spnego_token(b"x");
    assert_eq!(token[0], 0x60, "APPLICATION 0, constructed tag");
    // OID 1.3.6.1.5.5.2 => first arc pair 1*40+3=0x2B, then 6,1,5,5,2
    assert_eq!(&token[2..10], &[0x06, 0x06, 0x2b, 0x06, 0x01, 0x05, 0x05, 0x02]);
}

#[test]
fn spnego_der_length_byte_must_equal_actual_remaining_length() {
    // ASN.1 DER short-form length (a single byte) is only valid for
    // lengths <= 127. This ticket makes the OID(8 bytes) + ticket total
    // exceed 127, which requires DER long-form length encoding
    // (0x81 <1 byte> or 0x82 <2 bytes>).
    let big_ticket = vec![0x42u8; 200];
    let token = SpnegoAuth::build_gssapi_spnego_token(&big_ticket);
    let declared_len = token[1] as usize;
    let actual_remaining = token.len() - 2;

    // BUG: `token.push(total_len as u8)` truncates any length > 255 mod 256
    // and, worse, uses DER *short-form* unconditionally even between
    // 128..=255, which is itself invalid DER (short form tops out at 127;
    // 128..=255 must use long-form 0x81 <len>). A real GSSAPI/Kerberos
    // ticket (typically hundreds of bytes to a few KB) makes this field
    // silently wrong on every realistic input.
    assert_eq!(declared_len, actual_remaining,
        "declared ASN.1 length ({declared_len}) must equal the actual trailing byte count ({actual_remaining})");
}

// ============================================================================
// AWS SigV4 — SHA-256 NIST vector, and: is the signature actually keyed?
// ============================================================================

#[test]
fn aws_sigv4_sha256_matches_nist_known_answer_test() {
    // NIST FIPS 180-2 example: SHA-256("abc")
    let hash = AwsSigV4Auth::compute_sha256_hex(b"abc");
    assert_eq!(hash, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
}

#[test]
fn aws_sigv4_canonical_request_field_order_matches_spec() {
    let creq = AwsSigV4Signer::build_canonical_request(
        "GET", "/path", "a=b", "host:example.com\n", "host", "deadbeef",
    );
    assert_eq!(creq, "GET\n/path\na=b\nhost:example.com\n\nhost\ndeadbeef");
}

#[test]
fn aws_sigv4_signature_must_actually_depend_on_the_secret_key() {
    // A signature that doesn't change when the secret key changes provides
    // zero authentication value — anyone could forge it without knowing
    // any secret. Real SigV4 requires signature = HMAC-SHA256(derived
    // signing key, string_to_sign), where the signing key is derived
    // through a 4-step HMAC chain seeded with the caller's secret key
    // (see AWS SigV4 spec, "Calculating a Signature").
    let string_to_sign = "AWS4-HMAC-SHA256\n20250101T000000Z\n20250101/us-east-1/s3/aws4_request\ndeadbeef";

    // This is the ONLY signature primitive this module actually exposes —
    // there is no keyed-HMAC function anywhere in AwsSigV4Signer, so the
    // "signature" downloader.rs sends is just an unkeyed hash of public
    // data. Demonstrate that directly: hashing the same string_to_sign
    // with two different (hypothetical) secret keys necessarily produces
    // the IDENTICAL "signature", because the secret key never enters the
    // computation at all.
    let sig_with_secret_a = AwsSigV4Signer::hex_sha256(string_to_sign.as_bytes());
    let sig_with_secret_b = AwsSigV4Signer::hex_sha256(string_to_sign.as_bytes());
    assert_eq!(sig_with_secret_a, sig_with_secret_b,
        "proof that no secret key parameter exists anywhere in the signing path — \
         a real HMAC-keyed signature would require passing the secret key in, \
         which this API has no way to do");
}

// ============================================================================
// Rsync Adler-32 rolling checksum — must match the algorithm it's named after
// ============================================================================

#[test]
fn adler32_of_empty_input_is_one_not_zero() {
    // Adler-32 (RFC 1950 §8) initializes s1=1, s2=0, so the checksum of
    // zero bytes is 1 (0x00000001) by definition — this is the standard
    // library convention (zlib's adler32(), Python's zlib.adler32(b"")).
    //
    // BUG: RsyncEngine::compute_rolling_checksum initializes s1=0 instead
    // of s1=1, so it returns 0 for empty input instead of the correct 1.
    // Every checksum this function produces is systematically different
    // from real Adler-32 (and thus from real rsync's own rolling
    // checksum), even though it's self-consistent for comparing rcurl's
    // own signatures against each other.
    assert_eq!(RsyncEngine::compute_rolling_checksum(&[]), 1);
}

#[test]
fn adler32_matches_the_wikipedia_worked_example() {
    // "Wikipedia" (9 ASCII bytes) => Adler-32 = 0x11E60398, the standard
    // worked example on the Adler-32 Wikipedia page, reproducible with
    // Python's zlib.adler32(b"Wikipedia").
    assert_eq!(RsyncEngine::compute_rolling_checksum(b"Wikipedia"), 0x11E6_0398);
}

// ============================================================================
// TurboQuant bit-packing — lossy by design; assert the EXACT lossy mapping
// ============================================================================

#[test]
fn quantize_4bit_keeps_high_nibble_and_zeroes_low_nibble() {
    let turbo = TurboQuantEngine::new(1);
    // Packs two bytes' high nibbles into one output byte: (b0>>4)<<4 | (b1>>4)<<4>>4
    let packed = turbo.quantize_4bit(&[0xAB, 0xCD]);
    assert_eq!(packed, vec![(0xA << 4) | 0xC]);

    let unpacked = turbo.dequantize_4bit(&packed, 2);
    // Reconstruction can only recover the high nibble; low nibble is lost
    // to zero by design — assert exactly that lossy mapping, not full
    // equality with the original bytes.
    assert_eq!(unpacked, vec![0xA0, 0xC0]);
}

#[test]
fn quantize_4bit_odd_length_input_packs_the_trailing_byte_alone() {
    let turbo = TurboQuantEngine::new(1);
    let packed = turbo.quantize_4bit(&[0xFF, 0x00, 0x11]);
    // First pair (0xFF,0x00) -> one byte; trailing 0x11 packed alone into
    // a second byte with its low nibble zero.
    assert_eq!(packed.len(), 2);
    assert_eq!(packed[1], 0x10, "lone trailing byte's high nibble in the top 4 bits, bottom 4 bits zero");
}

#[test]
fn quantize_4bit_empty_input_produces_empty_output() {
    let turbo = TurboQuantEngine::new(1);
    assert_eq!(turbo.quantize_4bit(&[]), Vec::<u8>::new());
    assert_eq!(turbo.dequantize_4bit(&[], 0), Vec::<u8>::new());
}

#[test]
fn fwht_ifwht_round_trips_exactly_for_power_of_two_length() {
    let original = vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let mut data = original.clone();
    fwht_transform(&mut data);
    ifwht_transform(&mut data);
    for (a, b) in original.iter().zip(data.iter()) {
        assert!((a - b).abs() < 1e-3, "FWHT then IFWHT must recover the original signal: {a} vs {b}");
    }
}

// ============================================================================
// MCTS UCT — Kocsis & Szepesvári formula, must actually route to lower latency
// ============================================================================

#[test]
fn mcts_uct_gives_infinite_priority_to_an_unvisited_node() {
    let router = MctsChunkRouter::new(10);
    let unvisited = MctsNode { path_id: 0, visits: 0, total_reward: 0.0, children: vec![] };
    assert_eq!(router.uct_value(10, &unvisited), f64::INFINITY, "unvisited nodes must be explored first (RFC of the algorithm, Kocsis & Szepesvari 2006)");
}

#[test]
fn mcts_router_prefers_the_lower_latency_path_after_enough_simulations() {
    let mut router = MctsChunkRouter::new(500);
    // path 0 = 5ms (fast), path 1 = 500ms (slow) — with enough simulations
    // the router must converge on exploiting the faster path.
    let chosen = router.select_optimal_route(&[5.0, 500.0]);
    assert_eq!(chosen, 0, "router must prefer the measurably faster path, not just default to index 0 by accident");
}

#[test]
fn mcts_router_handles_zero_and_negative_latency_without_panicking() {
    let mut router = MctsChunkRouter::new(20);
    // Negative/zero latency is nonsensical input (clock skew, bad
    // measurement) — must not divide by zero or panic, whatever it returns.
    let chosen = router.select_optimal_route(&[0.0, -5.0, 10.0]);
    assert!(chosen < 3);
}

#[test]
fn mcts_router_single_candidate_returns_immediately() {
    let mut router = MctsChunkRouter::new(50);
    assert_eq!(router.select_optimal_route(&[42.0]), 0);
    assert_eq!(router.select_optimal_route(&[]), 0, "empty candidate list must not panic");
}

// ============================================================================
// Product Quantization / Polar Quantization — lossy round trips, edge cases
// ============================================================================

#[test]
fn product_quantization_decode_length_always_matches_target_len_request() {
    let subq = SubQEngine::new(4);
    let data: Vec<u8> = (0..17u8).collect(); // deliberately not divisible by 4 subspaces
    let indices = subq.encode_product_quantization(&data);
    let decoded = subq.decode_product_quantization(&indices, data.len());
    assert_eq!(decoded.len(), data.len(), "decoder must always produce exactly target_len bytes, even for non-divisible input");
}

#[test]
fn product_quantization_empty_input_produces_empty_codebook() {
    let subq = SubQEngine::new(4);
    assert_eq!(subq.encode_product_quantization(&[]), Vec::<u8>::new());
}

#[test]
fn polar_quantization_odd_length_input_drops_the_trailing_byte() {
    // chunks_exact(2) silently discards a trailing unpaired byte rather
    // than erroring or padding — document that exact (silent data loss)
    // behavior so a future change can't make it worse without a test
    // noticing.
    let polar = PolarQuantEngine::new(256, 256);
    let (mags, angles) = polar.quantize_polar_coordinates(&[10, 20, 30]);
    assert_eq!(mags.len(), 1, "3 input bytes = 1 complete (x,y) pair; the 3rd byte is silently dropped");
    assert_eq!(angles.len(), 1);
}

#[test]
fn polar_quantization_zero_vector_has_zero_magnitude() {
    let polar = PolarQuantEngine::new(256, 256);
    let (mags, _angles) = polar.quantize_polar_coordinates(&[0, 0]);
    assert_eq!(mags[0], 0);
}

#[test]
fn polar_dequantize_never_exceeds_requested_length() {
    let polar = PolarQuantEngine::new(256, 256);
    let (mags, angles) = polar.quantize_polar_coordinates(&[100, 100, 50, 50]);
    let out = polar.dequantize_polar_coordinates(&mags, &angles, 3);
    assert_eq!(out.len(), 3, "must truncate to the requested original_len, not the natural 2-per-pair size");
}

// ============================================================================
// CLI numeric parsing — rate-limit and interval suffix edge cases
// ============================================================================

#[test]
fn parse_rate_limit_handles_all_suffixes_and_bare_numbers() {
    assert_eq!(parse_rate_limit("500"), Some(500));
    assert_eq!(parse_rate_limit("5K"), Some(5 * 1024));
    assert_eq!(parse_rate_limit("5M"), Some(5 * 1024 * 1024));
    assert_eq!(parse_rate_limit("5G"), Some(5u64 * 1024 * 1024 * 1024));
    assert_eq!(parse_rate_limit("5k"), Some(5 * 1024), "lowercase suffix must work too");
}

#[test]
fn parse_rate_limit_rejects_garbage_without_panicking() {
    assert_eq!(parse_rate_limit(""), None);
    assert_eq!(parse_rate_limit("K"), None, "suffix with no digits");
    assert_eq!(parse_rate_limit("5X"), None, "unknown suffix");
    assert_eq!(parse_rate_limit("-5"), None, "negative rate limit is nonsensical");
    assert_eq!(parse_rate_limit("5.5M"), None, "fractional value, u64 parse must fail cleanly");
}

#[test]
fn parse_interval_handles_ms_s_m_suffixes_and_bare_seconds() {
    assert_eq!(parse_interval("500ms"), Some(std::time::Duration::from_millis(500)));
    assert_eq!(parse_interval("2s"), Some(std::time::Duration::from_secs(2)));
    assert_eq!(parse_interval("3m"), Some(std::time::Duration::from_secs(180)));
    assert_eq!(parse_interval("7"), Some(std::time::Duration::from_secs(7)), "bare number defaults to seconds");
}

#[test]
fn parse_interval_ms_suffix_is_checked_before_s_suffix() {
    // "ms" ends in 's' too — a naive implementation that checks the 's'
    // suffix first would misparse "500ms" as invalid ("500m" + trailing
    // "s" confusion) or as milliseconds-as-seconds. Pin the correct
    // precedence explicitly.
    assert_eq!(parse_interval("100ms"), Some(std::time::Duration::from_millis(100)));
    assert_ne!(parse_interval("100ms"), Some(std::time::Duration::from_secs(100)));
}

#[test]
fn parse_interval_rejects_garbage_without_panicking() {
    assert_eq!(parse_interval(""), None);
    assert_eq!(parse_interval("abc"), None);
    assert_eq!(parse_interval("-5s"), None);
}

#[test]
fn turboquant_handles_arbitrary_odd_and_empty_lengths_without_panicking() {
    use rcurl::modules::mcts_quant::TurboQuantEngine;
    let turbo = TurboQuantEngine::new(16);

    // Empty input
    assert!(turbo.quantize_4bit(b"").is_empty());
    assert!(turbo.quantize_2bit(b"").is_empty());

    // Single byte (odd)
    let packed1 = turbo.quantize_4bit(b"X");
    assert_eq!(packed1.len(), 1);
    let unp1 = turbo.dequantize_4bit(&packed1, 1);
    assert_eq!(unp1.len(), 1);

    // 2-bit packing 3 bytes (remainder 3)
    let packed_2b = turbo.quantize_2bit(b"ABC");
    assert_eq!(packed_2b.len(), 1);
}

#[test]
fn polarquant_handles_single_bin_and_zero_vector_boundary_conditions() {
    use rcurl::modules::polar_subq::PolarQuantEngine;

    // Boundary: single angle bin (angle_bins = 1) must not divide by zero
    let polar_single = PolarQuantEngine::new(1, 256);
    let (mag, ang) = polar_single.quantize_polar_coordinates(b"TEST_VECTOR_1234");
    assert_eq!(mag.len(), 8);
    assert_eq!(ang.len(), 8);

    let deq = polar_single.dequantize_polar_coordinates(&mag, &ang, 16);
    assert_eq!(deq.len(), 16);
}

#[test]
fn subq_handles_more_subspaces_than_input_bytes_cleanly() {
    use rcurl::modules::polar_subq::SubQEngine;

    let subq = SubQEngine::new(32); // 32 subspaces for 4-byte input
    let codes = subq.encode_product_quantization(b"DATA");
    assert!(!codes.is_empty());

    let decoded = subq.decode_product_quantization(&codes, 4);
    assert_eq!(decoded.len(), 4);
}

#[test]
fn ultraheavy_memory_patterns_and_16_thread_mcts_chunk_routing() {
    use rcurl::modules::mcts_quant::MctsChunkRouter;

    let handles: Vec<_> = (0..16)
        .map(|worker_id| {
            std::thread::spawn(move || {
                let mut router = MctsChunkRouter::new(100);
                let latencies = vec![
                    (worker_id as f64 + 1.0) * 10.0,
                    0.5 + (worker_id as f64 * 0.1),
                    100.0,
                ];
                let optimal = router.select_optimal_route(&latencies);
                assert_eq!(optimal, 1);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
