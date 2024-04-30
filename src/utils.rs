use crate::prelude::*;

pub fn save(profile: &Profile) {
    let window = web_sys::window().unwrap().window();
    let storage = window.local_storage().unwrap().unwrap();

    let profile_str = serde_json::to_string(&profile).unwrap();
    let _ = storage.set("profile", &profile_str);
}

pub fn load() -> Profile {
    let window = web_sys::window().unwrap().window();
    let storage = window.local_storage().unwrap().unwrap();
    if let Ok(profile_res) = storage.get("profile") {
        if let Some(profile_str) = profile_res {
            let profile = serde_json::from_str(&profile_str).unwrap();
            return profile;
        }
    }

    let profile = Profile::new();
    save(&profile);

    profile
}

/// Handles opening link
pub fn open_link(link: &str, new_tab: bool) {
    let target = if new_tab { "_blank" } else { "" };

    let _window = web_sys::window()
        .unwrap()
        .window()
        .open_with_url_and_target(link, target);
}
