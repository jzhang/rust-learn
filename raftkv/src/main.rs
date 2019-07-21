extern crate actix_web;
extern crate serde_derive;

use structopt::StructOpt;

mod opt;
use self::opt::Opt;

use actix_web::{web, App, middleware, HttpRequest, HttpResponse, HttpServer, Responder};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct KVP {
    k: String,
    v: String, 
}

#[derive(Debug, Serialize, Deserialize)]
struct JOIN {
    id: u64,
    addr: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

fn index(item: web::Json<MyObj>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0) // <- send response
}

fn get_key(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Get key {}!", &name)
}

fn del_key(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Del Key {}!", &name)
}

fn set_key(item: web::Json<KVP>) -> HttpResponse {
    println!("model: {:?}", item);

    HttpResponse::Ok().json(item.0)
}

fn join(item: web::Json<JOIN>) -> HttpResponse {
    println!("model: {:?}", item);
    HttpResponse::Ok().json(item.0)
}

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    pretty_env_logger::init();

    let opt = Opt::from_args();
    println!("{:?}", opt);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/key/{name}", web::get().to(get_key))
            .route("/key/{name}", web::delete().to(del_key))
            .route("/key",        web::post().to(set_key))
            .route("/join",       web::post().to(join))
            .route("/",           web::post().to(index))
    })
    .bind(opt.haddr)
    .expect("Can not bind to address")
    .run()
    .unwrap();
}