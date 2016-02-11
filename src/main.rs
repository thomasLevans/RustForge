extern crate rust_forge;
extern crate iron;
#[macro_use]
extern crate router;

use rust_forge::organic;
use iron::prelude::*;

fn main() {
    let router = router!(get "/organic" => organic::get_all);

    Iron::new(router).http("localhost:8080")
        .unwrap();
}
