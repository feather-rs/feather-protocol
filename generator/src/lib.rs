use async_std::fs::File;
use async_std::io::prelude::WriteExt;
use async_std::io::ReadExt;
use async_std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
use walkdir::WalkDir;

type Version = String;

mod compile;
mod generate;
mod integrate;
mod parse;
mod universalize;

pub async fn compile_def(input_dir: &str, output_dir: &str) -> anyhow::Result<()> {
    // Compile each version definition file
    let mut packet_definitions = vec![];
    for entry in WalkDir::new(input_dir) {
        let entry = entry?;
        let entry = entry.path();

        if !entry.as_os_str().to_str().unwrap().ends_with(".def") {
            continue;
        }

        let mut file = File::open(entry).await?;
        let mut input = String::new();
        file.read_to_string(&mut input).await?;

        let token_tree = parse::parse_str(&input)?;
        let defs = compile::compile_tree(&token_tree)?;
        packet_definitions.push((defs, entry.to_str().unwrap().to_owned()));
    }

    // Perform integration across versions
    let integration = integrate::integrate(
        &packet_definitions
            .iter()
            .cloned()
            .map(|(def, _)| def)
            .collect::<Vec<_>>(),
    )?;

    // Generate code for each version
    for (defs, input_file) in &packet_definitions {
        let code = generate::generate_packet_code(defs, input_file.as_str(), &integration)?;

        let mut path = PathBuf::from_str(output_dir)?;
        path.push(format!("{}.rs", defs.version.to_lowercase()));

        let mut file = File::create(&path).await?;
        file.write_all("// This is GENERATED CODE. Please do not edit.\n".as_bytes())
            .await?;
        file.write_all(code.as_bytes()).await?;
        file.flush().await?;

        // Run rustfmt
        Command::new("rustfmt")
            .arg(path.to_string_lossy().to_string())
            .output()
            .expect("running rustfmt failed");
    }

    // Universalize
    let path = format!("{}/send.rs", output_dir);
    let universal = universalize::universalize(
        &packet_definitions
            .iter()
            .cloned()
            .map(|(def, _)| def)
            .collect::<Vec<_>>(),
        &integration,
    )?;
    let mut file = File::create(&path).await?;

    file.write_all(universal.as_bytes()).await?;
    file.flush().await?;

    Command::new("rustfmt")
        .arg(&path)
        .output()
        .expect("running rustfmt failed");

    Ok(())
}
