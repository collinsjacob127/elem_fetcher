/*
jacob collins
rust web scraper
July 31, 2022
*/
use scraper::{Html, Selector};

/// @param link: String - takes the URL of the webpage to be scraped
/// @param tags: Vec<String> - takes the tag code to filter the page by
pub fn scavenger(link: &str, tags: Vec<&str>) -> Vec<Vec<String>> {
    let mut out = Vec::new();
    let mut tag_items;
    // Getting full info of web page with reqwest
    let response = reqwest::blocking::get(link,)
    .unwrap()
    .text()
    .unwrap();
    // Using scraper to parse the response
    let document = Html::parse_document(&response);
    // Loop through each tag and collect text inside all elements in instances of that tag
    for tag in tags {
        tag_items = Vec::new();
        // Defining the query to sort through document with
        let title_selector = Selector::parse(tag).unwrap();
        // Finding what we want in document 
        let titles = document.select(&title_selector).map(|x| x.inner_html());
        // Running through each found element and printing them
        titles
            .for_each(|item| tag_items.push(item));
        out.push(tag_items);
    }
    return out;
}
