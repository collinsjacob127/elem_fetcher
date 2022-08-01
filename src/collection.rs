/*
Jacob Collins
Rust Web Scraper
July 31, 2022
*/
use crate::scavenger::scavenger;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
pub struct Collection {
    manga_lst: MangaList,
}
    
type MangaList = Option<Vec<Manga>>;

struct Manga {
    link: String,
    title: Option<String>,
    last_updated: Option<String>,
}


#[derive(Hash)]
struct Domain {
    domain: String,
    chap_tag: String,
    lst_update_tag: String,
    title_tag: String,
}

impl Collection {
    pub fn new() -> Self {
        Collection { manga_lst: (None) }
    }
    pub fn add_manga(&mut self, link: &str) {
        let new_manga = Manga::new(link);
        match self.manga_lst.take() {
            Some(mut cur_manga_lst) => {
                cur_manga_lst.push(new_manga);
            }
            None => {
                let mut new_manga_vec = Vec::<Manga>::new();
                new_manga_vec.push(new_manga);
                self.manga_lst = Some(new_manga_vec);
            }
        }
    }
    fn get_manga(&self) -> &MangaList {
        &self.manga_lst
    }
}

impl Manga {
    pub fn new(link: &str) -> Self {
        Manga {
            link: (link.to_string()), 
            last_updated: None,
            title: None,
        }
    }
    fn generate_elements(&mut self, link: &str) {

    // Library of manga sites
    let asura_scans = Domain {
        domain: "https://www.asurascans.com/".to_owned(),
        chap_tag: "span.chapternum".to_owned(),
        lst_update_tag: "span.chapterdate" .to_owned(),
        title_tag: "h1.entry-title".to_owned(),
    };
        match scavenger(link,
        ["td>a", "td>i"].to_vec()) {
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
    fn update() {

    }
}

#[cfg(test)]
mod test {
    #[test]
    fn basics() {
        unimplemented!("oopsie");
    }
}