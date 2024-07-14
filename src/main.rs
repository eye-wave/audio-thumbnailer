
mod error;
mod config;

#[cfg(feature="config_file")]
use config::Config;

pub use error::{Error,Result};

fn main() -> Result<()> {
    println!("Hello world!");
    
    #[cfg(feature="config_file")]
    {
        let config = Config::load()?;
        println!("{config:#?}");
    
    }

    Ok(())
}
