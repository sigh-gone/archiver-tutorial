use crate::client::Client;
use crate::html::HtmlRecord;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;
use std::fs::File;
use std::io::Write;
use url::Url;

pub struct Archiver;

impl Archiver {
    pub async fn create_archive(
        &mut self,
        client: &mut Client,
        url: &str,
        path: &str,
    ) -> Result<String, String> {
        //create record
        let mut record = client
            .fetch_html_record(url)
            .await
            .unwrap_or_else(|_| panic!("fetch_html_record failed \n url {}", url));
        //save the page
        match Archiver::save_page(&mut record, client, path).await {
            Ok(archive_path) => Ok(archive_path),
            Err(e) => Err(e),
        }
    }

    async fn save_page(
        html_record: &mut HtmlRecord,
        client: &mut Client,
        base_path: &str,
    ) -> Result<String, String> {
        //set up the directory to save the page in
        let url = Url::parse(&html_record.origin).expect("can't parse origin url");
        let host_name = url.host().expect("can't get host").to_string();
        let mut url_path = url.path().to_string();
        let mut base_path = base_path.to_string();
        if !base_path.ends_with('/') {
            base_path.push('/');
        }
        if !url_path.ends_with('/') {
            url_path.push('/');
        }
        let directory = format!(
            "{}{}{}{}",
            base_path, host_name, url_path, html_record.date_time
        );

        //create the directory
        fs::create_dir_all(&directory).map_err(|e| format!("Failed to create directory: {}", e))?;

        //Get images
        match html_record.get_image_links() {
            Ok(Some(t_image_links)) => {
                assert!(fs::create_dir_all(format!("{}/images", directory)).is_ok());
                for link in t_image_links {
                    if let Ok(image_bytes) = client.fetch_image_bytes(&link.1).await {
                        if let Ok(tmp_image) = image::load_from_memory(&image_bytes) {
                            let file_name = get_file_name(&link.1)
                                .unwrap_or_else(|| random_name_generator() + ".png");
                            let fqn = format!("{}/images/{}", directory, file_name);
                            let body_replacement_text = format!("./images/{}", file_name);

                            if (file_name.ends_with(".png")
                                && tmp_image
                                    .save_with_format(&fqn, image::ImageFormat::Png)
                                    .is_ok())
                                || (!file_name.ends_with(".png") && tmp_image.save(&fqn).is_ok())
                            {
                                html_record.body =
                                    html_record.body.replace(&link.0, &body_replacement_text);
                            }
                        }
                    }
                }
            }
            Ok(None) => {
                println!("no images for url: {}", url);
            }
            Err(e) => {
                println!("error {}", e)
            }
        }

        //Get css links
        match html_record.get_css_links() {
            Ok(Some(t_css_links)) => {
                assert!(fs::create_dir_all(format!("{}/css", directory)).is_ok());
                for link in t_css_links {
                    let file_name =
                        get_file_name(&link.1).unwrap_or_else(|| random_name_generator() + "css");
                    if let Ok(css) = client.fetch_string_resource(&link.1).await {
                        let fqn = format!("{}/css/{}", directory, file_name);
                        let mut file = File::create(&fqn).unwrap();
                        if file.write(css.as_bytes()).is_ok() {
                            let body_replacement_text = format!("./css/{}", file_name);
                            html_record.body =
                                html_record.body.replace(&link.0, &body_replacement_text);
                        } else {
                            println!("couldnt write css for url {}", &fqn);
                        }
                    }
                }
            }
            Ok(None) => {
                println!("no css for url: {}", url);
            }
            Err(e) => {
                println!("error for url {}\n error: {}", url, e)
            }
        }

        //get js links
        match html_record.get_js_links() {
            Ok(Some(t_js_links)) => {
                assert!(fs::create_dir(format!("{}/js", directory)).is_ok());
                for link in t_js_links {
                    let file_name =
                        get_file_name(&link.1).unwrap_or_else(|| random_name_generator() + "js");
                    if let Ok(js) = client.fetch_string_resource(&link.1).await {
                        let fqn = format!("{}/js/{}", directory, file_name);

                        if let Ok(mut output) = File::create(fqn) {
                            if output.write(js.as_bytes()).is_ok() {
                                let body_replacement_text = format!("./js/{}", file_name);
                                html_record.body =
                                    html_record.body.replace(&link.0, &body_replacement_text);
                            }
                        }
                    }
                }
            }
            Ok(None) => {
                println!("no js for url: {}", url);
            }
            Err(e) => {
                println!("error for url : {}\n error :{}", url, e);
            }
        }

        //write html to file
        let fqn_html = format!("{}/index.html", directory);
        let mut file_html = File::create(fqn_html.clone()).unwrap();
        if file_html.write(html_record.body.as_bytes()).is_ok() {
            Ok(fqn_html)
        } else {
            Err("error archiving site".to_string())
        }
    }
}

fn get_file_name(link: &str) -> Option<String> {
    let urlp = Url::parse(link).unwrap();
    if urlp.query().is_some() {
        return None;
    } else if let Some(segment_vector) = urlp.path_segments().map(|c| c.collect::<Vec<_>>()) {
        let segment_file = *segment_vector.last().unwrap();
        return Some(segment_file.to_string());
    }
    None
}

fn random_name_generator() -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    s
}
