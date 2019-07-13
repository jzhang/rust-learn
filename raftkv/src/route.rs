use hyperdrive::{body::Json, FromRequest};
use serde::Deserialize;

#[derive(FromRequest)]
pub enum Route {
    #[get("/key/{k}")]
    GetKey { k: String },

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
    #[post("/key/{k}")]
    DelKey { k: String },
}

#[derive(Deserialize)]
pub struct KvPair {
    pub k: String,
    pub v: String,
}

#[derive(Deserialize)]
pub struct JoinPeer {
    pub id: u64,
    pub addr: String,
}