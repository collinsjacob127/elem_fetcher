/*
Jacob Collins
Rust Web Scraper
July 31, 2022
*/
use scraper::{Html, Selector};
fn main() {
    // Getting full info of web page with reqwest
    let response = reqwest::blocking::get(
        "https://returnofthemounthuasectmanhwa.com",
    )
    .unwrap()
    .text()
    .unwrap();
    // Using scraper to parse the response
    let document = Html::parse_document(&response);
    // Defining the query to sort through document with
    let title_selector = Selector::parse("td>a").unwrap();
    // Finding what we want in document 
    let titles = document.select(&title_selector).map(|x| x.inner_html());
    // Running through each found element and printing them
    titles
        .zip(1..20)
        .for_each(|item| println!("{}", item.0));
}
