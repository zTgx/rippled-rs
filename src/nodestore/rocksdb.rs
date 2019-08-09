use serde_json::{Value};

pub struct RocksDBFactory {}
impl RocksDBFactory {
    pub fn put(_value: &Value) {
        // use rocksdb::{DB, Options};
        // // NB: db is automatically closed at end of lifetime
        // let path = "./rippled-rs/db/rocksdb";
        // {
        //    let db = DB::open_default(path).unwrap();
        //    db.put(b"arg", value.to_string()).unwrap();
        //    match db.get(b"arg") {
        //        Ok(Some(value)) => println!("retrieved value {}", value.to_utf8().unwrap()),
        //        Ok(None) => println!("value not found"),
        //        Err(e) => println!("operational problem encountered: {}", e),
        //    }
        //    db.delete(b"my key").unwrap();
        // }
        // let _ = DB::destroy(&Options::default(), path);
    }
}
