#![allow(missing_docs)]

use serde::Deserialize;
use yamalgam_serde::from_str;

#[derive(Debug, Deserialize, PartialEq)]
struct A {
    x: i64,
}

#[derive(Debug, Deserialize, PartialEq)]
struct B {
    y: String,
}

#[test]
fn from_str_erased_different_types() {
    let a: A = from_str("x: 42").unwrap();
    assert_eq!(a, A { x: 42 });
    let b: B = from_str("y: hello").unwrap();
    assert_eq!(b, B { y: "hello".into() });
}
