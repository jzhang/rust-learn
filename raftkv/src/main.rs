
use std::path::PathBuf;
use structopt::StructOpt;

extern crate hyper;

use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

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


fn kvstore(req: Request<Body>) -> Response<Body> {
    match (req.method(), req.uri().path()) {
        (&Method::GET,   "/key") => {
            Response::new(Body::from("get key"))
        },
        (&Method::POST, "/key") => {
            Response::new(Body::from("set key"))
        },
        (&Method::POST, "/join") => {
            Response::new(Body::from("join"))
        },
        _ => {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("not found"))
                .unwrap()
        },
    }
}

fn main() {
    pretty_env_logger::init();

    let opt = Opt::from_args();
    println!("{:?}", opt);

    let addr = opt.haddr.parse().unwrap();    
    
    let new_svc = || {
        service_fn_ok(kvstore)
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);   
}
