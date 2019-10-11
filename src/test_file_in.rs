use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::{Read, Write};

pub fn read_file() {
    let path = Path::new("hello.txt");
    let display = path.display();

    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("could't read {}: {}", display, Error::description(&why)),
        Ok(_) => print!("{} contains:\n {}", display, s),
    }
}


static OUT_CONTENT: &'static str = "It's friday today, but still be working tomorrow! 因为明天要补班！";

pub fn out_file() {

    let path = Path::new("say.txt");
    let display = path.display();

    let mut file = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    match file.write_all(OUT_CONTENT.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, Error::description(&why)),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
