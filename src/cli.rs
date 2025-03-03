use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "evelynn")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Ssh(SshArgs),
}

#[derive(Debug, Args)]
pub struct SshArgs {
    pub server_id: String,
}
