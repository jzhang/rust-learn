use std::path::PathBuf;
use std::io;
use std::net::SocketAddr;

pub type Error = io::Error;

pub struct SimpleDB {
    dbpath: PathBuf,
    inmem: bool,
}

impl SimpleDB {
    pub fn open(path: PathBuf, mem: bool) -> Result<SimpleDB, Error> {
        let db = SimpleDB {
            dbpath: path,
            inmem: mem
        };
        Ok(db)
    }
    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        Some(key.to_owned())
    }

    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<(), Error> {
        Ok(())
    }

    pub fn delete(&self, key: &[u8]) -> Result<(), Error> {
        Ok(())
    }

    pub fn join(&self, id: u64, addr: String) -> Result<(), Error> {
        Ok(())
    }
}
