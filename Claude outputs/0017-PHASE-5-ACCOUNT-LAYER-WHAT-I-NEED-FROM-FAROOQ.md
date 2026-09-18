# 0017 — PHASE 5: ACCOUNT LAYER — WHAT I NEED FROM FAROOQ (checklist)

**FA-020 · 2026-09-18 · AI2-Claude2 · owner prepares these, then says "Phase 5 go"**

Goal: one Farooq Remote account (e-mail+password, Google, Apple), login from any device
anywhere, all of a user's devices in one list with online/offline, address book synced,
all data stored only on Farooq's Oracle server. No new server, no paid service
(Apple sign-in is the only paid item — Apple's own $99/yr Developer Program).

Everything below is prepared by Farooq himself. **Never paste any secret, password,
key or client-secret into the chat.** Where a value is needed on the server, you will
type it into a `.env` file on the VM through an FR-*.bat I give you, or into the
GitHub Secrets page yourself. I only need to know *that* it exists.

---

## A. DNS (required first — nothing works without it)

| # | Do | Where | Done ✓ |
|---|---|---|---|
| A1 | Add DNS record **A** `remote` → `145.241.158.114`, TTL default | farooqmusic.com DNS (Hostinger / wherever the domain's DNS is) | ☐ |
| A2 | Optional: **A** `api-remote` → same IP (not needed; skip unless you want a separate name) | same | ☐ |
| A3 | Confirm: open https://dnschecker.org, type `remote.farooqmusic.com`, most locations show 145.241.158.114 | browser | ☐ |

Result: `remote.farooqmusic.com` becomes the ID server name, the API address and the web pages address.

## B. OCI firewall (Oracle console, read-only check + 2 rules)

| # | Do | Where | Done ✓ |
|---|---|---|---|
| B1 | Default Security List of **fm2-vcn** → check that **TCP 80** and **TCP 443** ingress from 0.0.0.0/0 already exist (Farooq Music 2.0 uses them). If yes, nothing to do. | OCI console → Networking → VCN → Security Lists | ☐ |
| B2 | Do **not** open 21114 (the API will be served on 443 under nginx). | — | ☐ |

## C. E-mail sender (for registration verification + password reset)

| # | Do | Done ✓ |
|---|---|---|
| C1 | Choose a mailbox that will send system mail, e.g. `noreply@farooqmusic.com` or an existing Hostinger mailbox. Create it if needed (Hostinger e-mail is included in your hosting — check, do not buy anything). | ☐ |
| C2 | Note for yourself (not for me): SMTP host, port (465 or 587), username, password. You will type these into the server `.env`. | ☐ |
| C3 | Tell me only: the **From address** to show (e.g. `noreply@farooqmusic.com`) and the **From name** (e.g. `Farooq Remote`). | ☐ |

## D. Google sign-in (free)

| # | Do | Where | Done ✓ |
|---|---|---|---|
| D1 | Google Cloud console → your existing project (or a new free project "Farooq Remote") → **APIs & Services → OAuth consent screen** → External → App name `Farooq Remote`, support e-mail, developer contact e-mail, app logo (use App-logo-icon), homepage `https://www.farooqmusic.com`, privacy `https://www.mymandoob.com/farooqremote/privacy`. Publish (or keep in Testing and add your own Gmail as test user for the first tests). | console.cloud.google.com | ☐ |
| D2 | **Credentials → Create credentials → OAuth client ID → Web application**. Name `Farooq Remote Web`. Authorized redirect URI: `https://remote.farooqmusic.com/api/oidc/callback` | same | ☐ |
| D3 | You get a **Client ID** and **Client secret**. Keep them; you will type them into the server `.env`. Tell me only "Google done". | — | ☐ |

Note: Google may charge nothing for this; the OAuth client is free. It does not need billing enabled.

## E. Apple sign-in (optional — needs Apple Developer Program, $99/yr)

Skip this section until you have the Apple Developer account (the same account needed later for the Mac and iPhone apps). Google + e-mail can go live without Apple.

| # | Do | Where | Done ✓ |
|---|---|---|---|
| E1 | Enrol in Apple Developer Program (individual, $99/yr). | developer.apple.com | ☐ |
| E2 | Certificates, Identifiers & Profiles → **Identifiers → App IDs** → new App ID `com.farooqmusic.farooqremote`, capability **Sign in with Apple**. | same | ☐ |
| E3 | **Identifiers → Services IDs** → new, identifier `com.farooqmusic.farooqremote.web`, enable Sign in with Apple, primary App ID = E2, domain `remote.farooqmusic.com`, return URL `https://remote.farooqmusic.com/api/oidc/callback`. | same | ☐ |
| E4 | **Keys** → new key, enable Sign in with Apple, download the `.p8` file **once** (Apple never shows it again) → keep on NAS beside the server key backups. Note Key ID and your Team ID. | same | ☐ |
| E5 | You will type Team ID, Key ID, Services ID and the .p8 contents into the server `.env`. Tell me only "Apple done". | — | ☐ |

## F. Server .env values (you type them; I only prepare the template)

I will give you `FR-05-ACCOUNTS.bat` that opens the file `/opt/farooqremote/api/.env` in an editor over SSH from HOME-01 (same pattern as FR-INSTALL). You fill these lines:

```
APP_URL=https://remote.farooqmusic.com
DB_NAME=farooqremote              (I create the database and a user; you set the password below)
DB_USER=farooqremote
DB_PASSWORD=                      ← you choose, long random
JWT_SECRET=                       ← you choose, long random (or I generate on the VM, never shown to me)
MAIL_HOST=  MAIL_PORT=  MAIL_USER=  MAIL_PASSWORD=  MAIL_FROM=  MAIL_FROM_NAME=
GOOGLE_CLIENT_ID=  GOOGLE_CLIENT_SECRET=
APPLE_TEAM_ID=  APPLE_KEY_ID=  APPLE_SERVICE_ID=  APPLE_PRIVATE_KEY=   (empty until section E)
```

## G. Decisions I need from you (answer in the chat, one line each)

| # | Question | Your answer |
|---|---|---|
| G1 | Registration allowed for everyone (open, like TeamViewer) — or invite-only at first? | |
| G2 | Must new accounts verify their e-mail before first login? (Yes = normal) | |
| G3 | Device limit per free account: unlimited, or a number (TeamViewer free = 3 managed devices)? Unlimited is the default. | |
| G4 | Username = the e-mail address, or a separate username chosen at registration? | |
| G5 | Language of the web pages: English + Urdu (both), or English + Urdu + Arabic? | |
| G6 | Where the "Register / Forgot password / My devices" web pages live: `https://remote.farooqmusic.com/` (default) or on mymandoob.com? | |
| G7 | Admin: you want an admin page (list users, disable a user, see device counts)? Yes/No | |
| G8 | Self-hosters: include the account API in the FarooqRemote-Server package (so they get accounts on their own server too)? Yes/No | |

## H. Approvals I need (the golden rule)

| # | Approval | Yes/No |
|---|---|---|
| H1 | One new nginx server block file `/etc/nginx/sites-available/remote.farooqmusic.com` on the Oracle VM + a free Let's Encrypt certificate (certbot, nginx plugin). FM2's own nginx files are not edited. Done via FR-*.bat, output saved to 0-docs. | |
| H2 | One new PostgreSQL database `farooqremote` + role on the existing PostgreSQL. FM2's database untouched. | |
| H3 | PHP-FPM pool for the API under the existing PHP 8 (same version FM2 uses). | |
| H4 | Client change: default API server `https://remote.farooqmusic.com` in `src/farooq.rs` + ID server changed from the IP to `remote.farooqmusic.com` (so the DNS name is used everywhere). New Windows build afterwards (~40 min Actions). | |
| H5 | Nightly backup of the `farooqremote` database into `/opt/farooqremote/backups` (added to the existing FR-03-backup.sh, not to the FM2 03:15 job). | |

## I. What I will deliver in Phase 5 (for the record)

1. `FarooqRemote-Server/api/` — PHP 8 implementation of the RustDesk client HTTP API: login, logout, currentUser, heartbeat, login-options, oidc/auth + auth-query + callback (Google, Apple), ab (address book get/set, tags), devices/peers, sysinfo, users (self), audit. Password hashing argon2id, JWT tokens, rate limiting.
2. Database schema + migration script (users, identities, email_verifications, password_resets, sessions, devices, address_book, tags, login_audit).
3. Web pages (English + Urdu): register, verify e-mail, login, forgot/reset password, my devices, account settings (change password, delete account), privacy page. Same teal/amber look as the app.
4. Optional admin page (G7).
5. FR-05-ACCOUNTS.bat (install API + DB + nginx block + certificate) and FR-06-ACCOUNTS-VERIFY.bat.
6. Client: `src/farooq.rs` API + hostname defaults; rebuild; the Login button, Google/Apple buttons, synced address book and device list work in the existing client UI.
7. Urdu user guide 0018 (register, login, add devices, use from anywhere) and English admin guide.
8. Server package update so self-hosters get the same (G8).

## J. Cost picture (once)

- My side: plan credits across several sessions (API, DB, pages, install scripts, guides, client rebuild, testing with you).
- GitHub Actions: free minutes (public repo) for the client rebuild.
- Money: $0 for DNS/certificate/database/Google. Apple sign-in: $99/yr Apple Developer Program (you decide when).

---

**When you come back:** tick A–D (E optional), answer G1–G8, give H1–H5 yes/no. Then I write the Phase 5 plan record (0018) and start with the database + API.
