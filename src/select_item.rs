use skim::{Skim, SkimOptions};
use std::io::Cursor;
extern crate skim;
use skim::prelude::*;

pub fn select(found_dirs: String, options: &SkimOptions) -> String {
    let item_reader = SkimItemReader::default();
    let dirs = item_reader.of_bufread(Cursor::new(found_dirs));
    if let Some(items) = Skim::run_with(&options, Some(dirs))
        .iter()
        .find(|out| out.final_event == Event::EvActAccept(None))
    {
        items.selected_items[0].text().to_string()
    } else {
        "".to_string()
    }
}
