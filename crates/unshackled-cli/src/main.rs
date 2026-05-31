use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "unshackled")]
#[command(about = "Provider-neutral coding-agent harness")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Print version and build information.
    Doctor,
    /// Initialize project-local harness state.
    Init,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli.command.unwrap_or(Command::Doctor) {
        Command::Doctor => {
            println!("{} {}", unshackled_tui::APP_NAME, env!("CARGO_PKG_VERSION"));
            println!("status: scaffold");
        }
        Command::Init => {
            println!("initialized scaffold");
        }
    }

    Ok(())
}
