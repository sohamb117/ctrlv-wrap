mod args;
//use serde::{Deserialize, de::DeserializeOwned};
use clap::Clap;
use urlencoding::encode;
use ureq;
use args::Args;
use std::{
    ffi::OsStr,
    fs::File,
    io::Read,
};

fn main() {
    let options: Args = Args::parse();
    let mut input: Box<dyn Read> = if options.file.as_os_str() == OsStr::new("-") {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(&options.file).expect("Unable to open file"))
    };
    let mut content = String::new();
    input.read_to_string(&mut content);

    //let resp = ureq::post("https://https://api.ctrl-v.app/api").set("expiry", &options.expiry).set("content", &format!("print({})", content)).set("title", &options.title).set("language", &options.lang).set("password", &options.pass).call();
    //let resp = ureq::post("https://api.ctrl-v.app/api").set("title", "bruh").set("content", "print(\"abcdef\")").send_string("abc");

    let reqcontent: String = (format!("{}&{}",
            format!("content={}", encode(&content)),
            format!("title={}", options.title)));

    match ureq::post("https://api.ctrl-v.app/api").set("Content-Type", "application/x-www-form-urlencoded").send_string(&reqcontent)  {
        Ok(response) => {println!("{:?}", response.into_string())},
        Err(_) => {println!("Error encountered")},
    };
    //let header = resp?.header("hash").unwrap();
}
