//! Handles generation of packet code from protocol JSON files.
//!
//! Code generation is handled in three phases:
//! * __Parsing__, in which the JSON files are parsed into an in-memory
//! representation of the various protocol versions.
//! * __Integration__, in which the parsed data is analyzed. This step handles e.g.
//! defining which packets need to be overridden for a given version and which can
//! be inherited from another.
//! * __Generation__, in which the parsed data and the integration results are used
//! to output Rust code.

mod generations;
mod integration;
mod parsing;

pub fn generate(
    input_versions: &[impl AsRef<str>],
    output_dir: impl AsRef<str>,
) -> anyhow::Result<()> {
    let protocol = parsing::parse(input_versions[0].as_ref())?;
    println!("{:#?}", protocol);
    Ok(())
}
