
use std::collections::HashMap;

struct KvStore {
    map: HashMap<Vec<u8>, Vec<u8>>, 
    inmem: bool,
}

type Error = std::io::Error;

impl KvStore {
    pub fn open(mem:bool) -> KvStore {
        KvStore {
            map: HashMap::new(),
            inmem: mem,
        }
    }
    pub fn put(&self, key: &[u8], data: &[u8]) -> Result<(),Error> {
        Ok(())
    }
    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        Some(vec![0u8,1,2,3])
    }
}

fn main() {  
    let kv = KvStore::open(false);

    let r = false;
    match r {
        true => {
            kv.put(b"abcd", b"efgh").unwrap();
            println!("true");
        }
        false => {
            kv.get(b"abcd");
            println!("false");
        }
    }
}