pub mod file_store;

use std::error::Error;

pub trait NoteKey {}

pub trait Note<K: NoteKey> {
    fn get_key(&self) -> &K;
    fn get_workspace(&self) -> &str;
    fn get_title(&self) -> &str;
    fn get_body(&self) -> &str;

    fn update_title(&mut self, title: &str);
    fn update_body(&mut self, body: &str);
}

pub trait Save<K: NoteKey, T: Note<K>> {
    fn save(&mut self, t: &T) -> Result<K, Box<dyn Error>>;
}

pub trait Delete<K: NoteKey> {
    fn delete(&mut self, t: &K) -> Result<(), Box<dyn Error>>;
}

pub trait Get<K: NoteKey, T: Note<K>> {
    fn get(&self, t: &K) -> Option<T>;
}

pub trait List<K: NoteKey, T: Note<K>> {
    fn list_keys(&self) -> Vec<K>;
    fn list_values(&self) -> Vec<T>;
}

pub trait Store<K: NoteKey, T: Note<K>>: Save<K, T> + Delete<K> + Get<K, T> + List<K, T> {}
