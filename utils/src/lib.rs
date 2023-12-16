use reqwest;
use dotenv;

pub fn fetch_input(year: u32, day: u32) -> String {
    dotenv::from_path("../key.env").unwrap();
    let token = std::env::var("TOKEN").unwrap();

    let client = reqwest::blocking::Client::new();
    let res = client
        .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
        .header(reqwest::header::COOKIE, format!("session={token}"))
        .send()
        .expect("something went wrong with sending request");

    res.text().expect("Could not convert to string")
}
