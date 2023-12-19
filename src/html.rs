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
        //checks for base64 images
        lazy_static! {
            static ref RE3: Regex = Regex::new(r";base64,").unwrap();
        }
        let mut link_hashset: HashSet<(String, String)> = HashSet::new();

        //select image tags
        let selector = Selector::parse("img").unwrap();

        //loop through img tags
        for element in self.html.select(&selector) {
            //grab the source attribute of the tag
            match element.value().attr("src") {
                //if we have a link
                Some(link) => {
                    //see if a relative link
                    if Url::parse(link) == Err(ParseError::RelativeUrlWithoutBase) {
                        //get base url
                        let plink = Url::parse(&self.origin)
                            .expect("get css links, origin could not be parsed")
                            .join(link)
                            .expect("css links, could not join")
                            .to_string();
                        //push to return vector
                        link_hashset.insert((link.to_string(), plink.to_string()));
                        //check if base64 and continue if so
                    } else if RE3.is_match(link) {
                        continue;
                    //if fully formed link, push to return vector
                    } else if let Ok(parsed_link) = Url::parse(link) {
                        link_hashset.insert((link.to_string(), parsed_link.to_string()));
                    }
                }
                //No src, contine
                None => continue,
            };
        }
        //If hashset is empty return an Ok of None
        if link_hashset.is_empty() {
            Ok(None)
        //return some image links
        } else {
            Ok(Some(link_hashset))
        }
    }

    pub fn get_css_links(&self) -> Result<Option<HashSet<(String, String)>>, String> {
        let mut link_hashset: HashSet<(String, String)> = HashSet::new();

        //get links
        let selector = Selector::parse("link").unwrap();
        //loop through elements
        for element in self.html.select(&selector) {
            //check if stylesheets
            if element.value().attr("rel").unwrap() == "stylesheet" {
                //get the href
                match element.value().attr("href") {
                    Some(link) => {
                        //take care of relative links here
                        if Url::parse(link) == Err(ParseError::RelativeUrlWithoutBase) {
                            //create url
                            let plink = Url::parse(&self.origin)
                                .expect("get css links, origin could not be parsed")
                                .join(link)
                                .expect("css links, could not join")
                                .to_string();
                            //add to hashset
                            link_hashset.insert((link.to_string(), plink.to_string()));
                        } else if let Ok(parsed_link) = Url::parse(link) {
                            link_hashset.insert((link.to_string(), parsed_link.to_string()));
                        }
                    }
                    None => continue,
                };
            }
        }

        if link_hashset.is_empty() {
            Ok(None)
        } else {
            Ok(Some(link_hashset))
        }
    }

    //get js links
    pub fn get_js_links(&self) -> Result<Option<HashSet<(String, String)>>, String> {
        //create hashset
        let mut link_hashset: HashSet<(String, String)> = HashSet::new();
        //get the selector which is basically used for getting the script tags
        let selector = Selector::parse("script").unwrap();
        for element in self.html.select(&selector) {
            //get src attribute of the script tag
            match element.value().attr("src") {
                Some(link) => {
                    if Url::parse(link) == Err(ParseError::RelativeUrlWithoutBase) {
                        //parse relative url
                        let plink = Url::parse(&self.origin)
                            .expect("get js links, origin could not be parsed ")
                            .join(link)
                            .expect("js links, could not join")
                            .to_string();
                        link_hashset.insert((link.to_string(), plink.to_string()));
                    } else if let Ok(parsed_link) = Url::parse(link) {
                        //url doesnt need to be parsed, add it to the hashset
                        link_hashset.insert((link.to_string(), parsed_link.to_string()));
                    }
                }
                None => continue,
            };
        }

        //if hashset is empty return a result of None
        if link_hashset.is_empty() {
            Ok(None)
        } else {
            //return a result of some
            Ok(Some(link_hashset))
        }
    }
}
