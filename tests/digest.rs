use stable_digest::StableDigest;

#[test]
fn known_vector_empty() {
    // SHA-256("") = e3b0c442...
    assert_eq!(
        StableDigest::sha256("").as_str(),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
}

#[test]
fn known_vector_abc() {
    // SHA-256("abc") = ba7816bf...
    assert_eq!(
        StableDigest::sha256("abc").as_str(),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn bytes_and_str_material_agree() {
    let from_str = StableDigest::sha256("hello");
    let from_bytes = StableDigest::sha256(b"hello");
    let from_vec = StableDigest::sha256([104u8, 101, 108, 108, 111]);
    assert_eq!(from_str, from_bytes);
    assert_eq!(from_str, from_vec);
}

#[test]
fn deterministic_and_shaped() {
    let a = StableDigest::sha256("payload");
    let b = StableDigest::sha256("payload");
    assert_eq!(a, b);
    assert_eq!(a.as_str().len(), 64);
    assert!(a.as_str().chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
}

#[test]
fn different_material_differs() {
    assert_ne!(StableDigest::sha256("a"), StableDigest::sha256("b"));
}

#[test]
fn display_equals_as_str() {
    let d = StableDigest::sha256("x");
    assert_eq!(format!("{d}"), d.as_str());
}

#[test]
fn ord_is_lexicographic_over_hex() {
    // Ord derives from the inner String, i.e. lexicographic over the hex text.
    let a = StableDigest::sha256("");   // e3b0...
    let b = StableDigest::sha256("abc"); // ba78...
    assert!(b < a, "ba78… should sort before e3b0…");
}
