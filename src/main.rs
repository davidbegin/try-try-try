#![allow(dead_code)]
#![allow(unused_variables)]

extern crate type_printer;
mod the_printers;

fn main() {
    the_printers::headers::print_title();

    // I want to work with http requests
    //
    // we need 3 things
    //
    // status code
    //
    // headers
    //
    // body

    let response = HttpResponse {
        status: 200u16,
        headers: "{}".to_string(),
        body: "<html></html>".to_string()
    };

    // So I now need a function that takes an HttpResponse
    // and returns a Result
    //
    // and then I can branch on that result

    let result = response_validator(response);

    if result.is_ok() {
        println!("successful response");
    } else if result.is_err() {
        println!("bad response");
    }
}

struct HttpResponse {
    status: u16,
    headers: String,
    body: String,
}

fn response_validator(response: HttpResponse) -> Result<&'static str, &'static str> {
    Ok("durf")
}
