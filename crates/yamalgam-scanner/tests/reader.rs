#![allow(missing_docs)]

use yamalgam_core::Mark;
use yamalgam_scanner::reader::Reader;

#[test]
fn peek_returns_first_char() {
    let reader = Reader::new("hello");
    assert_eq!(reader.peek(), Some('h'));
}

#[test]
fn advance_moves_forward() {
    let mut reader = Reader::new("hi");
    assert_eq!(reader.advance(), Some('h'));
    assert_eq!(reader.advance(), Some('i'));
    assert_eq!(reader.advance(), None);
}

#[test]
fn tracks_line_and_column() {
    let mut reader = Reader::new("ab\ncd");
    reader.advance(); // a
    reader.advance(); // b
    assert_eq!(
        reader.mark(),
        Mark {
            line: 0,
            column: 2,
            offset: 2
        }
    );
    reader.advance(); // \n
    assert_eq!(
        reader.mark(),
        Mark {
            line: 1,
            column: 0,
            offset: 3
        }
    );
    reader.advance(); // c
    assert_eq!(
        reader.mark(),
        Mark {
            line: 1,
            column: 1,
            offset: 4
        }
    );
}

#[test]
fn peek_at_lookahead() {
    let reader = Reader::new("abc");
    assert_eq!(reader.peek_at(0), Some('a'));
    assert_eq!(reader.peek_at(1), Some('b'));
    assert_eq!(reader.peek_at(2), Some('c'));
    assert_eq!(reader.peek_at(3), None);
}

#[test]
fn is_eof() {
    let mut reader = Reader::new("a");
    assert!(!reader.is_eof());
    reader.advance();
    assert!(reader.is_eof());
}

#[test]
fn empty_input() {
    let reader = Reader::new("");
    assert!(reader.is_eof());
    assert_eq!(reader.peek(), None);
}

#[test]
fn multibyte_chars_track_correctly() {
    let mut reader = Reader::new("é"); // 2 bytes in UTF-8
    assert_eq!(reader.peek(), Some('é'));
    reader.advance();
    // offset should be 2 (byte offset), column should be 1 (char offset)
    assert_eq!(reader.mark().offset, 2);
    assert_eq!(reader.mark().column, 1);
}

#[test]
fn cr_lf_is_single_newline() {
    let mut reader = Reader::new("a\r\nb");
    reader.advance(); // a
    reader.advance(); // \r\n (treated as single newline)
    assert_eq!(
        reader.mark(),
        Mark {
            line: 1,
            column: 0,
            offset: 3
        }
    );
}
