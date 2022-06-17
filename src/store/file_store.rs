use std::error::Error;
use std::fmt::{Debug, format};
use std::fs;
use std::fs::{create_dir, File};
use std::io::ErrorKind::AlreadyExists;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use uuid::{Uuid};
use serde::{Serialize, Deserialize};
use serde_json::from_str;
use crate::{Delete, Get, List, Note, NoteKey, Save};
use crate::store::Store;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FileBasedNoteKey(Uuid);

impl NoteKey for FileBasedNoteKey {}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileBasedNote {
    key: FileBasedNoteKey,
    workspace: String,
    title: String,
    body: String,
}

impl FileBasedNote {
    pub fn new(workspace: &str, title: &str, body: &str) -> FileBasedNote {
        FileBasedNote {
            key: FileBasedNoteKey(Uuid::new_v4()),
            workspace: workspace.to_owned(),
            title: title.to_owned(),
            body: body.to_owned()
        }
    }
}

impl Note<FileBasedNoteKey> for FileBasedNote {
    fn get_key(&self) -> &FileBasedNoteKey {
        &self.key
    }

    fn get_workspace(&self) -> &str {
        &self.workspace
    }

    fn get_title(&self) -> &str {
        &self.title
    }

    fn get_body(&self) -> &str {
        &self.body
    }

    fn update_title(&mut self, title: &str) {
        self.title = title.to_owned();
    }

    fn update_body(&mut self, body: &str) {
        self.body = body.to_owned();
    }
}

pub struct FileStore {
    store_directory: String,
}

impl FileStore {
    pub fn new(store_directory: &str) -> Result<FileStore, Box<dyn Error>> {
        if let Err(e) = create_dir(store_directory) {
            if e.kind() != AlreadyExists {
                return Err(Box::new(e));
            }
        }

        Ok(FileStore {
            store_directory: store_directory.to_owned()
        })
    }

    fn get_note_location(&self, key: &FileBasedNoteKey) -> PathBuf {
        Path::new(self.store_directory.as_str())
            .join(format!("{}", key.0))
            .to_path_buf()
    }
}

impl Save<FileBasedNoteKey, FileBasedNote> for FileStore {
    fn save(&mut self, note: &FileBasedNote) -> Result<FileBasedNoteKey, Box<dyn Error>> {
        let path = self.get_note_location(note.get_key());
        let serialized = serde_json::to_string(&note)?;

        let mut file = File::create(path)?;
        file.write_all(serialized.as_ref())?;

        Ok(note.get_key().to_owned())
    }
}

impl Delete<FileBasedNoteKey> for FileStore {
    fn delete(&mut self, note: &FileBasedNoteKey) -> Result<(), Box<dyn Error>> {
        fs::remove_file(self.get_note_location(&note))?;
        Ok(())
    }
}

impl Get<FileBasedNoteKey, FileBasedNote> for FileStore {
    fn get(&self, key: &FileBasedNoteKey) -> Option<FileBasedNote> {
        let mut file = match File::open(self.get_note_location(key)) {
            Ok(file) => Some(file),
            Err(err) => {
                println!("Couldn't open file for note {}", key.0);
                println!("{}", err);
                None
            }
        }?;

        let mut str= String::new();
        if let Err(_) = file.read_to_string(&mut str) {
            println!("Couldn't read contents for note {}", key.0);
            return None;
        }

        match from_str(str.as_ref()) {
            Ok(note) => Some(note),
            Err(err) => {
                println!("Couldn't parse note {}", key.0);
                println!("{}", err);
                None
            }
        }
    }
}

impl List<FileBasedNoteKey, FileBasedNote> for FileStore {
    fn list_keys(&self) -> Vec<FileBasedNoteKey> {
        fs::read_dir(&self.store_directory)
            .expect("Store folder couldn't be read")
            .into_iter()
            .map(|path| path.expect("Couldn't read file").file_name())
            .filter_map(|file_name| Uuid::from_str(file_name.to_str().expect("Corrupt file").as_ref()).ok())
            .map(|uuid| FileBasedNoteKey(uuid))
            .collect()
    }

    fn list_values(&self) -> Vec<FileBasedNote> {
        self.list_keys()
            .iter()
            .filter_map(|key| self.get(key))
            .collect()
    }
}

impl Store<FileBasedNoteKey, FileBasedNote> for FileStore {

}
