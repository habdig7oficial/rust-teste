
use actix_web::{get, HttpResponse, Responder};

//use crate::hellow;
#[allow(non_camel_case_types)]
pub struct server_config {
    pub port: u16,
    pub addr: &'static str, //(&'static str, u16),
}

pub const SRCONF: server_config = server_config{
    port: 7777,
    addr: "127.0.0.1"
};


#[get("/cpu")]
pub async fn teste() -> impl Responder {
    let a = std::thread::available_parallelism().unwrap().get();
    println!("{}", a);
    HttpResponse::Ok().body("Hello from modules")
}


#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from index")
}
