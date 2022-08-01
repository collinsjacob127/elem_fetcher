/*
jacob collins
rust web scraper
July 31, 2022
*/
use scraper::{Html, Selector};

/// @param link: String - takes the URL of the webpage to be scraped
/// @param tags: Vec<String> - takes the tag code to filter the page by
pub fn scavenger(link: &str, tags: Vec<&str>) -> Vec<String> {
    let mut out = Vec::new();
    // Getting full info of web page with reqwest
    let response = reqwest::blocking::get(
link,
    )
    .unwrap()
    .text()
    .unwrap();
    // Using scraper to parse the response
    let document = Html::parse_document(&response);
    for tag in tags {
        // Defining the query to sort through document with
        let title_selector = Selector::parse(tag).unwrap();
        // Finding what we want in document 
        let titles = document.select(&title_selector).map(|x| x.inner_html());
        // Running through each found element and printing them
        titles
            .zip(1..2)
            .for_each(|item| out.push(item.0));
    }
    return out;
}