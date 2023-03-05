use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(name = "deploy")]
    Deploy {
        #[structopt(long = "rpi4")]
        rpi4: bool,

        #[structopt(parse(from_os_str))]
        config: PathBuf,
    },
}
