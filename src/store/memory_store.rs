use std::collections::HashMap;
use std::error::Error;
use uuid::Uuid;
use crate::{Delete, List, Note, NoteKey, Save};
use crate::store::{Get, Store};

#[derive(Clone, Hash, Eq, PartialEq, Default, Debug)]
pub struct MemoryBasedNoteKey(Uuid);

impl NoteKey for MemoryBasedNoteKey {}

impl MemoryBasedNoteKey {
    fn new() -> MemoryBasedNoteKey {
        MemoryBasedNoteKey(Uuid::new_v4())
    }
}

#[derive(Clone, Default, Debug)]
pub struct MemoryBasedNote {
    title: String,
    body: String,
}

impl Note<MemoryBasedNoteKey> for MemoryBasedNote {
    fn get_key(&self) -> &MemoryBasedNoteKey {
        todo!()
    }

    fn get_workspace(&self) -> &str {
        todo!()
    }

    fn get_title(&self) -> &str {
        &self.title
    }

    fn get_body(&self) -> &str {
        &self.body
    }

    fn update_title(&mut self, title: &str) {
        todo!()
    }

    fn update_body(&mut self, body: &str) {
        todo!()
    }
}

impl MemoryBasedNote {
    pub fn new(title: String, body: String) -> MemoryBasedNote {
        MemoryBasedNote {
            title,
            body,
        }
    }
}

pub struct MemoryStore {
    notes: HashMap<MemoryBasedNoteKey, MemoryBasedNote>,
}

impl MemoryStore {
    pub fn new() -> Result<MemoryStore, Box<dyn Error>> {
        Ok(MemoryStore {
            notes: HashMap::new(),
        })
    }
}

impl Save<MemoryBasedNoteKey, MemoryBasedNote> for MemoryStore {
    fn save(&mut self, note: MemoryBasedNote) -> Result<MemoryBasedNoteKey, Box<dyn Error>> {
        let key = MemoryBasedNoteKey::new();
        self.notes.insert(key.to_owned(), note);
        Ok(key)
    }
}

impl Delete<MemoryBasedNoteKey> for MemoryStore {
    fn delete(&mut self, key: MemoryBasedNoteKey) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

impl Get<MemoryBasedNoteKey, MemoryBasedNote> for MemoryStore {
    fn get(&self, key: &MemoryBasedNoteKey) -> Option<MemoryBasedNote> {
        Some(self.notes.get(key)?.to_owned())
    }
}

impl List<MemoryBasedNoteKey, MemoryBasedNote> for MemoryStore {
    fn list_keys(&self) -> Vec<MemoryBasedNoteKey> {
        self.notes.keys().into_iter().map(|key| key.to_owned()).collect()
    }

    fn list_values(&self) -> Vec<MemoryBasedNote> {
        self.notes.values().into_iter().map(|value| value.to_owned()).collect()
    }
}

impl Store<MemoryBasedNoteKey, MemoryBasedNote> for MemoryStore {

}
