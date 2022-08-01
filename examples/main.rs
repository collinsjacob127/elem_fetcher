/*
Jacob Collins
Rust Web Scraper
July 31, 2022
*/
use web_scraper_rs::scavenger::scavenger;
fn main() {
    scavenger("https://returnofthemounthuasectmanhwa.com", ["td>a", "td>i"].to_vec()).iter().for_each(|item| println!("{}", item));
}
