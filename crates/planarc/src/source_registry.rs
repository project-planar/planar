use crate::{module_loader::Source, spanned::FileId};
use std::collections::BTreeMap;

#[derive(Default, Debug, Clone)]
pub struct SourceRegistry {
    files: BTreeMap<FileId, Source>,
    next_id: u32,
}

impl SourceRegistry {
    
    pub fn add_with_id(&mut self, source: Source, id: FileId) {
        self.files.insert(id, source);
    }

    pub fn add(&mut self, source: Source) -> (FileId, &Source) {
        let id = FileId(self.next_id);
        self.next_id += 1;
        self.files.insert(id, source);
        (id, &self.files[&id])
    }

    pub fn get(&self, id: FileId) -> Option<&Source> {
        self.files.get(&id)
    }
}
