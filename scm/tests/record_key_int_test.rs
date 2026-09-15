//! Integration tests for [`oltp_pattern::RecordKey`].

use oltp_pattern::RecordKey;

/// @covers: RecordKey::new/as_str — round-trips the input string exactly.
#[test]
fn test_as_str_returns_the_constructed_value() {
    let key = RecordKey::new("order-42");
    assert_eq!(key.as_str(), "order-42");
}

/// @covers: RecordKey::Display — produces the same text as `as_str`.
#[test]
fn test_display_matches_as_str() {
    let key = RecordKey::new("customer-7");
    assert_eq!(key.to_string(), key.as_str());
}

/// @covers: RecordKey::Eq/Hash — keys built from equal strings are equal (so
/// they collide correctly as HashMap keys).
#[test]
fn test_equal_strings_produce_equal_keys() {
    assert_eq!(RecordKey::new("a"), RecordKey::new("a"));
    assert_ne!(RecordKey::new("a"), RecordKey::new("b"));
}
