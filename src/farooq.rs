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
    bail,
    config::{keys, Config, LocalConfig, APP_NAME, DEFAULT_SETTINGS},
    log,
    protobuf::Enum,
    rendezvous_proto::{control_permissions::Permission, ControlPermissions},
    tls::TlsType,
    tokio, ResultType,
};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex, Once,
    },
    time::Duration,
};

/// Product name. No spaces: it is also used for the Windows service name,
/// data folders and the tray tooltip.
pub const APP_DISPLAY_NAME: &str = "FarooqRemote";

/// Farooq Remote 1.1.8 (owner 2 Nov 2026: «ver number is different in about, in file name, on site»): the product's own
/// version, the only place it is written. The RustDesk core keeps crate::VERSION (1.4.9) because the protocol compares
/// it with other computers; everything a person sees shows this number instead: About, Windows «Apps» (DisplayVersion),
/// the exe's file version, the installer's file name and the release tag (the Windows workflow reads it from this line
/// and stops when a release tag does not match), and the site (version.json and the download pages).
pub const FR_VERSION: &str = "1.1.9";

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

/// Farooq Remote 1.1.7 (owner 2 Nov 2026: «set easy access with menu to select which part should accessible for easy
/// access» — his choice: the work permissions): what a session that came in by easy access may do on this computer.
/// «full» (or empty) = everything, «view» = only see the screen, «custom» = the switches below. These replace the general
/// permissions for easy-access sessions only (Settings → Security → Password, under the easy-access box); sessions with
/// a password keep the general ones.
pub const OPTION_EASY_MODE: &str = "fr-easy-mode";
/// «custom»: the option `fr-easy-perm-<name>` = "N" switches that permission off for easy-access sessions.
pub const EASY_PERM_PREFIX: &str = "fr-easy-perm-";
pub const EASY_PERMS: [(&str, Permission); 12] = [
    ("keyboard", Permission::keyboard),
    ("remote-printer", Permission::remote_printer),
    ("clipboard", Permission::clipboard),
    ("file", Permission::file),
    ("audio", Permission::audio),
    ("camera", Permission::camera),
    ("terminal", Permission::terminal),
    ("tunnel", Permission::tunnel),
    ("restart", Permission::restart),
    ("recording", Permission::recording),
    ("block-input", Permission::block_input),
    ("privacy-mode", Permission::privacy_mode),
];

/// The permissions of an easy-access session in the format of ControlPermissions (2 bits each: 1 = off, 2 = on), so the
/// usual permission checks of src/server/connection.rs apply them.
pub fn easy_control_permissions() -> ControlPermissions {
    let mode = Config::get_option(OPTION_EASY_MODE);
    let mut bits: u64 = 0;
    for (name, p) in EASY_PERMS.iter() {
        let on = match mode.as_str() {
            "view" => false,
            "custom" => Config::get_option(&format!("{}{}", EASY_PERM_PREFIX, name)) != "N",
            _ => true,
        };
        let index = p.value() as u64;
        bits |= (if on { 2u64 } else { 1u64 }) << (index * 2);
    }
    ControlPermissions {
        permissions: bits,
        ..Default::default()
    }
}

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
/// 1.1.8: the error says what went wrong — `Some(http status)` when the server answered with an error, `None` when it
/// could not be reached or the answer was not readable.
async fn easy_post(
    url: &str,
    body: &serde_json::Value,
    bearer: Option<&str>,
) -> Result<serde_json::Value, Option<u16>> {
    let want = reqwest::Url::parse(url).map_err(|_| None)?;
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
                    return Err(Some(resp.status().as_u16()));
                }
                let text = resp.text().await.map_err(|_| None)?;
                return serde_json::from_str(&text).map_err(|_| None);
            }
            Err(e) => {
                log::warn!("easy access: request failed ({:?}): {}", tls, e);
                if e.is_timeout() {
                    return Err(None);
                }
            }
        }
    }
    Err(None)
}

