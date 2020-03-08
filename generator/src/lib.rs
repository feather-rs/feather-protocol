use async_std::fs::File;
use async_std::io::prelude::WriteExt;
use async_std::io::ReadExt;
use async_std::path::Path;
use std::process::Command;

mod compile;
mod generate;
mod parse;

pub async fn compile_def(input_path: &str, output_path: &str) -> anyhow::Result<()> {
    let mut input = File::open(input_path).await?;
    let mut output = File::create(output_path).await?;

    let mut in_str = String::new();
    input.read_to_string(&mut in_str).await?;
    in_str.retain(|char| char != '\n' && char != '\r');

    let ttree = parse::parse_str(&in_str)?;
    let compiled = compile::compile_tree(&ttree)?;
    let generated = generate::generate_packet_code(&compiled, input_path)?;

    output
        .write_all(&b"// This is GENERATED CODE. Do not edit.\n"[..])
        .await?;

    output.write_all(generated.as_bytes()).await?;
    output.flush().await?;

    // Run rustfmt
    Command::new("rustfmt")
        .arg(output_path)
        .output()
        .expect("running rustfmt failed");

    Ok(())
}
