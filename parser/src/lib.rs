use async_std::fs::File;
use async_std::io::prelude::WriteExt;
use async_std::io::ReadExt;
use async_std::path::Path;

mod compile;
mod parse;

pub async fn compile_def(input: impl AsRef<Path>, output: impl AsRef<Path>) -> anyhow::Result<()> {
    let mut input = File::open(input).await?;
    let mut output = File::create(output).await?;

    let mut in_str = String::new();
    input.read_to_string(&mut in_str).await?;
    in_str.retain(|char| char != '\n' && char != '\r');

    let ttree = parse::parse_str(&in_str)?;
    let compiled = compile::compile_tree(&ttree)?;

    let out_str = format!("{:#?}", compiled);
    output.write_all(out_str.as_bytes()).await?;

    Ok(())
}
