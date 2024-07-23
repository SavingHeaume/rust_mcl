use sha1::{Digest, Sha1};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

pub mod asset;
pub mod library;
pub mod version;

pub trait Download {
    fn download(&self, game_dir: &Path) -> Result<(), Box<dyn std::error::Error>>;
}

pub fn get<T: reqwest::IntoUrl + Copy>(url: T) -> reqwest::Result<reqwest::blocking::Response> {
    let client = reqwest::blocking::Client::builder()
        .timeout(None)
        .build()
        .expect("构建客户端失败");

    let mut retries = 0;
    let max_retries = 5;

    loop {
        match client.get(url).send() {
            Ok(response) => return Ok(response),
            Err(_e) if retries < max_retries => {
                retries += 1;
                sleep(Duration::from_secs(2));
            }
            Err(e) => return Err(e),
        }
    }
}

pub fn sha1<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    let mut hasher = Sha1::new();

    file_hashing::get_hash_file(path, &mut hasher)
}

pub trait LibaryAllowed {
    fn allowed(&self) -> bool;
}
