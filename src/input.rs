use std::io::{BufReader, Read};

use reqwest::blocking::{Client, Response};
use url::Url;

use crate::date::ChallengeDate;

/// Input data for a challenge.
pub struct Input(BufReader<Response>);

impl Input {
    /// Fetch the input for the given day's challenge from AoC's website.
    pub fn fetch(date: ChallengeDate) -> Self {
        let url = build_url(date);
        let session = fetch_session_token();
        let response = Client::new()
            .get(url)
            .header("Cookie", format!("session={session}"))
            .send()
            .expect("failed to get AoC input data");

        Self(BufReader::new(response))
    }

    /// Get a reference to the inner `BufReader`.
    pub fn reader(&mut self) -> &mut BufReader<Response> {
        &mut self.0
    }

    /// Reads the entirety of the buffer to a string.
    pub fn read_all(mut self) -> String {
        let mut buf = String::new();
        _ = self.reader().read_to_string(&mut buf).unwrap();
        buf
    }
}

/// Builds the URL to get the input for the given day's challenge from AoC's website.
fn build_url(date: ChallengeDate) -> Url {
    const BASE_URL: &str = "https://adventofcode.com";

    let mut url = Url::parse(BASE_URL).expect("bad BASE_URL, this is a programmer error");
    url.path_segments_mut().unwrap().extend([
        &date.year.to_string(),
        "day",
        &date.day.to_string(),
        "input",
    ]);

    url
}

/// Fetches the stored token in `session.txt`.
fn fetch_session_token() -> String {
    let session = std::fs::read_to_string("session.txt").expect("failed to read session.txt file");
    session.trim().to_owned()
}
