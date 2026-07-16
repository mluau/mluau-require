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
            fs.push_dir(dir_path);
        }
        
        // Insert the actual file
        fs.push_file(path_s, content);
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
            fs.push_dir(dir_path);
        }
        
        // Insert the actual file
        fs.push_file(path_s.to_string(), content.clone());
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
            fs.push_dir(dir_path);
        }
        
        // Insert the actual file
        let content = String::from_utf8_lossy_owned(T::get(&path_s).expect("internal error reading file").data.into_owned());
        fs.push_file(path_s.to_string(), content);
    }

    Vfs::new().add(fs)
}
