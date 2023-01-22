use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};


#[derive(Debug, Parser)]
#[command(name = "demo")]
#[command(about = "An example CLI demo", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
// 务必实现subcommand
#[derive(Subcommand, Debug)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Clone { remote: String, },
}

fn main() {
        
}
