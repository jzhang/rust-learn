
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
    pub fn put(&mut self, key: &[u8], data: &[u8]) -> Result<(),Error> {
        self.map.insert(key.to_owned(), data.to_owned());
        Ok(())
    }
    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
       match self.map.get(&key.to_owned()) {
           Some(v) => Some(v.to_vec()),
           None => None
       }
    }
}

fn main() {  
    let mut kv = KvStore::open(false);

    let r = false;
    match r {
        true => {
            kv.put(b"abcd", b"efgh").unwrap();
            println!("true");
        }
        false => {
            kv.put(b"abcd", b"efgh").unwrap();
            let r = kv.get(b"abcd");
            match r {
                Some(v) => println!("{:?}", String::from_utf8(v).unwrap()),
                None => println!("None")
            }
        }
    }
}