#![allow(dead_code)]
#![allow(unused_variables)]

// Httparty and Faraday combined!
//
// ruby everyone (applause)!
pub mod http_day {
    pub struct HttpResponse {
        pub status: u16,
        pub headers: String,
        pub body: String,
    }

    pub fn response_validator(response: HttpResponse) -> Result<&'static str, &'static str> {
        if response.status == 200 {
            Ok("nice response")
        } else if response.status == 404 {
            Err("Cool an Error")
        } else {
            Ok("Im like, whatever")
        }
    }
}

#[cfg(test)]
mod test {
    use super::http_day::*;

    #[test]
    fn it_returns_a_good_string_for_a_200_response() {
        let raw_response = HttpResponse {
            status  : 200,
            headers : "{}".to_string(),
            body    : "{}".to_string(),
        };

        assert_eq!(response_validator(raw_response).unwrap(), "nice response")
    }

    #[test]
    fn it_returns_a_bad_string_for_404_response() {
        let raw_response = HttpResponse {
            status: 404,
            headers: "{}".to_string(),
            body: "{}".to_string(),
        };

        let response = response_validator(raw_response);

        match response {
            Ok(v) => assert!(false),
            Err(e) => assert!(true, "Hey 404's are supposed to be Err's"),
        }
    }

}
