use clap::Parser;
use lazy_static::lazy_static;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Log file path
    #[arg(short, long, default_value = "atac.log")]
    pub log_file: String,

    /// Main application directory, containing collections
    #[arg(short, long)]
    pub directory: String,

    /// Avoid saving data to the collection file
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
}

lazy_static! {
    pub static ref ARGS: Args = Args::parse();
}