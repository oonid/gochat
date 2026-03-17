use std::env;

const DEFAULT_THIRD_PARTY_URLS: &[&str] = &[
    "https://accounts.google.com",
    "https://accounts.youtube.com",
    "https://mail.google.com/ServiceLogin",
    "https://mail.google.com/chat",
    "https://chat.google.com",
    "https://login.microsoftonline.com",
    "https://login.okta.com",
    "https://okta.com",
    "https://auth0.com",
    "https://sso.",
    "https://idp.",
    "https://federated.",
    "https://onelogin.com",
    "https://pingone.com",
    "https://pingidentity.com",
    "https://duo.com",
];

pub fn get_third_party_auth_urls() -> Vec<String> {
    let mut urls: Vec<String> = DEFAULT_THIRD_PARTY_URLS
        .iter()
        .map(|s| s.to_string())
        .collect();

    if let Ok(custom_urls) = env::var("NO_REDIRECT_URL") {
        for url in custom_urls.split(',') {
            let trimmed = url.trim();
            if !trimmed.is_empty() {
                let url_with_scheme =
                    if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
                        trimmed.to_string()
                    } else {
                        format!("https://{}", trimmed)
                    };
                urls.push(url_with_scheme);
            }
        }
    }

    urls
}

pub fn is_third_party_auth_url(url: &str, third_party_mode: bool) -> bool {
    if !third_party_mode {
        return false;
    }

    let auth_urls = get_third_party_auth_urls();
    auth_urls
        .iter()
        .any(|pattern| url.starts_with(pattern) || url.contains(&pattern[8..]))
}
