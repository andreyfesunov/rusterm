use clap::Parser;
use uuid::Uuid;

mod cli;

fn main() {
    let cli = cli::Cli::parse();
    let instance_id = Uuid::new_v4();

    println!("{:?}", cli);
    print!("Instance ID: {}", instance_id);
}
