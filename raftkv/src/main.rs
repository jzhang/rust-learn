
use std::path::PathBuf;
use structopt::StructOpt;

use hyperdrive::{FromRequest, body::Json, service::SyncService};
use hyper::{Server, Body};
use http::{Response};
use futures::prelude::*;
use serde::Deserialize;

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

#[derive(FromRequest)]
enum Route { 
    #[get("/key/{key}")]
    GetKey {
        key: String,
    },

    #[post("/key")]
    SetKey {
        #[body]
        data: Json<KvPair>,
    },
    #[post("/join")]
    Join {
        #[body]
        data: Json<JoinPeer>,
    },
}

#[derive(Deserialize)]
struct KvPair {
    key: String,
    val: String,
}

#[derive(Deserialize)]
struct JoinPeer {
    id: String,
    addr: String,
}

fn main() {
    pretty_env_logger::init();

    let opt = Opt::from_args();
    println!("{:?}", opt);

    let srv = Server::bind(&(opt.haddr).parse().unwrap())
        .serve(SyncService::new(|route: Route| {
        
        match route {
            Route::GetKey { key } => {
                Response::new(Body::from(format!("key is #{}", key)))
            }
            Route::SetKey { data } => {
                Response::new(Body::from(format!("#{} : #{} ", data.key, data.val )))
            }
            Route::Join { data } => {
                Response::new(Body::from(format!("nodeID = #{}, addr = #{}", data.id, data.addr)))
            }
        }
    }));
    
    tokio::run(srv.map_err(|e| {
        panic!("unexpected error: {}", e);
    }));

}