/// Farooq Remote 1.1.8 (owner 2 Nov 2026: «even easy access it ask you password» — he could not see why): when easy
/// access was not used, the password question says why, in his language (keys of src/lang/*.rs). The reasons found before
/// a ticket was sent are shown only for a computer that easy access has reached before (peer option `PEER_EASY_OK`), so
/// a connection to someone else's computer shows the usual question; the reasons after a ticket was sent are always
/// shown (the ticket proves that computer offers easy access to this account).
pub const WHY_SIGNED_OUT: &str = "fr-easy-why-signed-out";
pub const WHY_INSECURE: &str = "fr-easy-why-insecure";
pub const WHY_SERVER: &str = "fr-easy-why-server";
pub const WHY_NOT_OFFERED: &str = "fr-easy-why-not-offered";
pub const WHY_REFUSED: &str = "fr-easy-why-refused";
pub const WHY_NO_ANSWER: &str = "fr-easy-why-no-answer";
/// Peer option (stored with that computer's settings on the controller): easy access has let this computer in before.
pub const PEER_EASY_OK: &str = "fr-easy-ok";

/// Why no ticket was asked for on a connection that is not end-to-end encrypted.
pub fn easy_why_insecure() -> &'static str {
    if LocalConfig::get_option("access_token").is_empty() {
        WHY_SIGNED_OUT
    } else {
        WHY_INSECURE
    }
}

/// Controller: a one-time ticket to reach `peer_id` without its password — only when signed in and when that computer
/// allows easy access and is signed in to the same account (the account server decides); otherwise the reason (one of
/// the `WHY_` keys) and the password is asked as before. `target_pk` is the other computer's key, checked through the ID
/// server, of this end-to-end encrypted connection; the ticket is valid only for that key.
pub async fn easy_ticket(peer_id: &str, target_pk: &[u8]) -> Result<String, &'static str> {
    let token = LocalConfig::get_option("access_token");
    if token.is_empty() {
        return Err(WHY_SIGNED_OUT);
    }
    if peer_id.is_empty() || target_pk.len() != 32 {
        return Err(WHY_INSECURE);
    }
    let url = easy_api_url("/api/fr/easy/ticket").ok_or(WHY_SERVER)?;
    let body = serde_json::json!({
        "id": peer_id,
        "from": Config::get_id(),
        "pk": crate::encode64(target_pk),
    });
    let v = match easy_post(&url, &body, Some(token.as_str())).await {
        Ok(v) => v,
        Err(Some(401)) => return Err(WHY_SIGNED_OUT),
        Err(_) => return Err(WHY_SERVER),
    };
    if v["easy"].as_bool() != Some(true) {
        log::info!(
            "easy access: no ticket for {} (not offered by the account server)",
            peer_id
        );
        return Err(WHY_NOT_OFFERED);
    }
    let ticket = v["ticket"].as_str().unwrap_or_default().to_owned();
    if ticket.len() != 64 || !ticket.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(WHY_SERVER);
    }
    log::info!("easy access: ticket for {}", peer_id);
    Ok(ticket)
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
    let Ok(v) = easy_post(&url, &body, None).await else {
        log::warn!("easy access: no answer from the account server");
        return false;
    };
    let answered = v["ok"].as_bool() == Some(true);
    let user_ok = v["user"]
        .as_str()
        .map(|u| u.trim().to_lowercase() == owner)
        .unwrap_or(false);
    let from_ok = v["from"].as_str() == Some(from_id);
    let pk_ok = v["pk"].as_str() == Some(my_pk.as_str());
    let ok = answered && user_ok && from_ok && pk_ok;
    if ok {
        log::info!("easy access from {}: accepted", from_id);
    } else {
        log::info!(
            "easy access from {}: refused (ticket {}, account {}, computer {}, key {})",
            from_id,
            if answered { "valid" } else { "not valid" },
            if user_ok { "ok" } else { "not the owner" },
            if from_ok { "ok" } else { "other" },
            if pk_ok { "ok" } else { "other" }
        );
    }
    ok
}

/// "1.2.3" → (1, 2, 3); anything else (letters, signs, spaces, more or fewer parts) → None.
pub fn parse_version(v: &str) -> Option<(u32, u32, u32)> {
    let parts: Vec<&str> = v.split('.').collect();
    if parts.len() != 3 {
        return None;
    }
    let mut n = [0u32; 3];
    for (i, p) in parts.iter().enumerate() {
        if p.is_empty() || p.len() > 4 || !p.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        n[i] = p.parse().ok()?;
    }
    Some((n[0], n[1], n[2]))
}

/// Is `a` a newer version than `b`? (false when either is not a version)
pub fn is_newer(a: &str, b: &str) -> bool {
    match (parse_version(a), parse_version(b)) {
        (Some(a), Some(b)) => a > b,
        _ => false,
    }
}

