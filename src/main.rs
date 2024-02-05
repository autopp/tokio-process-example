mod dump_stdout;

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
}

impl Commands {
    fn run(&self) -> Result<String, String> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let fut = match self {
            Commands::DumpStdout => dump_stdout::dump_stdout(),
        };
        rt.block_on(fut)
    }
}

fn main() {
    let args = Args::parse();

    println!("result: {:?}", args.command.run());
}
