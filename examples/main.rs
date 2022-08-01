/*
Jacob Collins
Rust Web Scraper
July 31, 2022
*/
use web_scraper_rs::scavenger::scavenger;
fn main() {
    let test_output = 
        scavenger("https://returnofthemounthuasectmanhwa.com",
        ["td>a", "td>i"].to_vec());
    match test_output {
        Ok(test_output) => {
            let output_str = [test_output[0][0].to_string(), 
                test_output[1][0].to_string()].join(" - ");
            println!("{}", output_str);
        }
        Err(test_output) => {
            panic!("{}", test_output);
        }
    }
}
/*
Asura
Luminous
Nocutrnal
Flamescans
*/