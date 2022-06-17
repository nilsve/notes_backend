use crate::file_store::{FileBasedNote, FileBasedNoteKey, FileStore};
use crate::server::start;
use crate::store::*;

mod store;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut store = FileStore::new("./store").unwrap();

    let server = WebServer::new(store);

    server.start().await
}
