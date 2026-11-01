// Farooq Remote branding layer.
//
// Farooq Remote is a build of RustDesk (AGPL-3.0, https://github.com/rustdesk/rustdesk)
// developed by Mohammad Farooq — https://www.farooqmughal.net
// Downloads and self-hosting guides: https://www.farooqmughal.net/farooqremote
//
// This file is the single place where the product is renamed and pointed at the
// Farooq Remote ID/relay server. Everything else in the code base stays upstream.
//
// How it works (all upstream mechanisms, nothing patched elsewhere):
//   * APP_NAME != "RustDesk" switches the app into upstream's "custom client" mode:
//     every translated string that says "RustDesk" shows the new name, the window
//     title / service name / data folders use it, and the upstream update-checker
//     is turned off.
//   * The ID server, relay, public key and account (API) server are written to
//     DEFAULT_SETTINGS, i.e. the *default* values of Settings → Network. A user can
//     still overwrite them there (that is the "use your own server" mode); clearing
//     the field falls back to these defaults again.

use hbb_common::{
    config::{keys, Config, LocalConfig, APP_NAME, DEFAULT_SETTINGS},
    log,
    tls::TlsType,
};
use std::sync::Once;

/// Product name. No spaces: it is also used for the Windows service name,
/// data folders and the tray tooltip.
pub const APP_DISPLAY_NAME: &str = "FarooqRemote";

/// Farooq Remote public ID server (hbbs) — the company domain (1.1.0, owner 1 Nov 2026:
/// «we will use now farooqmughal.net for everything»); remote.farooqmughal.net → the Oracle VM.
pub const ID_SERVER: &str = "remote.farooqmughal.net";
/// Relay server (hbbr).
pub const RELAY_SERVER: &str = "remote.farooqmughal.net:21117";
/// Account server (user name + password, address book, my devices) — 1.1.0, Phase 5.
/// Served by nginx on the same VM (lejianwen/rustdesk-api, MIT); https only.
pub const API_SERVER: &str = "https://remote.farooqmughal.net";
/// Public key of the Farooq Remote ID server (data/id_ed25519.pub on the VM).
pub const SERVER_PUBLIC_KEY: &str = "11rU8HpBVbadu59plPSdMpFvkaJ2amnfcB+rJlZ2EWs=";

pub const WEBSITE_URL: &str = "https://www.farooqmughal.net";
pub const DOWNLOAD_URL: &str = "https://www.farooqmughal.net/farooqremote";
pub const PRIVACY_URL: &str = "https://www.farooqmughal.net/farooqremote/privacy";
pub const SOURCE_URL: &str = "https://github.com/FarooqStars/FarooqRemote-Client";

/// Farooq Remote 1.1.4 (owner 1 Nov 2026: «i install app on 2 different pc and it installed but not connecting» —
/// «Failed to secure tcp: deadline has elapsed» when an account is logged in).
/// Our ID server is the open-source hbbs: it never starts the «secure TCP» key exchange and it does not use the account
/// token. Upstream, when an account is logged in, waits for that key exchange before it sends the token to the ID
/// server, so every connection failed after the time-out. Farooq Remote keeps the account token away from the ID server
/// instead (nothing there needs it), so no key exchange is awaited; the connection between the two computers stays
/// end-to-end encrypted exactly as before. Set to true only with an ID server that sends the key exchange.
pub const ID_SERVER_SECURE_TCP: bool = false;

/// The account token as it may go to the ID server: empty unless the ID server secures the TCP connection.
pub fn token_for_id_server(token: String) -> String {
    if ID_SERVER_SECURE_TCP {
        token
    } else {
        String::new()
    }
}

