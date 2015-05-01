#![allow(dead_code)]
 #![allow(unused_variables)]

extern crate type_printer;

fn main() {
    headers::print_title();

    // Reuslt is an Enum, Sum Type, Tagged Variant, Tagged Union, some other names?!?!
    //
    // I'm not sure, because this is my time encountering any of those words before,
    // other than Enum, which seems to be used for a lot of different meanings.
    //
    // so it take 2 Generics
    //
    // T and E
    //
    // and each...one...goes...to...another...enum??
    //
    // I am really not sure
    // enum Result<T, E> {
    //     Ok(T)
    //     Err(E)
    // }
    //...moving on


    #[derive(Debug)]

    // Ok so I declare an enum Version to have 2 type
    // Version1, and Version2 ... which already exists?!
    enum Version { Version1, Version2 }


    // detour, I need a refresher, what is &[u8]?
    //
    // and array
    //
    // a vector?
    //
    // well a size wasn't declared making me thinking

    // this method takes an array
    //
    // and returns a Result, that has 1 of 2 Generics
    //
    // either a Version enum
    // or a borrowed static string slice
    fn parse_version(header:  &[u8])-> Result<Version, &'static str> {
        // println!("here is the header get(0): {:?}", header.get(0));

        // So whatever type we have, has get on it?
        //
        // and where does None come from
        match header.get(0) {
            None     => Err("invalid header length"),
            Some(&1) => Ok(Version::Version1),
            Some(&2) => Ok(Version::Version2),
            Some(_)  => Err("invalid Version"),
        }
    }

    // what does Some(_) match?!? anything not matched duhhhhhhh

    let version = parse_version(&[9]);

    // match version {
    //     Ok(v) => println!("working with version: {:?}", v),
    //     Err(e) => println!("error parsing header: {:?}", e),
    // }


    // so check this good_result out
}


mod headers {
    pub fn print_title() {
        println!("\nTry Try Try");
        println!("===========\n");
    }
}
