use clap::Parser;
use evelynn_cli::{
    cli::{Cli, Command},
    modules::ssh::handle_ssh,
};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Ssh(args)) => {
            if let Err(e) = handle_ssh(args.server_id).await {
                eprintln!("error > {}", e);
                std::process::exit(1);
            };
        }
        None => {
            eprintln!("No command provided. Use --help for usage information")
        }
    };
}
