use anyhow::bail;
use clap::{Parser, Subcommand};
use std::path::Path;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Generate,
}

fn main() -> anyhow::Result<()> {
    // from rtic's xtask (https://github.com/rtic-rs/rtic/blob/master/xtask/src/main.rs)
    let probably_running_from_repo_root = Path::new("./xtask").exists();
    if !probably_running_from_repo_root {
        bail!("xtasks can only be executed from the root of the `rtic` repository");
    }
    // end from

    let args = Args::parse();

    match &args.command {
        Command::Generate => xtask::generate::main(),
    };

    Ok(())
}
