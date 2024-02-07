mod dump_stdout;
mod signal;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command()]
    DumpStdout,
    #[command()]
    Signal,
}

impl Commands {
    async fn run(&self) -> Result<String, String> {
        match self {
            Commands::DumpStdout => dump_stdout::dump_stdout().await,
            Commands::Signal => signal::signal().await,
        }
    }
}

fn main() {
    let args = Args::parse();

    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(args.command.run());

    println!("result: {:?}", result);
}
