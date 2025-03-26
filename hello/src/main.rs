fn main() {
    println!("Hello, world!");
}

#[test]
fn foo() {
    let dir = std::env::var_os("CARGO_MANIFEST_DIR").unwrap();
    eprintln!("manifest dir: {:?}", dir.to_string_lossy());
    assert!(std::path::Path::new(&dir).ends_with(&"hello"));
}
