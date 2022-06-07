use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    pattern: String,
    #[clap(parse(from_os_str))]
    #[clap(short = 'p', long = "path")]
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()> {
    let args = Cli::parse();

    let filename = args.path.clone();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", filename.into_os_string().into_string().unwrap()))?;

    mi_terminal::find_matches(&content, &args.pattern, &mut std::io::stdout());

    return Ok(());
}
