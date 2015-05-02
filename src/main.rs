#![allow(dead_code)]
#![allow(unused_variables)]

extern crate type_printer;
mod the_printers;
mod lib;
use lib::http_day;

fn main() {
    the_printers::headers::print_title();

    let response = http_day::HttpResponse {
        status: 200,
        headers: "{}".to_string(),
        body: "{}".to_string(),
    };

    let result = http_day::response_validator(response);

    if result.is_ok() {
        println!("successful response");
    } else if result.is_err() {
        println!("bad response");
    }
}

