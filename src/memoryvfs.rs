use crate::Vfs;

use std::collections::HashMap;

/// Creates a Vfs from a map of paths to content.
pub fn create_memory_vfs_from_map(
    tree: HashMap<String, String>,
) -> crate::Vfs {
    let mut fs = crate::VfsBlock::new();
    for (path_s, content) in tree {
        let path = path_s.split('/').collect::<Vec<_>>();
        if path.len() >= 2 {
            // Folder part is everything except the last part
            let mut folder_part = Vec::with_capacity(path.len() - 1);
            let plen = path.len();
            for (i, part) in path.into_iter().enumerate() {
                if i == plen - 1 {
                    break;
                }
                folder_part.push(part);
            }

            let mut current_path = String::new();
            for (i, folder) in folder_part.iter().enumerate() {
                if i != 0 {
                    current_path.push('/');
                }
                current_path.push_str(folder);
                fs.insert(current_path.clone(), crate::VfsEntry::Dir);
            }
        }
        fs.insert(path_s, crate::VfsEntry::File(content));
    }

    Vfs::new().add(fs)
}

/// Creates a Vfs from a map of paths to content.
#[cfg(feature = "rust-embed")]
pub fn create_memory_vfs_from_embedded<T: rust_embed::RustEmbed>() -> crate::Vfs {
    let mut fs = crate::VfsBlock::new();
    for path_s in T::iter() {
        let path = path_s.split('/').collect::<Vec<_>>();
        if path.len() >= 2 {
            // Folder part is everything except the last part
            let mut folder_part = Vec::with_capacity(path.len() - 1);
            let plen = path.len();
            for (i, part) in path.into_iter().enumerate() {
                if i == plen - 1 {
                    break;
                }
                folder_part.push(part);
            }

            let mut current_path = String::new();
            for (i, folder) in folder_part.iter().enumerate() {
                if i != 0 {
                    current_path.push('/');
                }
                current_path.push_str(folder);
                fs.insert(current_path.clone(), crate::VfsEntry::Dir);
            }
        }
        let content = T::get(&path_s).expect("internal error reading file").data.into_owned();
        fs.insert(path_s.to_string(), crate::VfsEntry::File(String::from_utf8_lossy_owned(content)));
    }

    Vfs::new().add(fs)
}
