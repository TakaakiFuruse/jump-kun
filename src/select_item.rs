use skim::{Skim, SkimOptions};
use std::io::Cursor;
extern crate skim;
use skim::prelude::*;

pub fn select(found_dirs: String, options: &SkimOptions) -> String {
    let item_reader = SkimItemReader::default();
    let dirs = item_reader.of_bufread(Cursor::new(found_dirs));
    let items = Skim::run_with(&options, Some(dirs))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());
    if !items.is_empty() {
        items[0].text().to_string()
    } else {
        "".to_string()
    }
}
