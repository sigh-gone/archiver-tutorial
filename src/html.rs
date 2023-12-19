use chrono::Utc;
use lazy_static::lazy_static;
use regex::Regex;
use scraper::{Html, Selector};
use std::collections::HashSet;
use url::{ParseError, Url};

#[derive(Debug)]
pub struct HtmlRecord {
    pub origin: String,
    pub date_time: String,
    pub body: String,
    pub html: Html,
}

/*

HTML DOCUMENT

*/

impl HtmlRecord {
    pub fn new(origin: String, body: String) -> HtmlRecord {
        HtmlRecord {
            origin,
            date_time: Utc::now().format("%d-%m-%Y-%H:%M:%S").to_string(),
            html: Html::parse_document(&body),
            body,
        }
    }

    //the tuple returns the unparsed string in the 0's spot
    //returns the parsed link in the 1's spot
    pub fn get_image_links(&self) -> Result<Option<HashSet<(String, String)>>, String> {
        lazy_static! {
            static ref RE3: Regex = Regex::new(r";base64,").unwrap();
        }
        let mut ret_vec: Vec<(String, String)> = vec![];
        let selector = Selector::parse("img").unwrap();
        for element in self.html.select(&selector) {
            match element.value().attr("src") {
                Some(link) => {
                    if Url::parse(link) == Err(ParseError::RelativeUrlWithoutBase) {
                        let base = Url::parse(&self.origin)
                            .expect("get css links, origin could not be parsed");
                        let plink = base
                            .join(link)
                            .expect("css links, could not join")
                            .to_string();
                        ret_vec.push((link.to_string(), plink.to_string()))
                    } else if RE3.is_match(link) {
                        continue;
                    } else if let Ok(parsed_link) = Url::parse(link) {
                        ret_vec.push((link.to_string(), parsed_link.to_string()));
                    }
                }
                None => continue,
            };
        }

        let link_hashset: HashSet<(String, String)> = ret_vec.iter().cloned().collect();

        if link_hashset.is_empty() {
            Ok(None)
        } else {
            Ok(Some(link_hashset))
        }
    }

    pub fn get_css_links(&self) -> Result<Option<HashSet<(String, String)>>, String> {
        let mut ret_vec: Vec<(String, String)> = vec![];
        let selector = Selector::parse("link").unwrap();
        for element in self.html.select(&selector) {
            if element.value().attr("rel").unwrap() == "stylesheet" {
                match element.value().attr("href") {
                    Some(link) => {
                        //take care of relative links here
                        if Url::parse(link) == Err(ParseError::RelativeUrlWithoutBase) {
                            let base = Url::parse(&self.origin)
                                .expect("get css links, origin could not be parsed");
                            let plink = base
                                .join(link)
                                .expect("css links, could not join")
                                .to_string();
                            ret_vec.push((link.to_string(), plink.to_string()))
                        } else if let Ok(parsed_link) = Url::parse(link) {
                            ret_vec.push((link.to_string(), parsed_link.to_string()));
                        }
                    }
                    None => continue,
                };
            }
        }

        let link_hashset: HashSet<(String, String)> = ret_vec.iter().cloned().collect();

        if link_hashset.is_empty() {
            Ok(None)
        } else {
            Ok(Some(link_hashset))
        }
    }

    pub fn get_js_links(&self) -> Result<Option<HashSet<(String, String)>>, String> {
        let mut ret_vec: Vec<(String, String)> = vec![];
        let selector = Selector::parse("script").unwrap();
        for element in self.html.select(&selector) {
            match element.value().attr("src") {
                Some(link) => {
                    if Url::parse(link) == Err(ParseError::RelativeUrlWithoutBase) {
                        let base = Url::parse(&self.origin)
                            .expect("get js links, origin could not be parsed ");
                        let plink = base
                            .join(link)
                            .expect("js links, could not join")
                            .to_string();
                        ret_vec.push((link.to_string(), plink.to_string()))
                    } else if let Ok(parsed_link) = Url::parse(link) {
                        ret_vec.push((link.to_string(), parsed_link.to_string()));
                    }
                }
                None => continue,
            };
        }

        let link_hashset: HashSet<(String, String)> = ret_vec.iter().cloned().collect();

        if link_hashset.is_empty() {
            Ok(None)
        } else {
            Ok(Some(link_hashset))
        }
    }
}
