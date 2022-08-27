use std::path::PathBuf;
use std::fmt;

use clap::Parser;

/// Structure representing the command-line arguments
#[derive(Parser)]
pub struct Cli {
    /// tag to create [mandatory], use a unique value
    pub tag_name: String,
    /// output folder to add the output file to, default: current folder
    #[clap(short = 'o', long = "output", default_value = ".", parse(from_os_str))]
    pub folder: PathBuf,
    /// main element to apply style to
    #[clap(long = "wrapper", default_value = "div")]
    pub wrapper: String,
    /// css classes to add to wrapper
    #[clap(short = 'c', long = "classes", multiple_values = true)]
    pub classes: Vec<String>,
}

impl fmt::Display for Cli {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!(f, "Tag: {}\n\tFolder: {:?}\n\tWrapper: {}\n\tClasses: {:?}", self.tag_name, self.folder, self.wrapper, self.classes)
    }
}