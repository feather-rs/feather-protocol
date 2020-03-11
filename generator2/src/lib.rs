use std::env;
use std::fs::File;
use std::io::Read;

mod packets;

pub fn generate(output_dir: &str) -> anyhow::Result<()> {
    let input = include_str!(concat!(
        env!("OUT_DIR"),
        "/minecraft-data/data/pc/1.15.1/protocol.json"
    ));

    packets::generate(&[input], output_dir)?;

    Ok(())
}
