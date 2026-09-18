# 0016 — PHASE 4 WINDOWS CLIENT: PASS (branded build v1.0.0-test3)

**FA-020 · 2026-09-18 · AI2-Claude2 · supersedes 0014 §5 "Phase 4 branding deferred" and 0015 §2 (browser-upload steps no longer apply)**

## Result
Branded **FarooqRemote** Windows client built by GitHub Actions, installed on HOME-02, connected to Office PC (28 954 244) through the Farooq Remote server 145.241.158.114. Home screen shows "Based on RustDesk · developed by Mohammad Farooq", status Ready, new icon, Urdu in language list. Owner confirmed 18 Sep 15:32 Doha: "it is working".

## Where everything is now
| Item | Value |
|---|---|
| GitHub organization | **FarooqStars** (GitHub Team, 2 seats: Farooqmusic + farooqmusicai, both Owners; $48/yr per seat) |
| Client repo | https://github.com/FarooqStars/FarooqRemote-Client — default branch **farooq/develop** (from upstream tag 1.4.9 / 6c578292); `master` = untouched upstream mirror |
| Server repo | https://github.com/FarooqStars/FarooqRemote-Server (moved from farooqmusicai; farooq/develop unchanged) |
| Local clones (HOME-02) | `C:\Users\Dell\OneDrive\Documents\GitHub\FarooqRemote-Client` and `...\FarooqRemote-Server` — GitHub Desktop, commits by Farooq; AI writes files into the clone via Cowork, Farooq commits + pushes. Remote still points at farooqmusicai (GitHub redirects); re-point to FarooqStars when convenient. |
| Build | Actions → "Farooq Remote - Windows x64 build" → Run workflow → branch farooq/develop, optional release-tag. ~37–47 min. Free minutes (public repo). |
| Releases | https://github.com/FarooqStars/FarooqRemote-Client/releases — v1.0.0-test2 (old exe name), **v1.0.0-test3** (good): `farooqremote-1.0.0-x86_64.exe` + SHA256SUMS.txt |
| Commits on farooq/develop | 9d4bc11 branding change set · 21388e7 branding on every start path (Android boot) · a3f06fc permissions + FarooqRemote.exe rename + Node-24 actions + direct LLVM install |

## Problems met and root-cause fixes
1. Workflow not visible → workflows list only from the default branch → default branch set to farooq/develop (needed org Owner; hence the move to FarooqStars).
2. Run #1 "Publish Release" 403 → token read-only → `permissions: contents: write` in workflow.
3. Run #2 installed but shortcut/service pointed to missing `FarooqRemote.exe` → app name is FarooqRemote but packaged exe was rustdesk.exe → workflow renames to `FarooqRemote.exe` before packing (upstream does the same for custom clients).
4. Node 20 deprecation warnings → action pins bumped to Node 24 releases; `KyleMayes/install-llvm-action` has no Node 24 release → replaced by direct LLVM 15.0.6 installer step.
5. Bot review of the mistaken upstream PR (#16268, closed) found Android boot-start bypassing branding → `farooq::apply()` now called from `global_init()`, `read_custom_client()` and the Android `startServer` JNI entry.

## Still open (Phase 4 remainder)
- 4B Android workflow (APK, own keystore — key kept by Farooq, never in Git). Then macOS/iOS after the Apple Developer account ($99/yr).
- 4C self-host server package for all major platforms + Urdu guide (FarooqRemote-Server).
- 4D download page on www.mymandoob.com/farooqremote + privacy page (URLs are already compiled into the client) + Microsoft Store MSIX (free signing).
- Owner: re-point the two local clones' remote to FarooqStars; remove PR #16268 branch noise if any; delete test releases when v1.0.0 final exists.
- Remaining CI warnings (libyuv / vcpkg_replace_string) come from RustDesk's vcpkg recipes; left as upstream.

## Never (added)
Never open a pull request from FarooqRemote-Client to rustdesk/rustdesk (GitHub Desktop's default PR target) · never commit the Android keystore or Apple certs · master stays the upstream mirror.
