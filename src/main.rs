
fn main() {
    // Getting full info of web page with reqwest
    let response = reqwest::blocking::get(
        "https://returnofthemounthuasectmanhwa.com",
    )
    .unwrap()
    .text()
    .unwrap();
    // Using scraper to parse the response
    let document = scraper::Html::parse_document(&response);
    // Defining the query to sort through document with
    let title_selector = scraper::Selector::parse("td.a").unwrap();
    // Finding what we want in document 
    let titles = document.select(&title_selector).map(|x| x.inner_html());
    titles
        .zip(1..20)
        .for_each(|(item, number)| println!("{}. {}", number, item));
}
