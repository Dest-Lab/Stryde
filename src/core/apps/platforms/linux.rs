use std::{collections::HashSet, fs, path::PathBuf};


pub fn get_default_search_paths() -> Vec<PathBuf> {
    let mut search_paths = vec![];
    // read XDG_DATA_DIRS env var
    let xdg_data_dirs = std::env::var("XDG_DATA_DIRS").unwrap_or("/usr/share".to_string());
    let xdg_data_dirs: Vec<&str> = xdg_data_dirs.split(':').collect();
    // make a string sett from xdg_data_dirs
    let home_dir = dirs::home_dir().unwrap_or_default();
    let home_path = PathBuf::from(home_dir);
    let local_share_apps = home_path.join(".local/share/applications");
    let mut default_search_paths = vec![
        "/usr/share/applications",
        "/usr/share/xsessions",
        "/etc/xdg/autostart",
        "/var/lib/snapd/desktop/applications",
        local_share_apps.to_str().unwrap_or_default(),
    ];
    for path in xdg_data_dirs {
        default_search_paths.push(path);
    }

    for path in default_search_paths {
        search_paths.push(PathBuf::from(path));
    }
    search_paths
}

pub fn get_all_apps() -> Option<HashSet<PathBuf>> {
    let default_search_paths = get_default_search_paths();
    let search_dirs: HashSet<PathBuf> = default_search_paths
        .into_iter()
        .filter(|dir| dir.exists())
        .collect();
    // for each dir, search for .desktop files
    let mut apps: HashSet<PathBuf> = HashSet::new();
    for dir in search_dirs {
        for entry in fs::read_dir(&dir).ok()? {
            if entry.is_err() {
                continue;
            }
            let path = entry.ok()?.path();
            if path.extension().is_none() {
                continue;
            }

            if path.extension().unwrap_or_default() == "desktop" && path.is_file() {
                apps.insert(path);
            }
        }
    }
    Some(apps)
}