extern crate futures_state_stream;
extern crate serde;
extern crate tokio_core;
extern crate futures;
extern crate tokio_postgres as postgres;

use postgres::{Connection, TlsMode};
use tokio_core::reactor::Handle;

use futures::Future;
use futures_state_stream::StateStream;

pub const DATABASE_URL: &'static str = "postgres://localhost/testdb";

pub fn do_query(bar: &str, handle: &Handle) -> Box<Future<Item=(), Error=::postgres::Error>> {
    let f: Box<Future<Item=Connection, Error=::postgres::Error>> = Connection::connect(DATABASE_URL, TlsMode::None, handle);
    f.and_then(|conn| {
            conn.prepare("SELECT id FROM foo WHERE bar = $1").map_err(|(e, _)| e)
        })
        .and_then(|(s, c)| {
            c.query(&s, &[&bar])
                .for_each(|row| {
                        Ok(row.get(0))
                })
        })
        .into_box()
}

fn main() {
    println!("Hello, world!");
}
