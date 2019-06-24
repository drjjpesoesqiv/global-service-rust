extern crate json;
extern crate iron;
extern crate redis;
extern crate router;

use iron::prelude::*;
use iron::{ status, typemap, BeforeMiddleware };
use router::Router;
use redis::Connection;
use redis::Commands;
use std::io::Read;

struct RedisMiddleware;
impl typemap::Key for RedisMiddleware { type Value = Connection; }

impl BeforeMiddleware for RedisMiddleware {
  fn before(&self, req: &mut Request) -> IronResult<()> {
    let client = redis::Client::open("redis://redis:6379").unwrap();
    let con = client.get_connection().unwrap();
    req.extensions.insert::<RedisMiddleware>(con);
    Ok(())
  }
}

fn main() {
  let mut set_chain = Chain::new(set_handler);
  set_chain.link_before(RedisMiddleware);

  let mut get_chain = Chain::new(get_handler);
  get_chain.link_before(RedisMiddleware);

  let mut router = Router::new();
  router.get("/", handler, "index");
  router.post("/set", set_chain, "set");
  router.post("/get", get_chain, "get");

  Iron::new(router).http("localhost:7777").unwrap();
}

fn get_handler(req: &mut Request) -> IronResult<Response> {
  let con = req.extensions.get::<RedisMiddleware>().unwrap();
  
  let mut payload = String::new();
  req.body.read_to_string(&mut payload).expect("Failed to read request body");
  let data = json::parse(&mut payload).unwrap();

  let option:redis::RedisResult<String> = con.get(data["optionKey"].to_string());
  let option = match option {
    Ok(option) => option,
    Err(e) => panic!(e)
  };
  Ok(Response::with((status::Ok, option)))
}

fn set_handler(req: &mut Request) -> IronResult<Response> {
  let con = req.extensions.get::<RedisMiddleware>().unwrap();

  let mut payload = String::new();
  req.body.read_to_string(&mut payload).expect("Failed to read request body");
  let data = json::parse(&mut payload).unwrap();

  let _:redis::RedisResult<String> = con.set(data["optionKey"].to_string(), data["optionVal"].to_string());
  Ok(Response::with((status::Ok, "ok")))
}

fn handler(_req: &mut Request) -> IronResult<Response> {
  Ok(Response::with((status::Ok, "ok")))
}
