/*
Jacob Collins
Rust Web Scraper
July 31, 2022
*/

use std::ops::Index;

pub struct Collection {
    manga_lst: MangaList,
}
    
type MangaList = Option<Vec<Manga>>;

struct Manga {
    link: String,
    title: String,
    last_updated: String,
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
    fn get_manga(&self, index: i32) -> Option<&Manga> {
        unimplemented!("get_manga is unimplemented");
    }
}

impl Manga {
    pub fn new(link: &str) -> Self {
        Manga {
            link: (link.to_string()), 
            title: ("unavailable".to_string()), 
            last_updated: ("unavailable".to_string()) 
        }
    }
    pub fn update() {

    }
}

#[cfg(test)]
mod test {
    #[test]
    fn basics() {

    }
}