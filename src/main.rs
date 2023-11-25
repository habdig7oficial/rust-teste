use std::thread::available_parallelism;
use actix_web::{App, HttpServer};
use crate::config::SRCONF;

mod config;
0
#[actix_web::main]
#0
async fn main() -> std::io::Result<()> {
    return HttpServer::new(|| {

        #[]
        let mut app = App::new();


        for i in [config::teste] {
            app.service(i);
        }


        return app;
    })
        .bind((SRCONF.addr, SRCONF.port))?
        .workers(available_parallelism().unwrap().get())
        .run()
        .await;
}