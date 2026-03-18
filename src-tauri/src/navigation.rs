pub fn clean_google_redirect_url(url: &str) -> Option<String> {
    if !url.contains("google.com/url?")
        && !url.contains("google.com/imgres?")
        && !url.contains("google.com/redirect?")
    {
        return None;
    }

    if let Ok(parsed) = url::Url::parse(url) {
        for (key, value) in parsed.query_pairs() {
            if key == "url" || key == "q" || key == "dest_url" || key == "target" {
                let decoded = urlencoding::decode(&value).ok()?;
                if decoded.starts_with("http://") || decoded.starts_with("https://") {
                    return Some(decoded.to_string());
                }
            }
        }
    }

    None
}

pub fn is_google_meet_link(url: &str) -> bool {
    url.starts_with("https://meet.google.com/") || url.contains("meet.google.com")
}

pub fn is_internal_url(url: &str) -> bool {
    if url == "about:blank" {
        return true;
    }

    let google_domains = [
        "https://mail.google.com",
        "https://chat.google.com",
        "https://accounts.google.com",
        "https://accounts.youtube.com",
        "https://myaccount.google.com",
        "https://drive.google.com",
        "https://docs.google.com",
        "https://sheets.google.com",
        "https://slides.google.com",
        "https://ogs.google.com",
        "https://ssl.google.com",
        "https://workspace.google.com",
        "https://studio.workspace.google.com",
        "https://contacts.google.com",
        "https://calendar.google.com",
        "https://meet.google.com",
        "https://hangouts.google.com",
        "https://plus.google.com",
        "https://photos.google.com",
        "https://maps.google.com",
        "https://www.google.com",
        "https://google.com",
    ];

    for domain in &google_domains {
        if url.starts_with(domain) {
            return true;
        }
    }
    false
}

pub fn is_internal_url_with_auth(url: &str, third_party_auth_mode: bool) -> bool {
    if is_internal_url(url) {
        return true;
    }

    if third_party_auth_mode && crate::auth::is_third_party_auth_url(url, true) {
        return true;
    }

    false
}

pub fn process_url_for_navigation(url: &str) -> Option<String> {
    if let Some(cleaned) = clean_google_redirect_url(url) {
        return Some(cleaned);
    }

    if is_google_meet_link(url) {
        return Some(url.to_string());
    }

    None
}