/// The language the app shows ("en", "ur", "ar"; every other language is "en" here): read through the translation
/// tables, so it follows Settings → General → Language exactly like the texts do.
fn ui_lang() -> String {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let l = crate::lang::translate("fr-lang-code".to_owned());
        if l == "ur" || l == "ar" {
            return l;
        }
    }
    "en".to_owned()
}

/// The download page of the site in the app's language (always this site: the address never comes from the network).
pub fn download_page() -> String {
    match ui_lang().as_str() {
        "ur" => "https://www.farooqmughal.net/ur/farooqremote/".to_owned(),
        "ar" => "https://www.farooqmughal.net/ar/farooqremote/".to_owned(),
        _ => "https://www.farooqmughal.net/farooqremote/".to_owned(),
    }
}

/// Farooq Remote 1.1.8 (owner 2 Nov 2026: «if it is old version it should detect online if new version available ask
/// user to update»): the main window asks the site for the newest version when it starts and every 12 hours. The site
/// generator writes version.json together with the download pages, so the app announces a version exactly when the
/// site offers it. Only the version number is read from the answer; the «Download» button always opens our own page
/// (`download_page`), so a false answer could at most show a needless card. Off when the user unticks «Check for
/// software update on startup» (Settings → General).
const UPDATE_INFO_URL: &str = "https://www.farooqmughal.net/farooqremote/version.json";
const UPDATE_SITE_HOSTS: &[&str] = &["www.farooqmughal.net", "farooqmughal.net"];
const UPDATE_CHECK_EVERY: Duration = Duration::from_secs(12 * 3600);
const UPDATE_INFO_MAX_BYTES: usize = 4096;
static UPDATE_CHECK_STARTED: AtomicBool = AtomicBool::new(false);

lazy_static::lazy_static! {
    static ref NEW_VERSION: Mutex<String> = Default::default();
}

/// The newer version the site offers, or "" (none, or not known yet).
pub fn new_version() -> String {
    NEW_VERSION.lock().unwrap().clone()
}

/// Started by the main window (crate::common::check_software_update); a second call only repeats the known answer.
pub fn start_update_check() {
    if UPDATE_CHECK_STARTED.swap(true, Ordering::SeqCst) {
        push_update_event();
        return;
    }
    std::thread::spawn(|| loop {
        match fetch_newest_version() {
            Ok(v) => {
                let newer = is_newer(&v, FR_VERSION);
                log::info!(
                    "update check: the site offers {}, this is {}{}",
                    v,
                    FR_VERSION,
                    if newer { " - a newer version" } else { "" }
                );
                *NEW_VERSION.lock().unwrap() = if newer { v } else { String::new() };
                push_update_event();
            }
            Err(e) => log::info!("update check: {}", e),
        }
        std::thread::sleep(UPDATE_CHECK_EVERY);
    });
}

/// Tells the main window (flutter/lib/common.dart checkUpdate) about a newer version: the home page shows a card and the
/// app asks once per version.
fn push_update_event() {
    #[cfg(feature = "flutter")]
    {
        if new_version().is_empty() {
            return;
        }
        let data = serde_json::json!({
            "name": "check_software_update_finish",
            "url": download_page(),
        });
        let _ = crate::flutter::push_global_event(crate::flutter::APP_TYPE_MAIN, data.to_string());
    }
}

/// Reads the newest version from the site: https only, the certificate always checked, redirects only within the site
/// (at most 3), a time limit and a small answer, and the version must look like "1.2.3".
#[tokio::main(flavor = "current_thread")]
async fn fetch_newest_version() -> ResultType<String> {
    let limit = Duration::from_secs(15);
    let mut last = String::new();
    for tls in [TlsType::Rustls, TlsType::NativeTls] {
        let req = crate::hbbs_http::create_http_client_async_same_site(tls, UPDATE_SITE_HOSTS, 3)
            .get(UPDATE_INFO_URL)
            .timeout(limit);
        match req.send().await {
            Ok(mut resp) => {
                let host_ok = resp.url().scheme() == "https"
                    && resp
                        .url()
                        .host_str()
                        .map(|h| UPDATE_SITE_HOSTS.contains(&h))
                        .unwrap_or(false);
                if !resp.status().is_success() || !host_ok {
                    bail!("the site answered {} ({})", resp.status(), resp.url());
                }
                let mut body: Vec<u8> = Vec::new();
                while let Some(chunk) = resp.chunk().await? {
                    body.extend_from_slice(&chunk);
                    if body.len() > UPDATE_INFO_MAX_BYTES {
                        bail!("the answer is too long");
                    }
                }
                let v: serde_json::Value = serde_json::from_slice(&body)?;
                let ver = v["version"].as_str().unwrap_or_default().trim().to_owned();
                if parse_version(&ver).is_none() {
                    bail!("no version in the answer");
                }
                return Ok(ver);
            }
            Err(e) => {
                last = format!("{:?}: {}", tls, e);
                if e.is_timeout() {
                    break;
                }
            }
        }
    }
    bail!("the site could not be reached ({})", last)
}

