use serde::Deserialize;
use yamalgam_serde::{from_str, Deserializer};

#[test]
fn single_document_via_from_str() {
    let v: i64 = from_str("42").unwrap();
    assert_eq!(v, 42);
}

#[test]
fn from_str_errors_on_multiple_documents() {
    let result = from_str::<i64>("42\n---\n99");
    assert!(result.is_err());
    assert!(
        result.unwrap_err().to_string().contains("more than one document"),
        "error should mention multiple documents"
    );
}

#[derive(Debug, Deserialize, PartialEq)]
struct Item {
    name: String,
}

#[test]
fn documents_iterator() {
    let input = "---\nname: first\n---\nname: second\n---\nname: third";
    let docs: Vec<Item> = Deserializer::from_str(input)
        .documents::<Item>()
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(docs.len(), 3);
    assert_eq!(docs[0].name, "first");
    assert_eq!(docs[1].name, "second");
    assert_eq!(docs[2].name, "third");
}

#[test]
fn documents_mixed_scalars() {
    let input = "---\n42\n---\n99";
    let docs: Vec<i64> = Deserializer::from_str(input)
        .documents::<i64>()
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(docs, vec![42, 99]);
}

#[test]
fn documents_empty_stream() {
    let docs: Vec<i64> = Deserializer::from_str("")
        .documents::<i64>()
        .collect::<Result<_, _>>()
        .unwrap();
    assert!(docs.is_empty());
}

#[test]
fn documents_single_implicit() {
    let docs: Vec<i64> = Deserializer::from_str("42")
        .documents::<i64>()
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(docs, vec![42]);
}

#[test]
fn documents_with_doc_end_markers() {
    let input = "---\n42\n...\n---\n99\n...";
    let docs: Vec<i64> = Deserializer::from_str(input)
        .documents::<i64>()
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(docs, vec![42, 99]);
}

#[test]
fn documents_stops_on_error() {
    // An error in one document should stop iteration
    let input = "---\n42\n---\n[invalid";
    let results: Vec<Result<i64, _>> = Deserializer::from_str(input)
        .documents::<i64>()
        .collect();
    assert!(results[0].is_ok());
    assert_eq!(results[0].as_ref().unwrap(), &42);
    // The second document should fail (it's a sequence, not a scalar)
    assert!(results.len() >= 2);
    assert!(results[1].is_err());
}
