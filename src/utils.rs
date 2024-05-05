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

/// Handles opening link with web_sys::window
pub fn open_link(link: &str, new_tab: bool) {
    let url = if has_protocol(link) {
        link.to_string()
    } else {
        format!("https://{}", link)
    };

    let target = if new_tab { "_blank" } else { "" };

    let _window = web_sys::window()
        .unwrap()
        .window()
        .open_with_url_and_target(&url, target);
}

/// Checks if given string is a url, returning true if so
pub fn is_url(url_str: &str) -> bool {
    let reg =
        Regex::new(r"^((https?|ftp|smtp):\/\/)?(www.)?[a-z0-9]+\.[a-z]+(\/[a-zA-Z0-9#]+\/?)*$")
            .unwrap();

    reg.is_match(url_str)
}

// Checks if given string contains a url protocol, returning true if so
pub fn has_protocol(url_str: &str) -> bool {
    let reg = Regex::new(r"^((http|https|ftp|smtp)://)").unwrap();

    reg.is_match(url_str)
}
