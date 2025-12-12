use std::path::Path;
use std::path::PathBuf;

use regex::Regex;
use windows_icons::get_icon_by_path;
use winreg::RegKey;
use winreg::enums::*;

use crate::core::apps::model::AppList;

#[derive(Debug, PartialEq, Eq)]
enum SoftwareType {
    Regular,
    SystemComponent,
    WindowsInstaller,
}

#[derive(Debug)]
struct Software {
    name: String,
    version: String,
    install_location: String,
    icon: String,
    software_type: SoftwareType,
}

pub fn load_ico_image(path: &str) -> iced::widget::image::Handle {
    let img = get_icon_by_path(path).unwrap_or_default();

    let (width, height) = img.dimensions();
    iced::widget::image::Handle::from_rgba(width, height, img.into_raw())
}

fn get_registry_key_hash(key: &RegKey) -> Option<u64> {
    let now = key.query_info().map(|info| info.get_last_write_time_system());
    
    Some(now.ok()?.wMinute.into())
}

pub fn compute_windows_apps_hash() -> u64 {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let hku = RegKey::predef(HKEY_USERS);

    let uninstall_paths = [
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
        r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
    ];

    let mut hash: u64 = 0;

    for path in &uninstall_paths {
        if let Ok(key) = hklm.open_subkey(path) {
            hash += get_registry_key_hash(&key).unwrap_or_default();
        }
        if let Ok(key) = hkcu.open_subkey(path) {
            hash += get_registry_key_hash(&key).unwrap_or_default();
        }
    }

    for user_sid in hku.enum_keys().filter_map(Result::ok) {
        let user_uninstall_path = format!(
            r"{}\Software\Microsoft\Windows\CurrentVersion\Uninstall",
            user_sid
        );
        if let Ok(key) = hku.open_subkey(&user_uninstall_path) {
            hash += get_registry_key_hash(&key).unwrap_or_default();
        }
    }

    hash
}

fn get_uninstall_key_programs(
    uninstall_key: &RegKey,
    classes_key: &RegKey,
    include_updates: bool,
) -> Vec<Software> {
    let mut installed_programs = Vec::new();
    let windows_update_regex = Regex::new(r"KB[0-9]{6}$").unwrap();

    for subkey_name in uninstall_key.enum_keys().filter_map(Result::ok) {
        if let Ok(subkey) = uninstall_key.open_subkey(&subkey_name) {
            let is_system_component: u32 = subkey.get_value("SystemComponent").unwrap_or(0);
            let is_windows_installer: u32 = subkey.get_value("WindowsInstaller").unwrap_or(0);

            let software_type = if is_system_component == 1 {
                SoftwareType::SystemComponent
            } else if is_windows_installer == 1 {
                SoftwareType::WindowsInstaller
            } else {
                SoftwareType::Regular
            };
            
            if software_type == SoftwareType::SystemComponent || software_type == SoftwareType::WindowsInstaller {
                continue;
            }

            let release_type: String = subkey.get_value("ReleaseType").unwrap_or_default();
            let prog_version: String = subkey.get_value("DisplayVersion").unwrap_or_default();
            let name: String = subkey.get_value("DisplayName").unwrap_or_default();
            let install_location: String = subkey.get_value("InstallLocation").unwrap_or_default();
            let icon: String = subkey.get_value("DisplayIcon").unwrap_or_default();
            let parent_key_name: String = subkey.get_value("ParentKeyName").unwrap_or_default();
            let uninstall_string: String = subkey.get_value("UninstallString").unwrap_or_default();

            let is_update = windows_update_regex.is_match(&subkey_name)
                || !parent_key_name.is_empty()
                || release_type == "Security Update"
                || release_type == "Update Rollup"
                || release_type == "Hotfix";

            if is_update && !include_updates {
                continue;
            }

            if software_type == SoftwareType::Regular
                && !uninstall_string.is_empty()
                && !name.is_empty()
            {
                installed_programs.push(Software {
                    name,
                    version: prog_version,
                    install_location,
                    icon,
                    software_type,
                });
            } else if software_type == SoftwareType::WindowsInstaller {
                let msi_key_name = format!("Installer\\Products\\{}", subkey_name);
                let mut name = String::new();
                let mut icon = String::new();

                if let Ok(cr_guid_key) = classes_key.open_subkey(&msi_key_name) {
                    name = cr_guid_key.get_value("ProductName").unwrap_or_default();
                    icon = cr_guid_key.get_value("ProductIcon").unwrap_or_default();
                }

                if name.is_empty() {
                    name = subkey.get_value("DisplayName").unwrap_or_default();
                }

                if icon.is_empty() {
                    icon = subkey.get_value("DisplayIcon").unwrap_or_default();
                }

                if !name.is_empty() {
                    installed_programs.push(Software {
                        name,
                        version: prog_version,
                        install_location,
                        icon,
                        software_type,
                    });
                }
            }
        }
    }

    installed_programs
}

pub fn get_windows_apps() -> Option<Vec<AppList>>{
    let mut installed_programs = Vec::new();
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let hku = RegKey::predef(HKEY_USERS);

    let classes_key_path = r"SOFTWARE\Classes\Installer\Products";

    let classes_key = match hklm.open_subkey(classes_key_path) { Ok(key) => key, Err(_) => { eprintln!("Failed to open classes key."); return None; } };

    if let Ok(uninstall_key) =
        hkcu.open_subkey(r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall")
    {
        installed_programs.extend(get_uninstall_key_programs(
            &uninstall_key,
            &classes_key,
            true,
        ));
    }

    for user_sid in hku.enum_keys().filter_map(Result::ok) {
        let user_uninstall_path = format!(
            r"{}\Software\Microsoft\Windows\CurrentVersion\Uninstall",
            user_sid
        );
        if let Ok(uninstall_key) = hku.open_subkey(&user_uninstall_path) {
            installed_programs.extend(get_uninstall_key_programs(
                &uninstall_key,
                &classes_key,
                false,
            ));
        }
    }
    let mut apps: Vec<AppList> = Vec::new();

    for program in installed_programs {
        let mut icon_path: &str = &program.icon;
        if let Some(pos) = program.icon.find(",") {
            icon_path = &icon_path[..pos];
        };
        println!("Icon path {:?}, exec path {:?}", icon_path, program.install_location);
        if !program.install_location.is_empty() {
        apps.push(AppList {
            name: program.name,
            description: None,
            exec: Some(icon_path.into()),
            icon_path: Some(PathBuf::from(icon_path)),
            type_file: Some("Application".into()),
            terminal: Some(false),
        });
    }
    }
    return Some(apps);
}