extern crate structopt;

use structopt::StructOpt;


#[derive(StructOpt, Debug)]
#[structopt(name = "cli_tool", about = "A CLI tool for interacting with the API")]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(about = "Ask the server to generate text data, continuing a given prompt")]
    Run {
        #[structopt(long, default_value="0.0.0.0:9911", help = "Comma-separated list of colon-separated IP(s) and port(s) to listen on")]
        bindings: String,
    },
}

async fn listen(bindings: &String) {
    println!("Starting up! Binding to:");

    for binding in bindings.split(",") {
        println!("    {}", binding)
    }
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();

    match args.cmd {
        Command::Run { bindings } => {
            listen(&bindings).await;
        }
    }
}
