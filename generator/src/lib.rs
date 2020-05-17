use anyhow::Context;
use std::{
    convert::TryFrom,
    fs::File,
    io::{Read, Write},
    process::Command,
};

mod generate;
mod intermediate;
mod model;

/// Parses the input protocol file and writes
/// out generated code for the protocol.
pub fn generate(input: &str, output: &str) -> anyhow::Result<()> {
    let model = load_model(input).context("failed to load model")?;
    let intermediate = intermediate::Root::try_from(model)
        .context("failed to create intermediate representation")?;

    let generated = generate::generate(&intermediate);

    let mut file = File::create(output)
        .with_context(|| format!("failed to create output file `{}`", output))?;
    file.write_all(generated.as_bytes())?;
    file.flush()?;

    if !Command::new("rustfmt").arg(output).status()?.success() {
        anyhow::bail!("rustfmt did not esit successfully");
    }

    Ok(())
}

fn load_model(input: &str) -> anyhow::Result<model::Root<'static>> {
    let mut file =
        File::open(input).with_context(|| format!("failed to open input file `{}`", input))?;

    let mut s = String::new();
    file.read_to_string(&mut s)?;

    // Oh yes... let's bypass the borrow checker, shall we?
    let s = Box::leak(s.into_boxed_str());

    let parsed = ron::from_str(s).context("failed to parse protocol file")?;

    Ok(parsed)
}
