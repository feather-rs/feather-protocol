fn main() {
    let base = concat!(env!("CARGO_MANIFEST_DIR"), "/..");

    let input = format!("{}/protocols/1.15.2.ron", base);
    let output = format!("{}/src/generated.rs", base);

    if let Err(e) = feather_protocol_generator::generate(&input, &output) {
        panic!("{:?}", e);
    }
}
