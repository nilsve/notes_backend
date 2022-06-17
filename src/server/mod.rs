use std::marker::PhantomData;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::{Note, NoteKey, Store};

pub struct ServerData<K: NoteKey, T: Note<K>, S: Store<K, T>> {
    marker_k: PhantomData<K>,
    marker_t: PhantomData<T>,
    store: S,
}

impl<K: NoteKey, T: Note<K>, S: Store<K, T>> ServerData<K, T, S> {
    pub fn new(store: S) -> Self {
        ServerData {
            marker_k: Default::default(),
            marker_t: Default::default(),
            store,
        }
    }
}

pub async fn start<K: NoteKey, T: Note<K>, S: Store<K, T>> (store: S) -> std::io::Result<()> {
    HttpServer::new(|| {
        let data = ServerData::new(store);
        App::new()
            .service(list)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/")]
async fn list() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
