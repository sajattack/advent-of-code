extern crate html2text;
extern crate reqwest;

use std::boxed::Box;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io::BufReader;
use std::result;

use reqwest::header::COOKIE;

// Error macro shamelessly stolen from BurntSushi
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

type Result<T> = result::Result<T, Box<Error>>;

fn get_cookie() -> Result<String> {
    let args: Vec<String> = env::args().collect();
    let cookie: String;
    if args.len() != 2 {
        return err!(
            "You must specify your adventofcode session cookie as the first and only argument"
        );
    } else {
        cookie = args[1].clone();
    }
    Ok(cookie)
}

pub fn get_input(day: u8) -> Result<String> {
    let cookie = get_cookie().unwrap();
    let client = reqwest::Client::new();
    let input = client
        .get(&format!("https://adventofcode.com/2018/day/{}/input", day))
        .header(COOKIE, format!("session={}", cookie).as_str())
        .send()?
        .text()?;
    Ok(input)
}

pub fn submit_answer(day: u8, level: u8, answer: String) -> Result<String> {
    let cookie = get_cookie().unwrap();
    let client = reqwest::Client::new();
    let mut data = HashMap::new();
    let level_string = format!("{}", level);
    data.insert("level", level_string.as_str());
    data.insert("answer", answer.as_str());
    let html = client
        .post(&format!("https://adventofcode.com/2018/day/{}/answer", day))
        .header(COOKIE, format!("session={}", cookie).as_str())
        .form(&data)
        .send()?
        .text()?;
    let plaintext = html2text::from_read(BufReader::new(html.as_bytes()), 80);
    Ok(plaintext)
}
