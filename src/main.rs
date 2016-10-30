extern crate iron;
extern crate mount;
extern crate router;
extern crate rustc_serialize;
extern crate staticfile;

use std::path::Path;
use std::io::Read;
use std::sync::{Arc, Mutex};

use iron::prelude::*;
use iron::status;
use mount::Mount;
use router::Router;
use rustc_serialize::json;
use staticfile::Static;

#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    msg: String
}

fn main() {
    let greeting = Arc::new(Mutex::new(Greeting { msg: "Hello, World".to_string() }));
    let greeting_clone = greeting.clone();

    let mut router = Router::new();

    router.get("/get_file", get_file, "get_file");
    router.post("/post_file", post_file, "post_file");

    let mut mount = Mount::new();

    mount
        .mount("/api", router)
        .mount("/", Static::new(Path::new("static")));
    
    fn get_file(_: &mut Request) -> IronResult<Response> {
        let greeting = Greeting{msg: "hi".to_string()};
        let payload = json::encode(&greeting).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    // Receive a message by POST and play it back.
    fn post_file(request: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        Ok(Response::with(status::Ok))
    }

    Iron::new(mount).http("localhost:3000").unwrap();
}
