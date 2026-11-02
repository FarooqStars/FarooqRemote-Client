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
| `src/farooq.rs` (`easy_post`, `ACCEPT_SERVER_STRATEGY`), `src/client.rs` (`fr_easy_pk`), `src/client/io_loop.rs`, `src/server/connection.rs`, `src/hbbs_http/sync.rs`, `src/hbbs_http/http_client.rs` + `src/hbbs_http.rs` (`create_http_client_async_no_redirect`), `src/lang/{en,ur,ar}.rs` | 1.1.6: easy access after a security review — strict https, the ticket tied to the encrypted session, wrong tickets counted, heartbeat «strategy» ignored (see «1.1.6» below). |
| `src/farooq.rs` (`easy_control_permissions`), `src/server/connection.rs` (`fr_apply_easy_permissions`, the ticket checked once before the session-type checks), `src/client.rs` (`fr_easy_sent`, easy access before a saved password), `src/client/io_loop.rs` (saved-password fallback, 15 s), `desktop_setting_page.dart` (`_frEasyPerms`), `src/lang/{en,ur,ar}.rs` | 1.1.7: what easy access may do (Full Access / Screen Share / Custom), easy access tried before a saved password (see «1.1.7» below). |
| `src/farooq.rs` (`FR_VERSION`, update check, `peer_version_note`, `WHY_*`, `rustdesk_installed`), `src/common.rs`, `src/ui_interface.rs`, `src/flutter_ffi.rs`, `src/platform/windows.rs`, `src/hbbs_http/{sync,http_client}.rs`, `src/server/connection.rs`, `src/client.rs`, `src/client/io_loop.rs`, `src/lang.rs`, `src/lang/{en,ur,ar}.rs`, `flutter/lib/common.dart`, `desktop_home_page.dart`, `desktop_setting_page.dart`, `common/widgets/dialog.dart`, `models/model.dart`, workflow step «one version number» | 1.1.8: one version number everywhere, the app looks for a newer version on the site and asks, clear messages when versions differ or easy access is not used (see «1.1.8» below). |
| `src/farooq.rs` (settings folders, `has_earlier_settings`, `delete_settings_cmds`, `ask_keep_settings_on_uninstall`, `resend_key_once_per_version`), `src/platform/windows.rs` (`install_me` option `fr-reset`, `uninstall_me`), `src/rendezvous_mediator.rs` (`start_all`), `src/flutter_ffi.rs`, `flutter/lib/desktop/pages/install_page.dart`, `src/lang/{en,ur,ar}.rs` | 1.1.9: «keep the settings?» when installing and uninstalling; the key goes to the ID server again after each new version (see «1.1.9» below). |
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
  the certificate always checked (rustls with the system's roots, then the system's own TLS), no detour, no redirect
  followed (`hbbs_http::create_http_client_async_no_redirect`, same TLS and proxy settings as upstream's client), 6 s,
  only a 2xx answer from that very address counts. The account token is never sent without a checked certificate.
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
* A second independent review of 1.1.6 + 2.7-10 found no way in for anyone without the owner's account. Known limits:
  the owner's account (its token on his PCs) now opens every easy-access computer, as with TeamViewer — 2FA in
  Settings → Security is still asked on this path if switched on; anyone who knows a computer's ID **and** its uuid can
  send a heartbeat that switches its announcement off (easy access then asks for the password; nobody gets in that way).
* Build note: `LoginRequest.fr_ticket` (field 99) is added to `libs/hbb_common/protos/message.proto` only by the step
  in `.github/workflows/farooqremote-windows.yml`; any other build (Linux, Android, a local `cargo check`) must run the
  same step first. Upstream's ci.yml / flutter-ci.yml run only for `master` and pull requests, so they are not affected.

## 1.1.7 — what easy access may do; easy access before a saved password (2 Nov 2026)

* The owner's test on 4 PCs (2 Nov 06:26): easy access still asked for a password, and Home 2 stayed on «connecting».
  Claude's own test from HOME-01 (computer use, his yes): Home 2, Office and Duty office open together in three tabs;
  Home 2 and Duty office (no saved password) came in by easy access. **The cause on Home 2: the official RustDesk 1.4.9
  from the September tests was still installed and running there** (service «RustDesk», `custom-rendezvous-server` =
  our ID server). Both programs register the same ID (the ID comes from the machine), so the ID server sometimes sends a
  connection to the old RustDesk: it ignores the ticket and waits for a click (the controller stays on «connecting») or
  asks its own password. Office and Duty office have only Farooq Remote. Fix: remove RustDesk from Home 2.
* **What easy access may do** (owner: «set easy access with menu to select which part should accessible for easy
  access» — his choice: the work permissions). Under the easy-access box: Full Access (default) / Screen Share / Custom
  with the same switches as the Permissions card (keyboard and mouse, remote printer, clipboard, file transfer, audio,
  camera, terminal, TCP tunneling, remote restart, recording, blocking input, privacy mode). Options `fr-easy-mode`
  (`""`/`full`, `view`, `custom`) and `fr-easy-perm-<name>` = `N`. They replace the general permissions for easy-access
  sessions only: the computer checks the ticket once at the start of the login (before the file-transfer / camera /
  terminal / tunnel checks) and, when it is accepted, gives the session `control_permissions` built by
  `crate::farooq::easy_control_permissions()` and tells the other side every permission that changes. Sessions with a
  password keep the general Permissions card.
* **Easy access before a saved password**: a signed-in app tries the ticket first even when a password is saved (an old
  saved password no longer blocks easy access); a password given on purpose for the connection (preset / shared) still
  goes first. If the other computer refuses the ticket, the saved password is sent at once
  (`client::handle_login_from_ui`), and only without one the password question appears. If no answer comes within
  15 s (an older program that waits for a click), the password question appears instead of «connecting» for ever.
* Clearer log lines: «no ticket … (not offered by the account server)», «tried before the saved password», and on the
  computer being reached which check refused a ticket (ticket, account, computer, key).
* Server: unchanged (accounts 2.7-10).

## 1.1.8 — one version number, update check, clear messages (2 Nov 2026)

* Owner, 2 Nov 16:26 (after 1.1.7 worked «after lot of effort»): «if your App build does not match of last version
  update it should give clear message and if it is old version it should detect online if new version available ask
  user to update and ver number in different in about in file name ?? onsite??» — About showed 1.4.9 (RustDesk's own
  number), the file and the site 1.1.7. His screenshots also showed «Insecure connection» and the password question when
  connecting to Home 2, where the old RustDesk is still installed (see 1.1.7).
* **One version number.** `crate::farooq::FR_VERSION` is the only place it is written. About shows it (and «Based on
  RustDesk 1.4.9» in the box), Windows «Apps» shows it (`DisplayVersion`/`Version`/`VersionMajor…` in `install_me` and
  `update_me`), the account server's device list gets «1.1.8 (1.4.9)», the other computer gets it in
  `PeerInfo.platform_additions` (`fr_version`). The workflow step «one version number» reads it from `src/farooq.rs`,
  names the file with it, sets the exe's file version (`flutter/pubspec.yaml` `version: <v>+67`) and stops when a
  release tag is not exactly `v<version>`. The RustDesk core keeps `crate::VERSION` = 1.4.9, because the protocol
  compares it with other computers (features are switched by it).
* **Update check.** The main window reads `https://www.farooqmughal.net/farooqremote/version.json` when it starts and
  every 12 hours (`start_update_check`, through upstream's `check_software_update`, which is off for custom clients):
  https only, certificate checked, redirects only within www.farooqmughal.net / farooqmughal.net (at most 3,
  `create_http_client_async_same_site`), 15 s, at most 4 KB, and only a version «a.b.c» is taken from it. A newer one
  shows a card on the home page («A new version is available», Download) and asks once per version («Later» /
  «Download»). «Download» always opens our own download page in the app's language (never an address from the
  network). The site generator writes `version.json` together with the download pages, so the app announces a version
  exactly when the site offers it. «Check for software update on startup» (Settings → General) is now shown and turns
  it off.
* **Different versions.** After the login, when the other computer runs another Farooq Remote version (or tells none:
  1.1.7 and older, or another program), a note says both numbers and which computer to update, with a button to the
  download page — once per pair of versions per computer (peer option `fr-version-noted`).
* **Why easy access was not used.** The password question now carries the reason (`crate::farooq::WHY_*`): this
  computer is not signed in / the other computer could not be verified (insecure connection) / the account server
  could not be reached / the other computer does not offer easy access (sign in there and tick it) / it refused the
  ticket / it did not answer in 15 s. The reasons found before a ticket are shown only for a computer that easy access
  has reached before (peer option `fr-easy-ok`, set at an easy-access login), so connections to other people's computers
  show the usual question; the reasons after a ticket are always shown.
* **Insecure connection**: our own text (`fr-conn-e2ee-unavailable-tip`) names the usual cause — another remote-desktop
  program with the same ID on that computer (for example an old RustDesk) — and what to do.
* **RustDesk installed here**: when RustDesk itself is installed on the computer (uninstall key or service «RustDesk»),
  the home page says that with our server both programs have the same ID and how to uninstall it (closable for good:
  local option `fr-show-rustdesk-card` = `N`).
* `src/lang.rs`: texts of our own keys (`fr-…`) keep the word RustDesk (upstream replaces it with the app name).
* Server: unchanged (accounts 2.7-10). Site: `farooqremote/version.json` + pages for 1.1.8.

## 1.1.9 — keep the settings or not; the key again after each version (2 Nov 2026)

* Owner, 2 Nov 20:41, after removing the old RustDesk from Home 2 («this is the main cause, now it is working»): «i need
  when install or uninstall App ask you do you want keep setting with password yes/or No», «i have 1 error red explain me
  what is this and how to remove it» (the Home 2 tab: red shield, «Direct and unencrypted connection (TCP)»), and a new
  build, then installing it together on his PC with pictures for a proper guide.
* **Where the settings are.** Upstream never removes them on uninstall, so a new installation brought them back: the
  service's config (ID, key, permanent password, options such as easy access) in
  `%SystemRoot%\ServiceProfiles\LocalService\AppData\Roaming\FarooqRemote\config`, and each user's config (account
  sign-in, saved passwords of other computers, language) in `%APPDATA%\FarooqRemote\config`; at installation the
  user's config is also imported into the service (`--import-config`).
