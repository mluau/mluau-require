use std::collections::HashMap;

pub enum VfsEntry {
    Dir,
    File(String)
}

pub type VfsBlock = HashMap<String, VfsEntry>;

pub struct Vfs {
    blocks: Vec<VfsBlock>
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

    pub fn add(mut self, mut next_vfs: VfsBlock) -> Self {
        next_vfs.insert("".to_string(), VfsEntry::Dir); // ensure theres a root dir always
        self.blocks.push(next_vfs);
        self
    }

    pub fn extend(&mut self, other: Self) {
        self.blocks.extend(other.blocks);
    }

    pub fn push_dir(&mut self, b: usize, path: &str) {
        self.blocks[b].insert(path.to_string(), VfsEntry::Dir);
    }

    pub fn push_file(&mut self, b: usize, path: &str, content: String) {
        self.blocks[b].insert(path.to_string(), VfsEntry::File(content));
    }

    pub fn get(&self, path: &str) -> Option<&VfsEntry> {
        #[cfg(test)]
        println!("Get {path}");
        for block in self.blocks.iter() {
            if let Some(data) = block.get(path) {
                return Some(&data)
            }
        }

        None
    }
}
