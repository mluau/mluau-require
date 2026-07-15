use std::collections::HashMap;
use std::rc::Rc;

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

#[derive(Debug, Clone)]
/// A wrapper around a VFS file system
pub struct FilesystemWrapper(Rc<Vfs>);

impl FilesystemWrapper {
    pub fn new(fs: Vfs) -> Self {
        Self(Rc::new(fs))
    }

    pub fn read_file(&self, path: &str) -> Result<&String, &'static str> {
        match self.0.get(path).ok_or("file not found")? {
            VfsEntry::Dir => Err("file is a directory"),
            VfsEntry::File(p) => Ok(p)
        }
    }

    /// Fixes the path to conform to the VFS specific quirks/format
    pub fn path_fix<'a>(path: &'a str) -> &'a str {
        if path.starts_with("./") {
            return path.trim_start_matches("./");
        } else if path.starts_with('/') {
            return path.trim_start_matches("/");
        }

        path
    }

    pub fn is_file(&self, path: &str) -> bool {
        let path = Self::path_fix(path);
        match self.0.get(&path) {
            Some(VfsEntry::File(_)) => true,
            _ => false
        }
    }

    pub fn get_file(&self, path: &str) -> Result<&String, &'static str> {
        let path = Self::path_fix(path);
        let contents = self.read_file(&path)?;
        Ok(contents)
    }

    pub fn is_dir(&self, path: &str) -> bool {
        let path = Self::path_fix(&path);
        match self.0.get(&path) {
            Some(VfsEntry::Dir) => true,
            _ => false
        }
    }
}
