use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
pub struct Opt {
    /// Set Unique Node ID
    #[structopt(long = "id")]
    pub id: u64,
    /// Set HTTP bind Address
    #[structopt(long = "haddr", default_value = ":11000")]
    pub haddr: String,
    /// Set Raft bind Address
    #[structopt(long = "raddr", default_value = ":12000")]
    pub raddr: String,
    /// Set Join Address, If any
    #[structopt(long = "join")]
    pub join: String,
    /// Use in-memory storage for Raft
    #[structopt(long = "inmem", short = "m")]
    pub inmem: bool,
    // Raft DIR
    #[structopt(parse(from_os_str))]
    pub dir: PathBuf,
}