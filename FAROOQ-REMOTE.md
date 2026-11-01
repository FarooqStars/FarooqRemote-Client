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
| `src/lang/ur.rs` (new), `src/lang.rs` | Urdu language added (764 strings). |
| `src/lang/en.rs` | `powered_by_me` → "Based on RustDesk · developed by Mohammad Farooq". |
| `flutter/lib/desktop/pages/desktop_setting_page.dart` | About card: developer line, website/privacy/help links, source URL. |
| `flutter/assets/fonts/*` (new), `flutter/pubspec.yaml`, `flutter/lib/common.dart` | 1.1.1: Naskh font "FS Urdu" (Noto Naskh Arabic 2.021, OFL) for Urdu / Arabic / Persian; `MyTheme.lightTheme` / `darkTheme` became getters; `Locale('ur')` in the supported locales. |
| `flutter/lib/common.dart`, `flutter/lib/desktop/pages/desktop_home_page.dart`, `flutter/lib/mobile/pages/settings_page.dart` | rustdesk.com links → farooqmughal.net (1.1.0; before: farooqmusic.com / mymandoob.com). |
| `flutter/windows/runner/Runner.rc` | Company / product / description / copyright strings. |
| `flutter/android/app/src/main/AndroidManifest.xml` | App label "Farooq Remote". |
| `res/*`, `flutter/assets/icon.svg`, `flutter/windows/runner/resources/app_icon.ico`, Android mipmaps, `flutter/macos/Runner/AppIcon.icns`, iOS AppIcon set | Farooq Remote icons. |
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

## Never

Never remove RustDesk licence or copyright notices · never use RustDesk Pro code ·
never call the product "RustDesk" in stores or on download pages · never commit
keys or passwords.
