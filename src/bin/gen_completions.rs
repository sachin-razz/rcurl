#[path = "../cli.rs"]
mod cli;

use clap::CommandFactory;
use clap_complete::{generate_to, shells::*};
use cli::Cli;
use std::fs;

fn main() -> std::io::Result<()> {
    let mut cmd = Cli::command();
    let out_dir = "completions";
    fs::create_dir_all(out_dir)?;

    generate_to(Zsh, &mut cmd, "rcurl", out_dir)?;
    generate_to(Bash, &mut cmd, "rcurl", out_dir)?;
    generate_to(Fish, &mut cmd, "rcurl", out_dir)?;

    println!("Successfully generated Zsh, Bash, and Fish completions in completions/");
    Ok(())
}
