extern crate clap;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "lizard")]
#[clap(author = "hello@tlm.solutions")]
#[clap(version = "0.1.0")]
#[clap(about = "state server for tlms", long_about = None)]
pub struct Args {
    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    pub host: String,

    #[arg(long, default_value_t = 8080)]
    pub port: u16,

    #[arg(short, long, action)]
    pub swagger: bool,
}
