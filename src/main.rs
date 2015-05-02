#![allow(dead_code)]
#![allow(unused_variables)]

extern crate type_printer;
mod the_printers;

fn main() {
    the_printers::headers::print_title();

    // let response = HttpResponse {
    //     status: 200,
    //     headers: "{}".to_string(),
    //     body: "<html></html>".to_string()
    // };
    //
    // let result = response_validator(response);
    //
    // if result.is_ok() {
    //     println!("successful response");
    // } else if result.is_err() {
    //     println!("bad response");
    // }
}

