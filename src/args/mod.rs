pub mod aspect_ratio;
pub mod interpol;
pub mod size;

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

    ///quality of the generated image ( 0 = the worst, 10 = lossless )
    // #[arg(short, long)]
    // pub quality: Option<u8>,

    ///How to treat aspect ratio
    ///"crop" - crops edges to get 1:1 aspect ratio
    ///"stretch" - stretches image to 1:1
    ///"auto" - leaves as is
    #[arg(short, long, verbatim_doc_comment,default_value_t=String::from("auto"))]
    pub aspect_ratio: String,

    ///thumbnail size
    #[arg(short, long, default_value_t = 64)]
    pub size: u8,

    ///wave color
    #[arg(short, long, default_value_t = String::from("#ff0000"))]
    pub color: String,

    ///what interpolation algorithm to use
    ///"lanczos3", "gaussian", "nearest", "triangle",
    #[arg(short = 'p', long, verbatim_doc_comment,default_value_t = String::from("lanczos3"))]
    pub interpol: String,

    ///waveform width and height
    #[arg(long, verbatim_doc_comment,default_value_t=String::from("96:48"))]
    pub waveform_size: String,
}