/// Farooq Remote 1.1.8 (owner 2 Nov 2026: «if your App build does not match … it should give clear message»): the
/// computer being reached tells its version in PeerInfo.platform_additions (src/server/connection.rs); 1.1.7 and older,
/// and other programs, tell none.
pub const PEER_INFO_VERSION_KEY: &str = "fr_version";
/// Peer option: the version pair already shown for that computer ("its|mine"), so the note comes once per change.
pub const PEER_VERSION_NOTED: &str = "fr-version-noted";

/// The other computer's Farooq Remote version from its PeerInfo.platform_additions ("" when it tells none).
pub fn peer_fr_version(platform_additions: &str) -> String {
    serde_json::from_str::<serde_json::Value>(platform_additions)
        .ok()
        .and_then(|v| v[PEER_INFO_VERSION_KEY].as_str().map(|s| s.trim().to_owned()))
        .filter(|s| parse_version(s).is_some())
        .unwrap_or_default()
}

/// A note in the app's language when the other computer runs another version: (title key, text, link to the download
/// page); None when both are the same.
pub fn peer_version_note(peer_version: &str) -> Option<(&'static str, String, String)> {
    let key = if peer_version.is_empty() {
        "fr-peer-version-unknown"
    } else if is_newer(FR_VERSION, peer_version) {
        "fr-peer-version-older"
    } else if is_newer(peer_version, FR_VERSION) {
        "fr-peer-version-newer"
    } else {
        return None;
    };
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    let text = crate::lang::translate(key.to_owned());
    #[cfg(any(target_os = "android", target_os = "ios"))]
    let text = key.to_owned();
    let text = text
        .replace("{peer}", peer_version)
        .replace("{mine}", FR_VERSION);
    Some(("fr-peer-version-title", text, download_page()))
}

/// Farooq Remote 1.1.8: RustDesk itself is installed on this computer. When it is set to the Farooq Remote server it has
/// the same ID (both take it from the computer), and connections reach either program at random — the owner's home-02
/// (2 Nov 2026: «Insecure connection», endless «connecting», a password question). The home page says so, once.
pub fn rustdesk_installed() -> bool {
    #[cfg(windows)]
    {
        use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        return hklm
            .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\RustDesk")
            .is_ok()
            || hklm
                .open_subkey("SYSTEM\\CurrentControlSet\\Services\\RustDesk")
                .is_ok();
    }
    #[cfg(not(windows))]
    false
}

/// Farooq Remote 1.1.9 (owner 2 Nov 2026 20:41: «when install or uninstall App ask you do you want keep setting with
/// password yes/or No»). The settings live outside the program folder, so uninstalling (upstream) never removed them and
/// a new installation brought them back: the service's config (ID, key, permanent password, options such as easy access)
/// in %SystemRoot%\ServiceProfiles\LocalService\AppData\Roaming\FarooqRemote\config, and each user's config (account
/// sign-in, saved passwords of other computers, language) in %APPDATA%\FarooqRemote\config. Now the uninstaller asks
/// (Yes = keep, No = delete both, Cancel = stop), and the install page asks when settings are already there (No = the
/// installation starts like new: the old ones are deleted and not imported).
pub const INSTALL_OPTION_RESET: &str = "fr-reset";

/// The service's settings folder.
pub fn service_settings_dir() -> std::path::PathBuf {
    let root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_owned());
    std::path::Path::new(&root)
        .join("ServiceProfiles")
        .join("LocalService")
        .join("AppData")
        .join("Roaming")
        .join(APP_NAME.read().unwrap().as_str())
        .join("config")
}

