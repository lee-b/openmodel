extern crate structopt;

use structopt::StructOpt;
use std::process;

#[derive(StructOpt, Debug)]
#[structopt(name = "cli_tool", about = "A CLI tool for interacting with the API")]
struct Cli {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(about = "Ask the server to generate text data, continuing a given prompt")]
    GenText {
        prompt: String,

        #[structopt(long="api-url", env = "OPENMODEL_API_URL", help = "API server URL")]
        api_url: String,

        #[structopt(long="api-token", env = "OPENMODEL_API_TOKEN", help = "Token")]
        api_token: String,
    },
}

async fn gen_text(api_url: String, api_token: String, prompt: String) {
    // Implement data retrieval logic here
}

#[tokio::main]
async fn main() {
    let args = Cli::from_args();

    match args.cmd {
        Command::GenText { api_url, api_token, prompt } => {
            gen_text(api_url, api_token, prompt).await;
        }
    }
}
