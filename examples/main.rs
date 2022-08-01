/*
Jacob Collins
Rust Web Scraper
July 31, 2022
*/
use web_scraper_rs::scavenger::scavenger;
fn main() {
    let test_output = 
        // scavenger("https://returnofthemounthuasectmanhwa.com",
        // [".intro_content>h2", "td>a", "td>i"].to_vec());
        scavenger("https://flamescans.org/series/heavenly-demon-cultivation-simulation/",
        ["h1", ".wp-manga-chapter", ".chapter-release-date"].to_vec());
    match test_output {
        Ok(test_output) => {
            let output_str = 
                ["Title:", &test_output[0][0], 
                // "Latest Chapter:", &test_output[1][0],
                // "Last Updated", &test_output[2][0]
                ].join(" ");
            println!("{}", output_str);
        }
        Err(test_output) => {
            panic!("{}", test_output);
        }
    }
}
