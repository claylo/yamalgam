#![allow(missing_docs)]

use std::io::Cursor;

use serde::Deserialize;
use yamalgam_serde::from_reader;

#[derive(Debug, Deserialize, PartialEq)]
struct Item {
    name: String,
}

#[test]
fn deserialize_from_reader() {
    let data = b"name: yamalgam";
    let item: Item = from_reader(Cursor::new(data)).unwrap();
    assert_eq!(
        item,
        Item {
            name: "yamalgam".into()
        }
    );
}

#[test]
fn deserialize_from_reader_multi_line() {
    let data = b"host: localhost\nport: 8080";
    let item: std::collections::HashMap<String, String> = from_reader(Cursor::new(data)).unwrap();
    assert_eq!(item["host"], "localhost");
    assert_eq!(item["port"], "8080");
}

#[test]
fn deserialize_from_reader_sequence() {
    let data = b"- one\n- two\n- three";
    let items: Vec<String> = from_reader(Cursor::new(data)).unwrap();
    assert_eq!(items, vec!["one", "two", "three"]);
}

#[test]
fn deserialize_from_reader_empty() {
    let data = b"";
    // Empty input deserializes as unit / null.
    let result: Option<String> = from_reader(Cursor::new(data)).unwrap();
    assert_eq!(result, None);
}
