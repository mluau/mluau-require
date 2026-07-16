use crate::Vfs;

use std::collections::HashMap;

/// Creates a Vfs from a map of paths to content.
pub fn create_memory_vfs_from_map(
    tree: HashMap<String, String>,
) -> crate::Vfs {
    let mut fs = crate::VfsBlock::new();
    
    for (path_s, content) in tree {
        // Find every '/' and insert the slice up to that point as a directory
        for (i, _) in path_s.match_indices('/') {
            let dir_path = path_s[..i].to_string();
            fs.insert(dir_path, crate::VfsEntry::Dir);
        }
        
        // Insert the actual file
        fs.insert(path_s, crate::VfsEntry::File(content));
    }

    Vfs::new().add(fs)
}

/// Creates a Vfs from a map of paths to content.
pub fn create_memory_vfs_from_map_ref(
    tree: &HashMap<String, String>,
) -> crate::Vfs {
    let mut fs = crate::VfsBlock::new();
    
    for (path_s, content) in tree {
        // Find every '/' and insert the slice up to that point as a directory
        for (i, _) in path_s.match_indices('/') {
            let dir_path = path_s[..i].to_string();
            fs.insert(dir_path, crate::VfsEntry::Dir);
        }
        
        // Insert the actual file
        fs.insert(path_s.to_string(), crate::VfsEntry::File(content.clone()));
    }

    Vfs::new().add(fs)
}

/// Creates a Vfs from a map of paths to content.
#[cfg(feature = "rust-embed")]
pub fn create_memory_vfs_from_embedded<T: rust_embed::RustEmbed>() -> crate::Vfs {
    let mut fs = crate::VfsBlock::new();
    for path_s in T::iter() {
        // Find every '/' and insert the slice up to that point as a directory
        for (i, _) in path_s.match_indices('/') {
            let dir_path = path_s[..i].to_string();
            fs.insert(dir_path, crate::VfsEntry::Dir);
        }
        
        // Insert the actual file
        let content = String::from_utf8_lossy_owned(T::get(&path_s).expect("internal error reading file").data.into_owned());
        fs.insert(path_s.to_string(), crate::VfsEntry::File(content));
    }

    Vfs::new().add(fs)
}
