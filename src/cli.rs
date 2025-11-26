use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    name = "rusterm",
    version = env!("CARGO_PKG_VERSION"),
    about = "Share your terminal as a web application",
    arg_required_else_help = true,
    disable_help_flag = false,
    disable_version_flag = false
)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Serve the terminal as a web application
    Serve {
        /// Web application host
        #[arg(long, default_value = "0.0.0.0")]
        host: String,
        /// Web application port
        #[arg(long, default_value = "8080")]
        port: u16,
        /// Command to execute
        #[arg(long, required = true)]
        command: String,
        /// Timeout in seconds
        #[arg(long, default_value = None)]
        timeout: Option<u64>,
        /// Shutdown timeout in seconds (when timeout is reached, the command will be terminated, but if there is gracefull shutdown, it will wait for the command to finish)
        #[arg(long, default_value = None)]
        shutdown_timeout: Option<u64>,
        /// Port conflict resolution method
        #[arg(long, default_value = "connect")]
        port_conflict_resolve_method: PortConflictResolveMethod,
    },
    Daemon,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum PortConflictResolveMethod {
    /// Terminate the command immediately
    Terminate,
    /// Connect to the existing instance
    Connect,
}
