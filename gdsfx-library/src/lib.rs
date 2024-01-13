use std::{sync::OnceLock, path::PathBuf, fs, collections::HashMap};

use gdsfx_data::paths;
use once_cell::sync::Lazy;
use stats::Centiseconds;

use crate::credits::Credit;

pub mod favorites;
pub mod sorting;
pub mod stats;
pub mod tools;

mod credits;
mod requests;
mod parse;

pub type EntryId = u32;

type Bytes = Vec<u8>;

#[derive(Debug)]
pub struct Library {
    root: LibraryEntry,
    entries: HashMap<EntryId, LibraryEntry>,

    credits: Vec<Credit>,
}

impl Library {
    pub fn get_root(&self) -> &LibraryEntry {
        &self.root
    }

    pub fn get_entry(&self, id: EntryId) -> Option<&LibraryEntry> {
        self.entries.get(&id)
    }

    pub fn get_credits(&self) -> &Vec<Credit> {
        &self.credits
    }
}

#[derive(Debug, Clone)]
pub struct LibraryEntry {
    pub id: EntryId,
    pub name: String,
    pub parent_id: EntryId,
    pub kind: EntryKind,
}

#[derive(Debug, Clone)]
pub enum EntryKind {
    Category { children: Vec<EntryId> },
    Sound { bytes: i64, duration: Centiseconds },
}

fn try_read_file() -> Option<Bytes> {
    const SFX_LIBRARY_FILE: &str = "sfxlibrary.dat";
    
    static SFX_LIBRARY_PATH: Lazy<Option<PathBuf>> = Lazy::new(|| {
        paths::runtime::GD_FOLDER.as_ref()
            .map(|path| path.join(SFX_LIBRARY_FILE))
    });

    SFX_LIBRARY_PATH.as_ref()
        .and_then(|path| fs::read(path).ok())
}

pub fn get_library() -> &'static Library {
    static LIBRARY_DATA: OnceLock<Library> = OnceLock::new();

    LIBRARY_DATA.get_or_init(|| {
        let bytes = try_read_file()
        // TODO check sfx version
            .or_else(requests::fetch_library_data)
            .unwrap();

        let bytes = gdsfx_data::encoding::decode(&bytes);
        let string = std::str::from_utf8(&bytes).unwrap();
    
        parse::parse_library_string(string)
    })
}