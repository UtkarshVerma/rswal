use clap::Parser;

// TODO: -l and -c should be mutually exclusive
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub colorscheme: String,

    #[arg(short, long)]
    pub list_colorschemes: bool,

    #[arg(short, long)]
    pub alpha: Option<u8>,
}

pub fn parse_args() -> Args {
    Args::parse()
}
