#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "sigmaos-free-file-sync", about = "FreeFileSync folder synchronization for SigmaOS")]
struct Cli { #[command(subcommand)] cmd: Commands }
#[derive(Subcommand)]
enum Commands { List, Status }
fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::List   => println!("sigmaos-free-file-sync: no entries configured."),
        Commands::Status => println!("sigmaos-free-file-sync: ready."),
    }
}
