use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Cursor, Read, Write};
use std::path::PathBuf;

use reqwest::blocking::{Client, Response};
use url::Url;

use crate::date::ChallengeDate;

pub trait Reader: Read + BufRead {}

impl<T> Reader for T where T: Read + BufRead {}

/// Tagged wrapper for the input data `BufReader`.
enum InputSource {
    Website(BufReader<Response>),
    Cache(BufReader<File>),
    Memory(BufReader<Cursor<String>>),
}

impl InputSource {
    /// Get a mutable reference to the inner `BufReader`.
    fn inner_mut(&mut self) -> Box<&mut dyn Reader> {
        match self {
            Self::Website(reader) => Box::new(reader),
            Self::Cache(reader) => Box::new(reader),
            Self::Memory(reader) => Box::new(reader),
        }
    }

    /// Take ownership over the inner `BufReader`.
    fn into_inner(self) -> Box<dyn Reader> {
        match self {
            Self::Website(reader) => Box::new(reader),
            Self::Cache(reader) => Box::new(reader),
            Self::Memory(reader) => Box::new(reader),
        }
    }

    /// Returns whether the variant is `Self::Website(_)`.
    fn is_website(&self) -> bool {
        matches!(self, InputSource::Website(_))
    }
}

/// Input data for a challenge.
pub struct Input {
    date: Option<ChallengeDate>,
    source: InputSource,
}

impl Input {
    /// Fetch the input for the given day's challenge from AoC's website.
    pub fn fetch(date: ChallengeDate) -> Self {
        let source = if let Some(file_reader) = read_cached(&date) {
            InputSource::Cache(file_reader)
        } else {
            let url = build_url(&date);
            let session = fetch_session_token();
            let response = Client::new()
                .get(url)
                .header("Cookie", format!("session={session}"))
                .send()
                .expect("failed to get AoC input data");

            let status = response.status();
            if !status.is_success() {
                panic!("non-200 status returned when fetching input data: {status}");
            }

            InputSource::Website(BufReader::new(response))
        };

        Self { date: Some(date), source }
    }

    /// Seed with the given input data.
    pub fn memory(data: impl Into<String>) -> Self {
        Self { date: None, source: InputSource::Memory(BufReader::new(Cursor::new(data.into()))) }
    }

    /// Reads the entirety of the buffer to a string.
    pub fn read_all(mut self) -> String {
        let mut buf = String::new();
        _ = self.source.inner_mut().read_to_string(&mut buf).unwrap();
        if self.source.is_website() {
            if let Some(date) = &self.date {
                if let Err(error) = cache_all(date, &buf) {
                    eprintln!("failed to cache input data: {:?}", error);
                }
            }
        }
        buf
    }

    /// Reads the input line-by-line.
    pub fn read_lines(self) -> impl Iterator<Item = String> {
        self.source.into_inner().lines().map(Result::unwrap)
    }
}

/// Returns the path to the file that caches input for the given challenge date.
fn cache_path(date: &ChallengeDate) -> PathBuf {
    let file_name = format!("{}-{}.txt", date.year, date.day);
    PathBuf::new().join(".cache").join(file_name)
}

/// Write data to cache file for the given challenge date.
fn cache_all(date: &ChallengeDate, data: &str) -> io::Result<()> {
    let file_path = cache_path(&date);
    let directory = file_path.parent().unwrap();
    fs::create_dir_all(directory)?;
    let mut file = File::create(file_path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

/// Read data from cache file for the given challenge date.
fn read_cached(date: &ChallengeDate) -> Option<BufReader<File>> {
    let file_path = cache_path(&date);
    if !file_path.exists() {
        return None;
    }
    let file = File::open(file_path).unwrap();
    Some(BufReader::new(file))
}

/// Builds the URL to get the input for the given day's challenge from AoC's
/// website.
fn build_url(date: &ChallengeDate) -> Url {
    const BASE_URL: &str = "https://adventofcode.com";

    let mut url = Url::parse(BASE_URL).expect("bad BASE_URL, this is a programmer error");
    url.path_segments_mut().unwrap().extend([&date.year.to_string(), "day", &date.day.to_string(), "input"]);

    url
}

/// Fetches the stored token in `session.txt`.
fn fetch_session_token() -> String {
    let session = std::fs::read_to_string("session.txt").expect("failed to read session.txt file");
    session.trim().to_owned()
}
