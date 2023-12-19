use crate::html::HtmlRecord;
use bytes::Bytes;
use reqwest::header::USER_AGENT;
use url::Url;

const AGENT: &str =
    "Mozilla/5.0 (platform; rv:geckoversion) Gecko/geckotrail Firefox/firefoxversion";

pub(crate) struct Client {
    pub client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch_html_record(&mut self, url_str: &str) -> Result<HtmlRecord, reqwest::Error> {
        let url_parsed = Url::parse(url_str).expect("cannot parse");
        let res = self
            .client
            .get(url_parsed.as_str())
            .header(USER_AGENT, AGENT)
            .send()
            .await?;
        let body = res.text().await.expect("unable to parse html text");
        let body = replace_encoded_chars(body);
        let record: HtmlRecord = HtmlRecord::new(url_parsed.to_string(), body);

        Ok(record)
    }

    pub async fn fetch_image_bytes(&mut self, url_str: &str) -> Result<Bytes, String> {
        let url_parsed = Url::parse(url_str).expect("cannot parse");
        let res = self
            .client
            .get(url_parsed.as_str())
            .header(USER_AGENT, AGENT)
            .send()
            .await
            .map_err(|e| format!("fetch image bytes failed for url {}:\n {}", url_parsed, e))?;

        let status_value = res.status().as_u16();

        if status_value == 200 {
            let image_bytes = res.bytes().await.expect("unable to parse html text");
            Ok(image_bytes)
        } else {
            Err("status on image call not a 200 OKAY".to_string())
        }
    }

    pub async fn fetch_string_resource(&mut self, url_str: &str) -> Result<String, String> {
        let url_parsed = Url::parse(url_str).expect("cannot parse");
        let res = self
            .client
            .get(url_parsed.as_str())
            .header(USER_AGENT, AGENT)
            .send()
            .await
            .map_err(|e| format!("fetch string resource failed for url {}: {}", url_parsed, e))?;

        let status_value = res.status().as_u16();

        if status_value == 200 {
            let string_resource = res.text().await.expect("unable to parse html text");
            Ok(string_resource)
        } else {
            Err("status on css call not a 200 OKAY".to_string())
        }
    }
}
pub fn replace_encoded_chars(body: String) -> String {
    body.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&amp;", "&")
        .replace("&apos", "\'")
}
