# Farooq Remote — client branding notes

Farooq Remote is a build of [RustDesk](https://github.com/rustdesk/rustdesk) (tag 1.4.9)
developed by **Mohammad Farooq** — <https://www.farooqmughal.net>.
Downloads and guides for everyone: <https://www.farooqmughal.net/farooqremote>.

Licence: AGPL-3.0, unchanged from upstream (see `LICENCE`). RustDesk copyright
notices are kept. This repository *is* the source-availability required by the
licence; every published binary is built from a tagged commit of this repository
by the workflow below.

## What differs from upstream (branch `farooq/develop`)

| File | Change |
|---|---|
| `src/farooq.rs` (new) | The single branding point: app name `FarooqRemote`, default ID server / relay / public key / account (API) server, website URLs. |
| `src/lib.rs`, `src/common.rs` | `pub mod farooq;` and one call `crate::farooq::apply()` at the top of `load_custom_client()` (runs on desktop and mobile start-up). |
| `src/lang/ur.rs` (new), `src/lang.rs` | Urdu language added (764 strings; 1.1.2: +3 for the login window). |
| `flutter/lib/common/widgets/login.dart`, `src/lang/ar.rs` | 1.1.2: «Create account» and «Forgot password?» under the login button (they open the account website's sign-up / forgot-password page in the app's language); the user-name box says «E-mail or user name». |
| `src/lang/en.rs` | `powered_by_me` → "Based on RustDesk · developed by Mohammad Farooq". |
| `flutter/lib/desktop/pages/desktop_setting_page.dart` | About card: developer line, website/privacy/help links, source URL. |
| `flutter/assets/fonts/*` (new), `flutter/pubspec.yaml`, `flutter/lib/common.dart`, `flutter/lib/main.dart` | 1.1.1: Naskh font "FS Urdu" (Noto Naskh Arabic 2.021, OFL) for Urdu / Arabic / Persian; new getters `MyTheme.appLightTheme` / `appDarkTheme` (used by `flutter/lib/main.dart`; the upstream `lightTheme` / `darkTheme` lines stay as they are, because the Flutter 3.44 patch script finds them by those words); `Locale('ur')` in the supported locales. |
| `flutter/lib/common.dart`, `flutter/lib/desktop/pages/desktop_home_page.dart`, `flutter/lib/mobile/pages/settings_page.dart` | rustdesk.com links → farooqmughal.net (1.1.0; before: farooqmusic.com / mymandoob.com). |
| `flutter/windows/runner/Runner.rc` | Company / product / description / copyright strings. |
| `flutter/android/app/src/main/AndroidManifest.xml` | App label "Farooq Remote". |
| `res/*`, `flutter/assets/icon.svg`, `flutter/windows/runner/resources/app_icon.ico`, Android mipmaps, `flutter/macos/Runner/AppIcon.icns`, iOS AppIcon set | Farooq Remote icons. |
| `flutter/assets/logo.png` (new, 1.1.3), `res/farooq-branding/make_banner.py` | The owner's banner (icon + name on teal) at the top of the main window. |
| `src/client.rs` (one line in `_start_inner`), `src/farooq.rs` | 1.1.4: the account token is not sent to the open-source ID server, so a logged-in app connects (no wait for a key exchange the server never sends). |
| `src/farooq.rs` (easy_*), `src/client.rs` (`handle_hash`), `src/server/connection.rs` (login), `src/hbbs_http/sync.rs` (heartbeat), `desktop_setting_page.dart`, `src/lang/{en,ur,ar}.rs`, workflow step for `message.proto` | 1.1.5: easy access — the owner's own account connects without a password (see «1.1.5» below). |
| `src/farooq.rs` (`easy_post`, `ACCEPT_SERVER_STRATEGY`), `src/client.rs` (`fr_easy_pk`), `src/client/io_loop.rs`, `src/server/connection.rs`, `src/hbbs_http/sync.rs`, `src/lang/{en,ur,ar}.rs` | 1.1.6: easy access after a security review — strict https, the ticket tied to the encrypted session, wrong tickets counted, heartbeat «strategy» ignored (see «1.1.6» below). |
| `.github/workflows/farooqremote-windows.yml` (new) | Windows x64 build (manual trigger). |

Everything else is upstream, byte for byte. Upstream workflows are left in place but
are not triggered (they run on upstream's own tags/schedules).

## How the server is wired

`src/farooq.rs` writes the ID server, relay, key and the account (API) server into
`DEFAULT_SETTINGS`, which are the *defaults* of **Settings → Network**. So:

* **Use Farooq's server** — install and it is Ready; nothing to type.
* **Use your own server** — open Settings → Network, put your own ID server / relay /
  key. Clearing the fields returns to the Farooq Remote defaults.

The Windows workflow additionally patches the hard fallback constants in the
`hbb_common` submodule at build time, so even a cleared field can never fall back to
`rs-ny.rustdesk.com`.

## Building

GitHub → Actions → "Farooq Remote - Windows x64 build" → Run workflow.
Optional input `release-tag` (e.g. `v1.1.0`) also creates a GitHub Release with
`farooqremote-1.1.0-x86_64.exe` and `SHA256SUMS.txt`.

## 1.1.0 — accounts (Phase 5, 1 Nov 2026)

* Server name `remote.farooqmughal.net` (the company domain) instead of the raw IP;
  the same hbbs / hbbr and key, so 1.0.x clients keep working.
* Account server `https://remote.farooqmughal.net` (lejianwen/rustdesk-api, MIT, on the
  same VM behind nginx): **Login** with user name + password, a synced address book
  (with saved passwords and tags) and the device list. Registration is off — only the
  owner's account.
* All links → farooqmughal.net (website, download, privacy); source → FarooqStars.

The exe is unsigned: Windows SmartScreen shows "unknown publisher" → *More info* →
*Run anyway*. The Microsoft Store (MSIX) route is planned to avoid this.

## 1.1.1 — Urdu and Arabic in Naskh (1 Nov 2026)

* The owner: «add Arabic & Urdu … even on remote client with URDU naskh fonts». Urdu (Farooq's
  764 strings) and Arabic (upstream) were already in **Settings → General → Language**; now,
  when the app language is Urdu, Arabic or Persian, every text uses the Naskh font **FS Urdu**
  (Noto Naskh Arabic 2.021, hinted TTF, SIL OFL 1.1 — `flutter/assets/fonts/`), the same Urdu
  font as farooqstars.com and farooqmughal.net. English words keep the system font.
* `Locale('ur')` added, so a Windows set to Urdu gets the right-to-left layout as Arabic,
  Persian and Hebrew already do. The layout on an English Windows stays as upstream.
* About → **Help** opens the guide: `farooqmughal.net/ur/farooqremote/help/` when the app is
  in Urdu, otherwise `farooqmughal.net/farooqremote/help/`.

## 1.1.2 — anyone can make an account (1 Nov 2026)

* The owner: «make system from app user can Register this App by Email id and password, and we
  make this app proper working app for any users». The account server (release `accounts-2.7-3`,
  FarooqRemote-Server) now has sign-up with an e-mail address + password, proved by a 6-digit
  e-mail code, and «forgot password» by e-mail code.
* The login window has **Create account** and **Forgot password?** — they open
  `<account server>/_admin/#/register` or `#/forgot` in the browser, in the app's language
  (`?lang=ur|ar|en`). The user-name box says «E-mail or user name».

## 1.1.3 — the owner's banner in the main window (1 Nov 2026)

* The owner sent his branding files «for icons and logos» and chose «the banner in the app window».
  `flutter/assets/logo.png` = his `Installer-header-banner.jpg` (the white icon + «Farooq Remote» on
  his teal) cut to a banner strip with rounded corners (`res/farooq-branding/make_banner.py`). Upstream's
  `loadLogo()` shows `assets/logo.png` at the top of the left pane (at most 300 x 60) when the file exists —
  no Dart change. The app icon and the tray icon stay as they are (made from the same branding files in
  September by `res/farooq-branding/make_icons.py`).

## 1.1.4 — connecting while logged in (1 Nov 2026)

* The owner: «i install app on 2 different pc and it installed but not connecting» — with an account logged in,
  every connection stopped with «Failed to secure tcp: deadline has elapsed».
* Cause: when a token is present, upstream first waits for a «secure TCP» key exchange from the ID server (hbbs)
  before sending the token (`secure_tcp` in `src/client.rs`). Our ID server is the open-source hbbs (1.1.16): it
  never sends that key exchange (only RustDesk Server Pro and forks do) and it does not use the token at all.
* Fix: `src/farooq.rs` `ID_SERVER_SECURE_TCP = false` + `token_for_id_server()`; the first line of
  `Client::_start_inner` (`src/client.rs`) empties the token for the ID-server messages (punch-hole and relay
  request), so no key exchange is awaited and the account token never travels to the ID server. The session between
  the two computers is end-to-end encrypted exactly as before; the account functions (address book, devices) use the
  account server over https as before.

## 1.1.5 — easy access, like TeamViewer's (1 Nov 2026)

* The owner: «if i already use app login password for both side all the devices which visible under that account it
  should not ask password there should extra option in password windows for this feature … make little research on
  TeamViewer and give me same functionality». TeamViewer: assign the device to the account, tick «Grant easy access»
  once on each device; the account then connects without a password, everyone else still needs one.
* Settings → Security → Password: **«Easy access: my account connects without a password»** (Flutter
  `desktop_setting_page.dart` `_frEasyAccess`, words in en/ur/ar). Ticking it needs the account to be signed in; the
  account name is kept in the option `fr-easy-owner` (option `fr-easy-access` = `Y`), both read by the service.
* The heartbeat (`src/hbbs_http/sync.rs`) says `fr_easy` to the account server. A signed-in controller without a saved
  password asks the server for a one-time ticket (`crate::farooq::easy_ticket`, `src/client.rs` `handle_hash`) and sends
  it in the end-to-end encrypted login as `LoginRequest.fr_ticket` (field 99, added to `message.proto` by the workflow at
  build time; unknown to older apps, which ignore it). The computer being reached asks the server once
  (`crate::farooq::easy_check`, `src/server/connection.rs`) and lets the session in only for its own stored account and
  the computer that connected (audit value 9 = easy access); otherwise it answers «Empty Password», so the controller
  asks for the password as before. 2FA, if switched on, is still asked.
* Server side: FarooqRemote-Server `accounts-2.7-9` (lib/freasy, controller api/farooq_easy.go, ci/easy_test.py).
* 1.1.5 was a test build only (not given to anyone); use 1.1.6.

## 1.1.6 — easy access, after a security review (1 Nov 2026)

A review of 1.1.5 (before it was given to anyone) found ways around easy access; 1.1.6 closes them:

* **The server's answer can not be faked.** `easy_ticket` / `easy_check` no longer use upstream `post_request`, which
  retries with «accept any certificate» and can go through the ID server instead. `crate::farooq::easy_post`: https only,
  the certificate always checked (rustls with the system's roots, then the system's own TLS), no detour, 6 s, only a 2xx
  answer counts. The account token is never sent without a checked certificate.
* **A ticket belongs to one encrypted session.** The connection keeps the other computer's key (checked through the ID
  server) only when it is end-to-end encrypted (`src/client/io_loop.rs` → `LoginConfigHandler.fr_easy_pk`);
  `handle_hash` takes it once, so a ticket is asked for at most once per connection and never on an unencrypted one.
  The key goes with the ticket request; the server returns it with the check, and the computer being reached accepts
  the ticket only when it is its own key (`Config::get_key_pair().1`, the key it registers with the ID server).
* **Wrong tickets count like wrong passwords** (`check_failure` / `update_failure` in `src/server/connection.rs`).
* **Settings in a heartbeat answer are ignored** (`ACCEPT_SERVER_STRATEGY = false`, `src/hbbs_http/sync.rs`): upstream
  applies a «strategy» from the account server; ours never sends one, and a faked one could switch security settings
  (or easy access itself) on this computer.
* The tip under the box now says that this computer must also stay signed in to the same account (the server gives a
  ticket only for a computer signed in to the asking account; logging out on it stops easy access until the next login).
* Server side: FarooqRemote-Server `accounts-2.7-10` (the key in the ticket, the computer must be signed in to the same
  account, every ticket request counted in the limit, caps on the memory tables).

## Never

Never remove RustDesk licence or copyright notices · never use RustDesk Pro code ·
never call the product "RustDesk" in stores or on download pages · never commit
keys or passwords.
