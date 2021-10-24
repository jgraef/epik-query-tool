use color_eyre::eyre::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct CommandLine {
    keywords: Vec<String>,
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    color_eyre::install()?;
    pretty_env_logger::init();

    let command = CommandLine::from_args();

    Ok(())
}
