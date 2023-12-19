use crate::archiver::Archiver;
use crate::client::Client;

mod archiver;
mod client;
mod html;
#[tokio::main]
async fn main() {
    let url = "https://en.wikipedia.org/wiki/Rust_(programming_language)";

    /*
    change these two lines if you want to use an absolute path, or create the directory "/Projects/archive_test"
     */
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let new_dir = format!("{}{}", home_dir.to_str().unwrap(), "/Projects/archive_test");

    let mut client = Client::new();
    let mut archiver = Archiver;
    let path = archiver.create_archive(&mut client, url, &new_dir).await;

    //path of the archived site
    println!("{:?}", path);
}