/// This user's settings folder (the folder of the running app's own config file).
pub fn user_settings_dir() -> Option<std::path::PathBuf> {
    Config::file().parent().map(|p| p.to_path_buf())
}

/// Are there settings from an earlier or the current installation? When it can not be told (a folder the user may not
/// look into), «yes», so the install page rather asks once too often than deletes without asking.
pub fn has_earlier_settings() -> bool {
    #[cfg(windows)]
    {
        use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};
        let key = format!(
            "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\{}",
            APP_NAME.read().unwrap()
        );
        if RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(key).is_ok() {
            return true;
        }
        match std::fs::metadata(service_settings_dir()) {
            Ok(_) => return true,
            Err(e) if e.kind() != std::io::ErrorKind::NotFound => return true,
            _ => {}
        }
    }
    !LocalConfig::get_option("access_token").is_empty()
        || !hbb_common::config::PeerConfig::peers(None).is_empty()
}

/// The lines of the (elevated) install / uninstall script that delete the settings; empty when a path is not usable.
pub fn delete_settings_cmds() -> String {
    let mut dirs = vec![service_settings_dir()];
    if let Some(d) = user_settings_dir() {
        dirs.push(d);
    }
    let mut out = String::new();
    for d in dirs {
        let s = d.to_string_lossy().to_string();
        // only a folder named "config" inside a «FarooqRemote» folder, and nothing a script could misread
        let name_ok = s.ends_with(&format!("\\{}\\config", APP_NAME.read().unwrap()));
        if !name_ok || s.contains('"') || s.contains('%') || s.len() < 10 {
            log::warn!("settings folder not deleted (unexpected path): {}", s);
            continue;
        }
        out.push_str(&format!("\nif exist \"{s}\" rd /s /q \"{s}\"\n"));
    }
    out
}

/// What the person chose when uninstalling.
#[derive(Debug, PartialEq)]
pub enum KeepSettings {
    Keep,
    Delete,
    Cancel,
}

/// Asks, in the app's language, whether to keep this computer's settings (Windows question box: Yes / No / Cancel).
#[cfg(windows)]
pub fn ask_keep_settings_on_uninstall() -> KeepSettings {
    use winapi::um::winuser::{
        MessageBoxW, IDCANCEL, IDNO, MB_ICONQUESTION, MB_RIGHT, MB_RTLREADING, MB_SETFOREGROUND,
        MB_TOPMOST, MB_YESNOCANCEL,
    };
    let lang = ui_lang();
    let text = crate::lang::translate("fr-uninstall-keep-tip".to_owned());
    let caption = crate::lang::translate("fr-uninstall-title".to_owned());
    let w = |s: &str| s.encode_utf16().chain(std::iter::once(0)).collect::<Vec<u16>>();
    let (text, caption) = (w(text.as_str()), w(caption.as_str()));
    let mut flags = MB_YESNOCANCEL | MB_ICONQUESTION | MB_TOPMOST | MB_SETFOREGROUND;
    if lang == "ur" || lang == "ar" {
        flags |= MB_RTLREADING | MB_RIGHT;
    }
    let r = unsafe { MessageBoxW(std::ptr::null_mut(), text.as_ptr(), caption.as_ptr(), flags) };
    let choice = if r == IDNO {
        KeepSettings::Delete
    } else if r == IDCANCEL || r == 0 {
        KeepSettings::Cancel
    } else {
        KeepSettings::Keep
    };
    log::info!("uninstall: settings {:?}", choice);
    choice
}

/// Farooq Remote 1.1.9 (owner 2 Nov 2026 20:41, the red shield of Home 2: «Direct and unencrypted connection»): a
/// computer sends its key to the ID server only the first time, so after another program with the same ID (the old
/// RustDesk on Home 2) the ID server kept that program's key, and every connection to the computer stayed unencrypted
/// («Insecure connection»). Now the key is sent again once after each new version. The ID server keeps it when it comes
/// from the same computer and network (same uuid and address).
pub fn resend_key_once_per_version() {
    const SENT: &str = "fr-key-sent-version";
    if Config::get_option(SENT) != FR_VERSION {
        Config::set_key_confirmed(false);
        Config::set_option(SENT.to_owned(), FR_VERSION.to_owned());
        log::info!("the key goes to the ID server again ({})", FR_VERSION);
    }
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