/// Farooq Remote 1.1.5 — «easy access», like TeamViewer's (owner 1 Nov 2026: «if i already use app login password for
/// both side all the devices which visible under that account it should not ask password there should extra option in
/// password windows for this feature»).
/// On the computer to be reached, its owner ticks «Easy access» once while signed in (Settings → Security → Password); the
/// account name is kept in OPTION_EASY_OWNER and the heartbeat tells the account server (src/hbbs_http/sync.rs). From his
/// other computers, signed in to the same account and without a saved password, the app gets a one-time ticket
/// (`easy_ticket`) and sends it in the end-to-end encrypted login (LoginRequest.fr_ticket, field 99, added to the
/// protocol at build time). The computer asks the server about it once (`easy_check`) and lets the session in only for
/// its own stored account and the computer that connected; anything else gets the password question as before.
///
/// 1.1.6 (security review of 1.1.5, before it was given to anyone):
///   * both calls use `easy_post`: https only, the server's certificate always checked, no «accept any certificate»
///     retry and no detour through the ID server, so nobody in between can fake the server's answer or read the token;
///   * a ticket is asked for only on an end-to-end encrypted connection, once per connection, and it is tied to the
///     other computer's key as checked through the ID server (`target_pk`); the computer being reached accepts it only
///     when that key is its own, so a ticket can not be carried to another session;
///   * the account server gives a ticket only for a computer that is signed in to the same account (accounts 2.7-10);
///   * settings sent in a heartbeat answer («strategy») are not applied at all (`ACCEPT_SERVER_STRATEGY`).
/// Server side: FarooqRemote-Server farooqremote/accounts/api/lib/freasy (accounts 2.7-10).
pub const OPTION_EASY_ACCESS: &str = "fr-easy-access";
/// The account (user name, lower case) chosen when easy access was ticked.
pub const OPTION_EASY_OWNER: &str = "fr-easy-owner";
const EASY_HTTP_TIMEOUT_MS: u64 = 6_000;

/// Farooq Remote 1.1.6: upstream lets the account server change this computer's settings through the heartbeat answer
/// («strategy», a RustDesk Server Pro feature). The Farooq Remote account server never sends one, and such an answer
/// could switch off security settings (or easy access's own), so it is ignored. Set to true only for a server that
/// really sends strategies.
pub const ACCEPT_SERVER_STRATEGY: bool = false;

/// This computer lets its owner's account in without a password.
pub fn easy_access_on() -> bool {
    Config::get_option(OPTION_EASY_ACCESS) == "Y"
        && !Config::get_option(OPTION_EASY_OWNER).trim().is_empty()
}

fn easy_api_url(path: &str) -> Option<String> {
    let api = crate::common::get_api_server(
        Config::get_option("api-server"),
        Config::get_option("custom-rendezvous-server"),
    );
    if api.is_empty() || crate::is_public(&api) {
        return None;
    }
    let api = api.trim_end_matches('/');
    if !api.to_ascii_lowercase().starts_with("https://") {
        log::warn!("easy access needs an https account server; it stays off");
        return None;
    }
    Some(format!("{}{}", api, path))
}

/// A strict JSON POST for easy access: https only (see `easy_api_url`), the certificate is always checked (rustls with
/// the system's roots first, then the system's own TLS — never «accept invalid certificates»), no fallback through the
/// ID server, no redirect followed, one time limit, and only a 2xx answer from that very address counts.
async fn easy_post(
    url: &str,
    body: &serde_json::Value,
    bearer: Option<&str>,
) -> Option<serde_json::Value> {
    let want = reqwest::Url::parse(url).ok()?;
    let body = body.to_string();
    let limit = std::time::Duration::from_millis(EASY_HTTP_TIMEOUT_MS);
    for tls in [TlsType::Rustls, TlsType::NativeTls] {
        let mut req = crate::hbbs_http::create_http_client_async_no_redirect(tls)
            .post(url)
            .header("Content-Type", "application/json")
            .timeout(limit)
            .body(body.clone());
        if let Some(t) = bearer {
            req = req.header("Authorization", format!("Bearer {}", t));
        }
        match req.send().await {
            Ok(resp) => {
                if !resp.status().is_success() || resp.url() != &want {
                    log::warn!("easy access: the account server answered {}", resp.status());
                    return None;
                }
                let text = resp.text().await.ok()?;
                return serde_json::from_str(&text).ok();
            }
            Err(e) => {
                log::warn!("easy access: request failed ({:?}): {}", tls, e);
                if e.is_timeout() {
                    return None;
                }
            }
        }
    }
    None
}

