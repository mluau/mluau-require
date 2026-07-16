use std::{collections::HashMap, sync::Arc};

pub enum VfsEntry {
    Dir,
    File(String)
}

#[repr(transparent)]
/// A single block of a Vfs filesystem
pub struct VfsBlock(HashMap<String, VfsEntry>);

impl VfsBlock {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("".to_string(), VfsEntry::Dir); // ensure theres a root dir always
        Self(map)
    }
    pub fn push_dir(&mut self, path: String) {
        self.0.insert(path, VfsEntry::Dir);
    }

    pub fn push_file(&mut self, path: String, content: String) {
        self.0.insert(path, VfsEntry::File(content));
    }
}

#[derive(Clone)]
pub struct Vfs {
    blocks: Vec<Arc<VfsBlock>>
}

impl std::fmt::Debug for Vfs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Vfs")
    }
}

impl Vfs {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }

    pub fn add(mut self, next_vfs: VfsBlock) -> Self {
        self.blocks.push(next_vfs.into());
        self
    }

    pub fn add_arc(mut self, next_vfs: Arc<VfsBlock>) -> Self {
        self.blocks.push(next_vfs);
        self
    }

    pub fn extend(&mut self, other: Self) {
        self.blocks.extend(other.blocks);
    }

    pub fn extend_ref(&mut self, other: &Self) {
        self.blocks.extend(other.blocks.iter().cloned());
    }

    pub fn get(&self, path: &str) -> Option<&VfsEntry> {
        #[cfg(test)]
        println!("Get {path}");
        for block in self.blocks.iter() {
            if let Some(data) = block.0.get(path) {
                return Some(&data)
            }
        }

        None
    }
}
