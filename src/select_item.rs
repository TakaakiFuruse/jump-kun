use skim::{Skim, SkimOptions};
use std::io::Cursor;

pub fn select(found_dirs: String, options: &SkimOptions) -> String {
    let items = Skim::run_with(&options, Some(Box::new(Cursor::new(found_dirs))))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());
    if items.len() > 0 {
        items[0].get_output_text().to_string()
    } else {
        "".to_string()
    }
}
