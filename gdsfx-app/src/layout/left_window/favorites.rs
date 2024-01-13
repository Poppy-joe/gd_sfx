use eframe::egui::Ui;
use gdsfx_library::{LibraryEntry, EntryKind};

use crate::{GdSfx, settings};

pub fn render(ui: &mut Ui, gdsfx: &mut GdSfx, entry: &LibraryEntry) {

    match &entry.kind {
        EntryKind::Category { children } => {
            for child in children {
                render(ui, gdsfx, child);
            }
        },
        EntryKind::Sound { bytes, duration } => {
            if settings::has_favourite(id) {
                gui::add_sfx_button(ui, gdsfx, entry);
            }
        },
    }
}