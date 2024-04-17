use std::{sync::Arc, thread};

use rocksdb::DB;

const N: usize = 100_000;

fn main() {
    let db = DB::open_default("/tmp/xyz").unwrap();
    let db = Arc::new(db);

    db.put(b"key", b"value1").unwrap();

    let db1 = db.clone();
    let j1 = thread::spawn(move || {
        for _ in 1..N {
            db1.put(b"key", b"value1").unwrap();
        }
    });

    let db2 = db.clone();
    let j2 = thread::spawn(move || {
        for _ in 1..N {
            db2.put(b"key", b"value2").unwrap();
        }
    });

    let j3 = thread::spawn(move || {
        for _ in 1..N {
            let result = match db.get(b"key") {
                Ok(Some(v)) => !(&v[..] != b"value1" && &v[..] != b"value2"),
                _ => false,
            };
            assert!(result);
        }
    });
    j1.join().unwrap();
    j2.join().unwrap();
    j3.join().unwrap();
}