* **Uninstall** (Windows «Apps» or the Start-menu shortcut, both `FarooqRemote.exe --uninstall`): a Windows question box
  in the app's language (RTL for Urdu/Arabic): Yes = keep, No = the uninstall script also deletes both folders (after the
  programs are stopped), Cancel = nothing happens. `crate::farooq::ask_keep_settings_on_uninstall`,
  `delete_settings_cmds` (only folders named `…\FarooqRemote\config`; paths with `"` or `%` are refused).
* **Install** (the install page, also used for upgrades): when settings are already there (`has_earlier_settings`: the
  app's uninstall key, the service folder — «yes» when it can not be looked into —, an account sign-in or saved
  computers), «Keep the settings?» Yes / No / Cancel. No adds `fr-reset` to the options: the script deletes both
  folders right after the old programs are stopped and does not import the old config; the new service starts like new
  (new key; the ID is normally the same, it comes from the machine).
* **The key again after each version** (`resend_key_once_per_version`, called at the start of
  `RendezvousMediator::start_all`): a computer sends its key to the ID server only the first time (`key_confirmed`). On
  Home 2 the old RustDesk with the same ID left its own key there, so every connection to Home 2 stayed unencrypted (red
  shield; «Insecure connection» when the controller checks). Now `key_confirmed` is cleared once per version
  (option `fr-key-sent-version`), and the ID server takes the key again (same uuid and address). Note: if the address
  had changed since the old key was stored, the ID server answers UUID_MISMATCH and upstream gives the computer a new ID
  (it is still bound to the account through the login history).
* The install page's licence link now opens our terms (`/farooqremote/terms/`) instead of rustdesk.com.

## Never

Never remove RustDesk licence or copyright notices · never use RustDesk Pro code ·
never call the product "RustDesk" in stores or on download pages · never commit
keys or passwords.
