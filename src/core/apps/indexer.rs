use std::{fs, path::{Path, PathBuf}};

use crate::core::apps::{model::{AppList, CacheFile}, parser::parse_data, utils::last_modified};

pub fn indexing() -> Option<Vec<AppList>> {
    let cache_dir = dirs::cache_dir()?.join("stryde");
    // Get path to stryde cache
    let cache_path = cache_dir.join("cache.bin");

    if fs::metadata(&cache_dir).is_err() {
        fs::create_dir_all(&cache_dir).ok()?;
    }
    // If dir not exist, I make it
    #[cfg(target_os = "linux")]
    if cfg!(target_os = "linux"){
        let app_dir = dirs::home_dir()?.join(".local/share/applications");
        let system_dir = PathBuf::from("/usr/share/applications");
        let flatpak_dir = Path::new("/var/lib/flatpak/app");
    
        let hash = {
            let m1 = last_modified(&app_dir).unwrap_or(0);
            let m2 = last_modified(&system_dir).unwrap_or(0);
            let m3 = last_modified(&flatpak_dir).unwrap_or(0);
            m1 + m2 + m3
        };
        // Get sum of dates when was modified the apps dir (system, flatpak, user)

        if let Ok(cache) = load_cache(&cache_path) {
            if cache.hash == hash {
                return Some(cache.apps);
            }
        }
        // If cache loaded without errors and the cache hash equals to new one return the cache

        let cache_file = CacheFile {
            hash,
            apps: parse_data() // Parse apps
        };
        save_cache(&cache_file, &cache_path).ok()?;
        return Some(cache_file.apps);
    }
    #[cfg(target_os = "windows")]
    if cfg!(target_os = "windows"){
        use crate::core::apps::platforms::windows::compute_windows_apps_hash;

        let hash = compute_windows_apps_hash();

        if let Ok(cache) = load_cache(&cache_path) {
            if cache.hash == hash {
                return Some(cache.apps);
            }
        }
        // If cache loaded without errors and the cache hash equals to new one return the cache

        let cache_file = CacheFile {
            hash,
            apps: parse_data() // Parse apps
        };
        save_cache(&cache_file, &cache_path).ok()?;
        return Some(cache_file.apps);
    }
    Some(Vec::new())
}

fn load_cache(path: &Path) -> Result<CacheFile, Box<dyn std::error::Error>> {
     // read cache file and return the apps and hash
     let data = fs::read(path)?;
     Ok(bincode::deserialize(&data)?)
}

fn save_cache(cache: &CacheFile, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
       let data = bincode::serialize(cache)?;
       fs::write(path, data)?;
       Ok(())
       // Save the cache
}