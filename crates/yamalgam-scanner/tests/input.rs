#![allow(missing_docs)]

use yamalgam_scanner::input::Input;

#[test]
fn utf8_no_bom() {
    let input = Input::from_bytes(b"hello: world").unwrap();
    assert_eq!(input.as_str(), "hello: world");
}

#[test]
fn utf8_with_bom() {
    let input = Input::from_bytes(b"\xEF\xBB\xBFhello: world").unwrap();
    assert_eq!(input.as_str(), "hello: world");
}

#[test]
fn utf16le_with_bom() {
    // "a: b" in UTF-16LE with BOM
    let mut bytes: Vec<u8> = vec![0xFF, 0xFE]; // BOM
    for c in "a: b".encode_utf16() {
        bytes.extend_from_slice(&c.to_le_bytes());
    }
    let input = Input::from_bytes(&bytes).unwrap();
    assert_eq!(input.as_str(), "a: b");
}

#[test]
fn utf16be_with_bom() {
    // "a: b" in UTF-16BE with BOM
    let mut bytes: Vec<u8> = vec![0xFE, 0xFF]; // BOM
    for c in "a: b".encode_utf16() {
        bytes.extend_from_slice(&c.to_be_bytes());
    }
    let input = Input::from_bytes(&bytes).unwrap();
    assert_eq!(input.as_str(), "a: b");
}

#[test]
fn utf32le_with_bom() {
    // "a: b" in UTF-32LE with BOM
    let mut bytes: Vec<u8> = vec![0xFF, 0xFE, 0x00, 0x00]; // BOM
    for c in "a: b".chars() {
        bytes.extend_from_slice(&(c as u32).to_le_bytes());
    }
    let input = Input::from_bytes(&bytes).unwrap();
    assert_eq!(input.as_str(), "a: b");
}

#[test]
fn utf32be_with_bom() {
    // "a: b" in UTF-32BE with BOM
    let mut bytes: Vec<u8> = vec![0x00, 0x00, 0xFE, 0xFF]; // BOM
    for c in "a: b".chars() {
        bytes.extend_from_slice(&(c as u32).to_be_bytes());
    }
    let input = Input::from_bytes(&bytes).unwrap();
    assert_eq!(input.as_str(), "a: b");
}

#[test]
fn from_reader_works() {
    let cursor = std::io::Cursor::new(b"hello: world");
    let input = Input::from_reader(cursor).unwrap();
    assert_eq!(input.as_str(), "hello: world");
}

#[test]
fn empty_input() {
    let input = Input::from_bytes(b"").unwrap();
    assert_eq!(input.as_str(), "");
}

#[test]
fn utf8_multibyte_preserved() {
    let input = Input::from_bytes("héllo: wörld".as_bytes()).unwrap();
    assert_eq!(input.as_str(), "héllo: wörld");
}
