use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    ///input file name
    #[arg(short, long)]
    pub input: String,

    ///output file name of the generated thumbnail
    #[arg(short, long)]
    pub output: String,
}
