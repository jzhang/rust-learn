
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Set Unique Node ID
    #[structopt(long = "id")]
    id: String,
    /// Set HTTP bind Address
    #[structopt(long = "haddr", default_value = ":11000")]
    haddr: String,
    /// Set Raft bind Address
    #[structopt(long = "raddr", default_value = ":12000")]
    raddr: String,
    /// Set Join Address, If any
    #[structopt(long = "join")]
    join: String,
    /// Use in-memory storage for Raft
    #[structopt(long = "inmem", short = "m")]
    inmem: bool,
    // Raft DIR
    #[structopt(parse(from_os_str))]
    dir: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}