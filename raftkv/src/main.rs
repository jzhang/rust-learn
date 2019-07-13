use structopt::StructOpt;

use futures::prelude::*;
use http::{Response, StatusCode};
use hyper::{Body, Server};
use hyperdrive::{service::SyncService};
use std::net::{SocketAddr, ToSocketAddrs};

mod opt;
use self::opt::Opt;

mod route;
use self::route::Route;

mod engine;
use self::engine::SimpleDB;

fn main() {
    pretty_env_logger::init();

    let opt = Opt::from_args();
    println!("{:?}", opt);
    
    let db = SimpleDB::open(opt.dir, opt.inmem).unwrap();
    
    let srv =
        Server::bind(&(opt.haddr).parse().unwrap()).serve(SyncService::new(|route: Route| {
            match route {
                Route::GetKey { k } => { 
                    Response::new(Body::from(format!("get key = {}", k)))
                }
                Route::DelKey { k } => { 

                    Response::new(Body::from(format!("del key = {}", k)))     
                }
                Route::SetKey { data } => {
                    Response::new(Body::from(format!("set {} : {} ", data.k, data.v)))
                }
                Route::Join { data } => {
                    Response::new(Body::from(format!(
                        "nodeID = {}, addr = {}",
                        data.id, data.addr
                    )))}
            }
        }));
    tokio::run(srv.map_err(|e| {
        panic!("unexpected error: {}", e);
    }));
}