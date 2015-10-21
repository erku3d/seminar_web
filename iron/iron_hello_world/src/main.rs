extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
	println!("On 3000");
        Ok(Response::with((status::Ok, "Hello World!")))
    }

    Iron::new(hello_world).http("localhost:3000").unwrap();
    println!("On 3000");
}