/// Controller: a one-time ticket to reach `peer_id` without its password — only when signed in and when that computer
/// allows easy access and is signed in to the same account (the account server decides); `None` otherwise, and the
/// password is asked as before. `target_pk` is the other computer's key, checked through the ID server, of this
/// end-to-end encrypted connection; the ticket is valid only for that key.
pub async fn easy_ticket(peer_id: &str, target_pk: &[u8]) -> Option<String> {
    let token = LocalConfig::get_option("access_token");
    if token.is_empty() || peer_id.is_empty() || target_pk.len() != 32 {
        return None;
    }
    let url = easy_api_url("/api/fr/easy/ticket")?;
    let body = serde_json::json!({
        "id": peer_id,
        "from": Config::get_id(),
        "pk": crate::encode64(target_pk),
    });
    let v = easy_post(&url, &body, Some(token.as_str())).await?;
    if v["easy"].as_bool() != Some(true) {
        return None;
    }
    let ticket = v["ticket"].as_str()?.to_owned();
    if ticket.len() != 64 || !ticket.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    log::info!("easy access: ticket for {}", peer_id);
    Some(ticket)
}

/// Computer being reached: is `ticket` from this computer's owner, presented by the computer `from_id`, for a session
/// with this computer's own key? Asks the account server once (the ticket is used up either way); any doubt means «no».
pub async fn easy_check(ticket: &str, from_id: &str) -> bool {
    if !easy_access_on()
        || ticket.len() != 64
        || !ticket.chars().all(|c| c.is_ascii_hexdigit())
        || from_id.is_empty()
    {
        return false;
    }
    let Some(url) = easy_api_url("/api/fr/easy/check") else {
        return false;
    };
    let owner = Config::get_option(OPTION_EASY_OWNER).trim().to_lowercase();
    let my_pk = crate::encode64(Config::get_key_pair().1);
    let body = serde_json::json!({ "ticket": ticket, "id": Config::get_id() });
    let Some(v) = easy_post(&url, &body, None).await else {
        log::warn!("easy access: no answer from the account server");
        return false;
    };
    let ok = v["ok"].as_bool() == Some(true)
        && v["user"]
            .as_str()
            .map(|u| u.trim().to_lowercase() == owner)
            .unwrap_or(false)
        && v["from"].as_str() == Some(from_id)
        && v["pk"].as_str() == Some(my_pk.as_str());
    log::info!(
        "easy access from {}: {}",
        from_id,
        if ok { "accepted" } else { "refused" }
    );
    ok
}

static APPLIED: Once = Once::new();

/// Apply the branding. Safe to call more than once; runs only the first time.
pub fn apply() {
    APPLIED.call_once(|| {
        *APP_NAME.write().unwrap() = APP_DISPLAY_NAME.to_owned();
        let mut defaults = DEFAULT_SETTINGS.write().unwrap();
        defaults.insert(
            keys::OPTION_CUSTOM_RENDEZVOUS_SERVER.to_owned(),
            ID_SERVER.to_owned(),
        );
        defaults.insert(
            keys::OPTION_RELAY_SERVER.to_owned(),
            RELAY_SERVER.to_owned(),
        );
        defaults.insert(keys::OPTION_KEY.to_owned(), SERVER_PUBLIC_KEY.to_owned());
        defaults.insert(keys::OPTION_API_SERVER.to_owned(), API_SERVER.to_owned());
        log::info!(
            "{} branding applied: id server {}, relay {}, account server {}",
            APP_DISPLAY_NAME,
            ID_SERVER,
            RELAY_SERVER,
            API_SERVER
        );
    });
}
