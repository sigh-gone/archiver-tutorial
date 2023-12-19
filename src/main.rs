mod archiver;
mod client;
mod html;

use crate::archiver::Archiver;
use crate::client::Client;
#[tokio::main]
async fn main() {
    let url = "https://en.wikipedia.org/wiki/Rust_(programming_language)";

    /*
    change these two lines if you want to use an absolute path, or create the directory "/Projects/archive_test"
     */
    //this will grab your home directory
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    //make sure this directory exists:
    let custom_path = "/Projects/archive_test";
    //this is the absolute path to your home directory and the added directories to the spot you want to
    //add your archives to.
    let new_dir = format!("{}{}", home_dir.to_str().unwrap(), custom_path);

    //create the client and pass it to the archiver
    let mut client = Client::new();
    let mut archiver = Archiver;
    let path = archiver.create_archive(&mut client, url, &new_dir).await;

    //path of the archived site
    println!("{:?}", path);
}
